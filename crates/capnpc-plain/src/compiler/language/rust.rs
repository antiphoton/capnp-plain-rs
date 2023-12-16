mod keyword;

use std::{collections::BTreeMap, fs::File, io::Write};

use anyhow::Result;
use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::{
    compiler::{context::CompilerContext, get_output_file_name, split_fields},
    schema::schema_capnp::{
        CodeGeneratorRequest, Field, Field_1, Field__Slot, Node_1, Node__Enum, Node__Struct, Type,
        Value,
    },
};

use self::keyword::is_keyword;

fn field_ident(s: &str) -> Ident {
    let s = s.to_case(Case::Snake);
    if is_keyword(&s) {
        format_ident!("r#{}", s)
    } else {
        format_ident!("{}", s)
    }
}

fn define_type(context: &CompilerContext, ty: &Type, is_box: bool) -> Option<TokenStream> {
    let r = match ty {
        Type::Void => quote!(()),
        Type::Bool => quote!(bool),
        Type::Int8 => quote!(i8),
        Type::Int16 => quote!(i16),
        Type::Int32 => quote!(i32),
        Type::Int64 => quote!(i64),
        Type::Uint8 => quote!(u8),
        Type::Uint16 => quote!(u16),
        Type::Uint32 => quote!(u32),
        Type::Uint64 => quote!(u64),
        Type::Float32 => quote!(f32),
        Type::Float64 => quote!(f64),
        Type::Text => quote!(String),
        Type::Data => quote!(Vec<u8>),
        Type::Struct(type_struct) => {
            let Some(node) = context.get_node(type_struct.type_id) else {
                return None;
            };
            let name = format_ident!("{}", context.get_full_name(node));
            if is_box {
                quote!(Option<Box<#name>>)
            } else {
                quote!(#name)
            }
        }
        Type::List(type_list) => {
            let Some(item) = type_list.element_type.as_deref() else {
                return None;
            };
            let item = define_type(context, item, false)?;
            quote!(Vec<#item>)
        }
        Type::Enum(type_enum) => {
            let Some(node) = context.get_node(type_enum.type_id) else {
                return None;
            };
            let name = format_ident!("{}", context.get_full_name(node));
            quote!(#name)
        }
        _ => return None,
    };
    Some(r)
}

fn read_list(_context: &CompilerContext, offset: u32, ty: &Type) -> Option<TokenStream> {
    let callback = match ty {
        Type::Bool => quote!(CapnpListNode::read_bool_children),
        Type::Int8 => quote!(CapnpListNode::read_i8_children),
        Type::Uint8 => quote!(CapnpListNode::read_u8_children),
        Type::Int16 => quote!(CapnpListNode::read_i16_children),
        Type::Uint16 => quote!(CapnpListNode::read_u16_children),
        Type::Int32 => quote!(CapnpListNode::read_i32_children),
        Type::Uint32 => quote!(CapnpListNode::read_u32_children),
        Type::Int64 => quote!(CapnpListNode::read_i64_children),
        Type::Uint64 => quote!(CapnpListNode::read_u64_children),
        Type::Float32 => quote!(CapnpListNode::read_f32_children),
        Type::Float64 => quote!(CapnpListNode::read_f64_children),
        Type::Struct(_) => {
            quote!(CapnpListNode::read_struct_children)
        }
        Type::Enum(_) => {
            quote!(CapnpListNode::read_enum_children)
        },
        _ => return None,
    };
    let reader = format_ident!("reader");
    let r = quote!(#reader.read_list(#offset, #callback));
    Some(r)
}

fn read_slot(context: &CompilerContext, slot: &Field__Slot, is_box: bool) -> Option<TokenStream> {
    let Some(ty) = slot.r#type.as_deref() else {
        return None;
    };
    define_type(context, ty, is_box)?;
    let reader = format_ident!("reader");
    let offset = slot.offset;
    let default_value = slot.default_value.as_deref().unwrap_or(&Value::Void);
    let r = match (ty, default_value) {
        (Type::Void, _) => quote!(()),
        (Type::Bool, Value::Bool(x)) => quote!(#reader.read_bool(#offset, #x)),
        (Type::Bool, _) => quote!(#reader.read_bool(#offset, false)),
        (Type::Int8, Value::Int8(x)) => quote!(#reader.read_i8(#offset, #x)),
        (Type::Int8, _) => quote!(#reader.read_i8(#offset, 0)),
        (Type::Int16, Value::Int16(x)) => quote!(#reader.read_i16(#offset, #x)),
        (Type::Int16, _) => quote!(#reader.read_i16(#offset, 0)),
        (Type::Int32, Value::Int32(x)) => quote!(#reader.read_i32(#offset, #x)),
        (Type::Int32, _) => quote!(#reader.read_i32(#offset, 0)),
        (Type::Int64, Value::Int64(x)) => quote!(#reader.read_i64(#offset, #x)),
        (Type::Int64, _) => quote!(#reader.read_i64(#offset, 0)),
        (Type::Uint8, Value::Uint8(x)) => quote!(#reader.read_u8(#offset, #x)),
        (Type::Uint8, _) => quote!(#reader.read_u8(#offset, 0)),
        (Type::Uint16, Value::Uint16(x)) => quote!(#reader.read_u16(#offset, #x)),
        (Type::Uint16, _) => quote!(#reader.read_u16(#offset, 0)),
        (Type::Uint32, Value::Uint32(x)) => quote!(#reader.read_u32(#offset, #x)),
        (Type::Uint32, _) => quote!(#reader.read_u32(#offset, 0)),
        (Type::Uint64, Value::Uint64(x)) => quote!(#reader.read_u64(#offset, #x)),
        (Type::Uint64, _) => quote!(#reader.read_u64(#offset, 0)),
        (Type::Float32, _) => quote!(#reader.read_f32(#offset, 0.0)),
        (Type::Float64, _) => quote!(#reader.read_f64(#offset, 0.0)),
        (Type::Data, _) => quote!(#reader.read_data(#offset)),
        (Type::Text, _) => {
            quote!(#reader.read_text(#offset))
        }
        (Type::Struct(type_struct), _) => {
            let Some(node) = context.get_node(type_struct.type_id) else {
                return None;
            };
            let name = format_ident!("{}", context.get_full_name(node));
            let r = quote!(#reader.read_struct(#offset));
            if is_box {
                quote!(#r.map(|x| Box::new(#name::from_node(x))))
            } else {
                quote!(#name::from_node(#r.unwrap()))
            }
        }
        (Type::List(type_list), _) => {
            let Some(ty) = type_list.element_type.as_deref() else {
                return None;
            };
            read_list(context, offset, ty).unwrap_or_else(|| quote!(vec![]))
        }
        (Type::Enum(type_enum), default_value) => {
            let default_value = match default_value {
                Value::Enum(x) => *x,
                _ => 0,
            };
            let Some(node) = context.get_node(type_enum.type_id) else {
                return None;
            };
            let name = format_ident!("{}", context.get_full_name(node));
            quote!(#name::decode(#reader.read_u16(#offset, #default_value)))
        }
        _ => return None,
    };
    Some(r)
}

fn write_list(offset: u32, ty: &Type, value: TokenStream) -> Option<TokenStream> {
    let value = match ty {
        Type::Struct(_) => {
            quote!(CapnpListNode::write_struct_children(&#value))
        }
        _ => return None,
    };
    let writer = format_ident!("writer");
    let r = quote!(#writer.write_child(#offset, CapnpNode::List(#value)););
    Some(r)
}

fn write_slot(slot: &Field__Slot, value: TokenStream, is_box: bool) -> Option<TokenStream> {
    let Some(ty) = slot.r#type.as_deref() else {
        return None;
    };
    let writer = format_ident!("writer");
    let offset = slot.offset;
    let default_value = slot.default_value.as_deref().unwrap_or(&Value::Void);
    let r = match (ty, default_value) {
        (Type::Bool, Value::Bool(x)) => quote!(#writer.write_bool(#offset, #value, #x);),
        (Type::Bool, _) => quote!(#writer.write_bool(#offset, #value, false);),
        (Type::Int8, Value::Int8(x)) => quote!(#writer.write_i8(#offset, #value, #x);),
        (Type::Int8, _) => quote!(#writer.write_i8(#offset, #value, 0);),
        (Type::Int16, Value::Int16(x)) => quote!(#writer.write_i16(#offset, #value, #x);),
        (Type::Int16, _) => quote!(#writer.write_i16(#offset, #value, 0);),
        (Type::Int32, Value::Int32(x)) => quote!(#writer.write_i32(#offset, #value, #x);),
        (Type::Int32, _) => quote!(#writer.write_i32(#offset, #value, 0);),
        (Type::Int64, Value::Int64(x)) => quote!(#writer.write_i64(#offset, #value, #x);),
        (Type::Int64, _) => quote!(#writer.write_i64(#offset, #value, 0);),
        (Type::Uint8, Value::Uint8(x)) => quote!(#writer.write_u8(#offset, #value, #x);),
        (Type::Uint8, _) => quote!(#writer.write_u8(#offset, #value, 0);),
        (Type::Uint16, Value::Uint16(x)) => quote!(#writer.write_u16(#offset, #value, #x);),
        (Type::Uint16, _) => quote!(#writer.write_u16(#offset, #value, 0);),
        (Type::Uint32, Value::Uint32(x)) => quote!(#writer.write_u32(#offset, #value, #x);),
        (Type::Uint32, _) => quote!(#writer.write_u32(#offset, #value, 0);),
        (Type::Uint64, Value::Uint64(x)) => quote!(#writer.write_u64(#offset, #value, #x);),
        (Type::Uint64, _) => quote!(#writer.write_u64(#offset, #value, 0);),
        (Type::Text, _) => {
            quote!(#writer.write_text(#offset, &#value);)
        }
        (Type::Struct(_type_struct), _) => {
            if is_box {
                quote! {
                    if let Some(child) = &#value {
                        #writer.write_child(#offset, CapnpNode::Struct(child.to_node()));
                    }
                }
            } else {
                quote! {}
            }
        }
        (Type::List(type_list), _) => {
            let Some(ty) = type_list.element_type.as_deref() else {
                return None;
            };
            write_list(offset, ty, value)?
        }
        (Type::Enum(_type_enum), default_value) => {
            let default_value = match default_value {
                Value::Enum(x) => *x,
                _ => 0,
            };
            quote!(#writer.write_u16(#offset, #value as u16, #default_value);)
        }
        _ => return None,
    };
    Some(r)
}

fn generate_common_struct(
    context: &CompilerContext,
    name: &Ident,
    fields: &[&Field],
) -> TokenStream {
    let definitions: Vec<_> = fields
        .iter()
        .filter_map(|field| {
            if field.0.discriminant_value != 0xffff {
                return None;
            }
            let name = field_ident(&field.0.name);
            match &field.1 {
                Field_1::Slot(slot) => {
                    let Some(ty) = slot.r#type.as_deref() else {
                        return None;
                    };
                    let ty = define_type(context, ty, true)?;
                    Some(quote! {
                        pub #name: #ty,
                    })
                }
                Field_1::Group(group) => {
                    let node = context.get_node(group.type_id)?;
                    let ty = format_ident!("{}", context.get_full_name(node));
                    Some(quote! {
                        pub #name: #ty,
                    })
                }
                _ => None,
            }
        })
        .collect();
    let parsers: Vec<_> = fields
        .iter()
        .filter_map(|field| {
            if field.0.discriminant_value != 0xffff {
                return None;
            }
            let name = field_ident(&field.0.name);
            match &field.1 {
                Field_1::Slot(slot) => {
                    let p = read_slot(context, slot, true)?;
                    Some(quote! {
                        #name: #p,
                    })
                }
                Field_1::Group(group) => {
                    let node = context.get_node(group.type_id)?;
                    let ty = context.get_full_name(node);
                    let ty = format_ident!("{}", ty);
                    Some(quote!(#name: #ty::from_node(reader),))
                }
                _ => None,
            }
        })
        .collect();
    let updaters: Vec<_> = fields
        .iter()
        .filter_map(|field| {
            if field.0.discriminant_value != 0xffff {
                return None;
            }
            let name = field_ident(&field.0.name);
            match &field.1 {
                Field_1::Slot(slot) => Some(write_slot(slot, quote!(self.#name), true)?),
                _ => None,
            }
        })
        .collect();

    let delcaration = quote! {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct #name {
            #(#definitions)*
        }
        impl CapnpPlainStruct for #name {
            fn from_node(reader: &CapnpStructNode) -> Self {
                #name {
                    #(#parsers)*
                }
            }
            fn update_node(&self, writer: &mut CapnpStructNode) {
                #(#updaters)*
            }
        }
    };
    delcaration
}

fn generate_variant_struct(
    context: &CompilerContext,
    name: &Ident,
    fields: &BTreeMap<u16, &Field>,
    discriminant_offset: u32,
) -> TokenStream {
    let definitions: Vec<_> = fields
        .iter()
        .filter_map(|(_, field)| {
            let field_name = format_ident!("{}", field.0.name.to_case(Case::UpperCamel));
            match &field.1 {
                Field_1::Slot(slot) => {
                    let Some(ty) = slot.r#type.as_deref() else {
                        return None;
                    };
                    if ty == &Type::Void {
                        Some(quote! {
                            #field_name,
                        })
                    } else {
                        let ty = define_type(context, ty, false);
                        Some(quote! { #field_name ( #ty ), })
                    }
                }
                Field_1::Group(group) => {
                    let node = context.get_node(group.type_id)?;
                    let ty = context.get_full_name(node);
                    let ty = format_ident!("{}", ty);
                    Some(quote!(#field_name (#ty),))
                }
                _ => None,
            }
        })
        .collect();
    let parsers: Vec<_> = fields
        .iter()
        .filter_map(|(i, field)| {
            let field_name = format_ident!("{}", field.0.name.to_case(Case::UpperCamel));
            match &field.1 {
                Field_1::Slot(slot) => {
                    let ty = slot.r#type.as_ref().unwrap();
                    if ty == &Box::new(Type::Void) {
                        return Some(quote! {
                            #i => Self::#field_name,
                        });
                    };
                    let p = read_slot(context, slot, false)?;
                    Some(quote! {
                        #i => Self::#field_name(#p),
                    })
                }
                Field_1::Group(group) => {
                    let node = context.get_node(group.type_id)?;
                    let ty = context.get_full_name(node);
                    let ty = format_ident!("{}", ty);
                    Some(quote! {
                        #i => Self::#field_name (#ty::from_node(reader)),
                    })
                }
                _ => None,
            }
        })
        .collect();
    let writers: Vec<_> = fields
        .iter()
        .filter_map(|(i, field)| {
            let field_name = format_ident!("{}", field.0.name.to_case(Case::UpperCamel));
            match &field.1 {
                Field_1::Slot(slot) => {
                    let ty = slot.r#type.as_ref().unwrap();
                    if ty == &Box::new(Type::Void) {
                        return Some(quote! {
                            Self::#field_name => #i,
                        });
                    };
                    if let Some(p) = write_slot(slot, quote!(*value), false) {
                        Some(quote! {
                            Self::#field_name(value) => {
                                #p
                                #i
                            }
                        })
                    } else {
                        Some(quote! {
                            Self::#field_name(..) => #i,
                        })
                    }
                }
                Field_1::Group(_group) => Some(quote! {
                    Self::#field_name(value) => {
                        value.update_node(writer);
                        #i
                    }
                }),
                _ => None,
            }
        })
        .collect();
    let name = format_ident!("{}", name);
    let declaration = quote! {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(tag = "t", content = "c")]
        pub enum #name {
            #(#definitions)*
            UnknownDiscriminant,
        }

        impl CapnpPlainStruct for #name {
            fn from_node(reader: &CapnpStructNode) -> Self {
                match reader.read_u16(#discriminant_offset, 0) {
                    #(#parsers)*
                    _ => Self::UnknownDiscriminant,
                }
            }
            fn update_node(&self, writer: &mut CapnpStructNode) {
                let discriminant_value = match self {
                    #(#writers)*
                    _ => {return;},
                };
                writer.write_u16(#discriminant_offset, discriminant_value, 0);
            }
        }
    };
    declaration
}

fn generate_node_struct(
    context: &CompilerContext,
    name: &str,
    node_struct: &Node__Struct,
) -> TokenStream {
    if name.is_empty() {
        return quote! {};
    }
    let total = format_ident!("{}", name);
    let (common_fields, variant_fields) = split_fields(&node_struct.fields);
    let discriminant_offset = node_struct.discriminant_offset;
    if variant_fields.is_empty() {
        generate_common_struct(context, &total, &common_fields)
    } else if common_fields.is_empty() {
        generate_variant_struct(context, &total, &variant_fields, discriminant_offset)
    } else {
        let common_name = format_ident!("{}_0", name);
        let common = generate_common_struct(context, &common_name, &common_fields);
        let variant_name = format_ident!("{}_1", name);
        let variant =
            generate_variant_struct(context, &variant_name, &variant_fields, discriminant_offset);
        quote! {
            #common
            #variant

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            pub struct #total(pub #common_name, pub #variant_name);

            impl CapnpPlainStruct for #total {
                fn from_node(reader: &CapnpStructNode) -> Self {
                    #total(
                        #common_name::from_node(reader),
                        #variant_name::from_node(reader),
                    )
                }
                fn update_node(&self, writer: &mut CapnpStructNode) {
                    self.0.update_node(writer);
                    self.1.update_node(writer);
                }
            }
        }
    }
}

fn generate_node_enum(name: &str, node_enum: &Node__Enum) -> TokenStream {
    let definitions: Vec<_> = node_enum
        .enumerants
        .iter()
        .map(|enumerant| {
            let name = format_ident!("{}", enumerant.name.to_case(Case::UpperCamel));
            let value = enumerant.code_order as isize;
            quote!(#name = #value,)
        })
        .collect();
    let name = format_ident!("{}", name);
    let max = node_enum.enumerants.len() as u16 - 1;
    quote!(
        #[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, Serialize, Deserialize)]
        pub enum #name {
            #(#definitions)*
            UnknownEnumerant,
        }

        impl CapnpPlainEnum for #name {
            fn decode(x: u16) -> Self {
                match x {
                    0..=#max => Self::from_u16(x).unwrap(),
                    _ => Self::UnknownEnumerant,
                }
            }
        }
    )
}

fn generate_code(code_generator_request: &CodeGeneratorRequest) -> TokenStream {
    let CodeGeneratorRequest { nodes, .. } = code_generator_request;
    let context = CompilerContext::new(code_generator_request);
    let mut output = vec![];
    for node in nodes {
        let name = context.get_full_name(node);
        let code = match &node.1 {
            Node_1::Struct(node_struct) => generate_node_struct(&context, &name, node_struct),
            Node_1::Enum(node_enum) => generate_node_enum(&name, node_enum),
            _ => quote!(),
        };
        output.push(code);
    }
    quote! {
        #(#output )*
    }
}

pub fn compile(code_generator_request: &CodeGeneratorRequest) -> Result<()> {
    let mut file = File::create(get_output_file_name(code_generator_request, "rs")?)?;
    let tokens = generate_code(code_generator_request);
    let tokens: TokenStream = tokens.into_iter().collect();
    let output = quote! {
        //! @generated
        #![allow(clippy::all)]
        #![allow(dead_code)]
        #![allow(non_camel_case_types)]
        #![allow(unused)]
        use anyhow::Result;
        use capnp_plain::{
            message::tree::{
                list_node::ListNode as CapnpListNode, struct_node::StructNode as CapnpStructNode,
                Node as CapnpNode,
            },
            CapnpPlainEnum, CapnpPlainStruct,
        };
        use num_derive::FromPrimitive;
        use num_traits::FromPrimitive;
        use serde::{Deserialize, Serialize};
        #tokens
    };
    let output = syn::parse2(output)?;
    let output = prettyplease::unparse(&output);
    file.write_all(output.as_bytes())?;
    Ok(())
}
