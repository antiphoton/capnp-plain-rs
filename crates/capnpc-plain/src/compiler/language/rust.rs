mod keyword;

use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::{
    compiler::context::CompilerContext,
    schema::{CodeGeneratorRequest, Field, Field_Union, Node_Union, Node__Struct, Type},
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

fn get_primitive(t: &Type) -> Option<TokenStream> {
    let t = match t {
        Type::Void => quote!(()),
        Type::Uint16 => quote! {u16},
        Type::Uint32 => quote! {u32},
        _ => return None,
    };
    Some(t)
}

fn generate_common_struct(name: Ident, fields: &[Field]) -> TokenStream {
    let fields: Vec<_> = fields
        .iter()
        .filter_map(|field| {
            if field.0.discriminant_value != 0xffff {
                return None;
            }
            let name = field_ident(&field.0.name);
            let Some(Field_Union::Slot(slot) )= &field.1 else {
                return None
            };
            let ty = get_primitive(&slot.r#type)?;
            Some(quote! {
                pub #name: #ty,
            })
        })
        .collect();
    let delcaration = quote! {
        #[derive(Debug, PartialEq, Eq)]
        pub struct #name {
            #(#fields )*
        }
    };
    delcaration
}

fn generate_union_struct(name: Ident, fields: &[Field], discriminant_count: u16) -> TokenStream {
    let variants: Vec<_> = (0..discriminant_count)
        .filter_map(|i| {
            let Some(field) = fields.iter().find(|x| x.0.discriminant_value == i) else {
                return None;
            };
            let Some(Field_Union::Slot(slot) )= &field.1 else {
                return None
            };
            let name = format_ident!("{}", field.0.name.to_case(Case::UpperCamel));
            let ty = &slot.r#type;
            if ty == &Type::Void {
                Some(quote! {
                    pub #name,
                })
            } else {
                let ty = get_primitive(&slot.r#type);
                Some(quote! { #name ( #ty ), })
            }
        })
        .collect();
    let name = format_ident!("{}", name);
    let declaration = quote! {
        #[derive(Debug, PartialEq, Eq)]
        pub enum #name {
            #(#variants )*
        }
    };
    declaration
}

fn generate_node_struct(name: &str, node_struct: &Node__Struct) -> TokenStream {
    if node_struct.discriminant_count == 0 {
        return generate_common_struct(format_ident!("{}", name), &node_struct.fields);
    }
    let common_name = format_ident!("{}_common", name);
    let common = generate_common_struct(common_name, &node_struct.fields);
    let variants_name = format_ident!("{}_variants", name);
    let variants = generate_union_struct(
        variants_name,
        &node_struct.fields,
        node_struct.discriminant_count,
    );
    quote! { #common #variants}
}

pub fn generate_code(code_generator_request: &CodeGeneratorRequest) -> TokenStream {
    let CodeGeneratorRequest { nodes, .. } = code_generator_request;
    let context = CompilerContext::new(code_generator_request);
    let mut output = vec![];
    for node in nodes {
        if let Some(Node_Union::Struct(node_struct)) = &node.1 {
            let struct_name = context.get_full_name(node);
            output.push(generate_node_struct(&struct_name, node_struct));
        }
    }
    quote! {
        #(#output )*
    }
}
