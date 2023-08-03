mod keyword;

use std::collections::BTreeMap;

use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::{
    compiler::context::CompilerContext,
    schema::{CodeGeneratorRequest, Field, Field_Union, Node_Union, Node__Struct, Type, Value},
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

impl Type {
    fn get_rust_primitive(&self) -> Option<TokenStream> {
        let t = match self {
            Self::Void => quote!(()),
            Self::Bool => quote!(bool),
            Self::Int8 => quote!(i8),
            Self::Int16 => quote!(i16),
            Self::Int32 => quote!(i32),
            Self::Int64 => quote!(i64),
            Self::Uint8 => quote!(u8),
            Self::Uint16 => quote!(u16),
            Self::Uint32 => quote!(u32),
            Self::Uint64 => quote!(u64),
            Self::Float32 => quote!(f32),
            Self::Float64 => quote!(f64),
            Self::Text => quote!(String),
            Self::Data => quote!(Vec<u8>),
            _ => return None,
        };
        Some(t)
    }
    fn get_rust_parser(&self, offset: u32, default_value: &Value) -> Option<TokenStream> {
        let reader = format_ident!("reader");
        let t = match (self, default_value) {
            (Self::Void, _) => quote!(()),
            (Self::Bool, _) => quote!(#reader.read_bool(#offset, false)),
            (Self::Int8, _) => quote!(#reader.read_i8(#offset, 0)),
            (Self::Int16, _) => quote!(#reader.read_i16(#offset, 0)),
            (Self::Int32, _) => quote!(#reader.read_i32(#offset, 0)),
            (Self::Int64, _) => quote!(#reader.read_i64(#offset, 0)),
            (Self::Uint8, _) => quote!(#reader.read_u8(#offset, 0)),
            (Self::Uint16, _) => quote!(#reader.read_u16(#offset, 0)),
            (Self::Uint32, _) => quote!(#reader.read_u32(#offset, 0)),
            (Self::Uint64, _) => quote!(#reader.read_u64(#offset, 0)),
            (Self::Text, _) => {
                quote!(#reader.read_pointer(#offset)?.into_list_reader()?.read_text()?)
            }
            _ => return None,
        };
        Some(t)
    }
}

fn generate_common_struct(name: &Ident, fields: &[&Field]) -> TokenStream {
    let definitions: Vec<_> = fields
        .iter()
        .filter_map(|field| {
            if field.0.discriminant_value != 0xffff {
                return None;
            }
            let name = field_ident(&field.0.name);
            let Some(Field_Union::Slot(slot) )= &field.1 else {
                return None
            };
            let ty = &slot.r#type.get_rust_primitive()?;
            Some(quote! {
                pub #name: #ty,
            })
        })
        .collect();
    let parsers: Vec<_> = fields
        .iter()
        .filter_map(|field| {
            if field.0.discriminant_value != 0xffff {
                return None;
            }
            let name = field_ident(&field.0.name);
            let Some(Field_Union::Slot(slot))= &field.1 else {
                return None
            };
            let p = &slot.r#type.get_rust_parser(slot.offset, &Value::Void)?;
            Some(quote! {
                #name: #p,
            })
        })
        .collect();
    let delcaration = quote! {
        #[derive(Debug, PartialEq)]
        pub struct #name {
            #(#definitions)*
        }
        impl CapnpPlainStruct for #name {
            fn try_from_reader(reader: StructReader) -> Result<Self> {
                let value = #name {
                    #(#parsers)*
                };
                Ok(value)
            }
        }
    };
    delcaration
}

fn generate_variant_struct(name: &Ident, fields: &BTreeMap<u16, &Field>) -> TokenStream {
    let definitions: Vec<_> = fields
        .iter()
        .filter_map(|(_, field)| {
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
                let ty = &slot.r#type.get_rust_primitive()?;
                Some(quote! { #name ( #ty ), })
            }
        })
        .collect();
    let arms: Vec<_> = fields
        .iter()
        .filter_map(|(i, field)| {
            let Some(Field_Union::Slot(slot) )= &field.1 else {
                return None
            };
            let name = format_ident!("{}", field.0.name.to_case(Case::UpperCamel));
            let ty = &slot.r#type;
            if ty == &Type::Void {
                return Some(quote! {
                    #i => Self::#name,
                });
            };
            let Some(p) = ty.get_rust_parser(slot.offset, &Value::Void) else {
                return None;
            };
            Some(quote! {
                #i => Self::#name(#p),
            })
        })
        .collect();
    let name = format_ident!("{}", name);
    let declaration = quote! {
        #[derive(Debug, PartialEq)]
        pub enum #name {
            #(#definitions)*
            UnknownDiscriminant,
        }

        impl CapnpPlainStruct for #name {
            fn try_from_reader(reader: StructReader) -> Result<Self> {
                let value = match reader.read_u16(0, 0) {
                    #(#arms)*
                    _ => Self::UnknownDiscriminant,
                };
                Ok(value)
            }
        }
    };
    declaration
}

fn generate_node_struct(name: &str, node_struct: &Node__Struct) -> TokenStream {
    let total = format_ident!("{}", name);
    let (common_fields, variant_fields) = node_struct
        .fields
        .iter()
        .partition::<Vec<_>, _>(|f| f.0.discriminant_value == 0xffff);
    let variant_fields: BTreeMap<_, _> = variant_fields
        .into_iter()
        .map(|f| (f.0.discriminant_value, f))
        .collect();
    if variant_fields.is_empty() {
        generate_common_struct(&total, &common_fields)
    } else if common_fields.is_empty() {
        generate_variant_struct(&total, &variant_fields)
    } else {
        let common_name = format_ident!("{}_0", name);
        let common = generate_common_struct(&common_name, &common_fields);
        let variant_name = format_ident!("{}_1", name);
        let variant = generate_variant_struct(&variant_name, &variant_fields);
        quote! {
            #common
            #variant

            #[derive(Debug, PartialEq)]
            pub struct #total(pub #common_name, pub #variant_name);

            impl CapnpPlainStruct for #total {
                fn try_from_reader(reader: StructReader) -> Result<Self> {
                    Ok(#total(
                        #common_name::try_from_reader(reader.clone())?,
                        #variant_name::try_from_reader(reader)?,
                    ))
                }
            }
        }
    }
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
