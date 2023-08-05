use std::{collections::BTreeMap, fs::File, io::Write};

use anyhow::Result;
use convert_case::{Case, Casing};

use crate::{
    compiler::{context::CompilerContext, get_output_file_name, split_fields},
    schema::schema_capnp::{
        CodeGeneratorRequest, Field, Field_1, Node_1, Node__Enum, Node__Struct, Type,
    },
};

fn define_type(context: &CompilerContext, ty: &Type) -> Option<String> {
    let r = match ty {
        Type::Void => "null".to_string(),
        Type::Bool => "boolean".to_string(),
        Type::Int8
        | Type::Int16
        | Type::Int32
        | Type::Int64
        | Type::Uint8
        | Type::Uint16
        | Type::Uint32
        | Type::Uint64
        | Type::Float32
        | Type::Float64 => "number".to_string(),
        Type::Text => "string".to_string(),
        Type::Data => "Array<number>".to_string(),
        Type::Struct(type_struct) => {
            let Some(node) = context.get_node(type_struct.type_id) else {
                return None;
            };
            context.get_full_name(node)
        }
        Type::List(type_list) => {
            let Some(item) = type_list.element_type.as_deref() else {
                return None;
            };
            let item = define_type(context, item)?;
            format!("Array<{}>", item)
        }
        Type::Enum(type_enum) => {
            let Some(node) = context.get_node(type_enum.type_id) else {
                return None;
            };
            context.get_full_name(node)
        }
        _ => return None,
    };
    Some(r)
}

fn generate_common_struct(context: &CompilerContext, name: &str, fields: &[&Field]) -> String {
    let definitions: Vec<_> = fields
        .iter()
        .filter_map(|field| {
            if field.0.discriminant_value != 0xffff {
                return None;
            }
            let field_name = field.0.name.to_case(Case::Snake);
            match &field.1 {
                Field_1::Slot(slot) => {
                    let Some(ty) = slot.r#type.as_deref() else {
                        return None;
                    };
                    let ty = define_type(context, ty)?;
                    Some(format!("{}: {}", field_name, ty))
                }
                Field_1::Group(group) => {
                    let node = context.get_node(group.type_id)?;
                    let ty = context.get_full_name(node);
                    Some(format!("{}: {}", field_name, ty))
                }
                _ => None,
            }
        })
        .map(|s| format!("  {}", s))
        .collect();
    format!(
        "export interface {} {{\n{};\n}};",
        name,
        definitions.join(";\n")
    )
}

fn generate_variant_struct(
    context: &CompilerContext,
    name: &str,
    fields: &BTreeMap<u16, &Field>,
) -> String {
    let definitions: Vec<_> = fields
        .iter()
        .filter_map(|(_, field)| {
            let field_name = field.0.name.to_case(Case::UpperCamel);
            match &field.1 {
                Field_1::Slot(slot) => {
                    let Some(ty) = slot.r#type.as_deref() else {
                        return None;
                    };
                    if ty == &Type::Void {
                        Some(format!(r#"{{ "t": "{}" }}"#, field_name))
                    } else {
                        let ty = define_type(context, ty)?;
                        Some(format!(r#"{{ "t": "{}", "c": {} }}"#, field_name, ty))
                    }
                }
                Field_1::Group(group) => {
                    let node = context.get_node(group.type_id)?;
                    let ty = context.get_full_name(node);
                    Some(format!(r#"{{ "t": "{}", "c": {} }}"#, field_name, ty))
                }
                _ => None,
            }
        })
        .map(|s| format!("  {}", s))
        .collect();
    format!(
        "export type {} = {{ \"t\": \"UnknownDiscriminant\" }} |\n{};\n",
        name,
        definitions.join(" |\n")
    )
}

fn generate_node_struct(
    context: &CompilerContext,
    name: &str,
    node_struct: &Node__Struct,
) -> String {
    let (common_fields, variant_fields) = split_fields(&node_struct.fields);
    if variant_fields.is_empty() {
        generate_common_struct(context, name, &common_fields)
    } else if common_fields.is_empty() {
        generate_variant_struct(context, name, &variant_fields)
    } else {
        let common_name = format!("{}_0", name);
        let common = generate_common_struct(context, &common_name, &common_fields);
        let variant_name = format!("{}_1", name);
        let variant = generate_variant_struct(context, &variant_name, &variant_fields);
        format!(
            "{}\n{}\nexport type {} = [{}, {}];\n",
            common, variant, name, common_name, variant_name
        )
    }
}

fn generate_node_enum(name: &str, node_enum: &Node__Enum) -> String {
    let definitions: Vec<_> = node_enum
        .enumerants
        .iter()
        .map(|enumerant| format!(r#""{}""#, enumerant.name.to_case(Case::UpperCamel)))
        .map(|s| format!("  {}", s))
        .collect();
    format!(
        "export type {} = \"UnknownEnumerant\" |\n{};\n",
        name,
        definitions.join(" |\n")
    )
}

fn generate_code(code_generator_request: &CodeGeneratorRequest) -> String {
    let CodeGeneratorRequest { nodes, .. } = code_generator_request;
    let context = CompilerContext::new(code_generator_request);
    let mut output = vec![];
    for node in nodes {
        let name = context.get_full_name(node);
        let code = match &node.1 {
            Node_1::Struct(node_struct) => generate_node_struct(&context, &name, node_struct),
            Node_1::Enum(node_enum) => generate_node_enum(&name, node_enum),
            _ => "".to_string(),
        };
        output.push(code);
    }
    output.join("\n")
}

pub fn compile(code_generator_request: &CodeGeneratorRequest) -> Result<()> {
    let mut file = File::create(get_output_file_name(code_generator_request, "ts")?)?;
    let code = generate_code(code_generator_request);
    file.write_all(code.as_bytes())?;
    Ok(())
}
