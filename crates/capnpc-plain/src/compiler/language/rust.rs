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
    fn rust_boxed(&self) -> bool {
        matches!(self, Self::Struct(_))
    }
    fn rust_declaration(&self, context: &CompilerContext) -> Option<TokenStream> {
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
            Self::Struct(type_struct) => {
                let Some(node) = context.get_node(type_struct.type_id) else {
                    return None;
                };
                let name = context.get_full_name(node);
                let name = format_ident!("{}", name);
                quote!(#name)
            }
            _ => return None,
        };
        Some(t)
    }
    fn rust_parser(
        &self,
        context: &CompilerContext,
        offset: u32,
        default_value: &Value,
    ) -> Option<TokenStream> {
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
            (Self::Struct(type_struct), _) => {
                let Some(node) = context.get_node(type_struct.type_id) else {
                    return None;
                };
                let name = context.get_full_name(node);
                let name = format_ident!("{}", name);
                quote!(#reader.read_struct_child::<#name>(#offset))
            }
            _ => return None,
        };
        Some(t)
    }
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
                Some(Field_Union::Slot(slot)) => {
                    let ty = &slot.r#type.rust_declaration(context)?;
                    if slot.r#type.rust_boxed() {
                        Some(quote! {
                                pub #name: Option<Box<#ty>>,
                        })
                    } else {
                        Some(quote! {
                        pub #name: #ty,
                        })
                    }
                }
                Some(Field_Union::Group(group)) => {
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
                Some(Field_Union::Slot(slot)) => {
                    let p = &slot
                        .r#type
                        .rust_parser(context, slot.offset, &Value::Void)?;
                    if slot.r#type.rust_boxed() {
                        Some(quote! {
                            #name: #p.ok().map(Box::new),
                        })
                    } else {
                        Some(quote! {
                            #name: #p,
                        })
                    }
                }
                Some(Field_Union::Group(group)) => {
                    let node = context.get_node(group.type_id)?;
                    let ty = context.get_full_name(node);
                    let ty = format_ident!("{}", ty);
                    Some(quote!(#name: #ty::try_from_reader(reader)?))
                }
                _ => None,
            }
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

fn generate_variant_struct(
    context: &CompilerContext,
    name: &Ident,
    fields: &BTreeMap<u16, &Field>,
) -> TokenStream {
    let definitions: Vec<_> = fields
        .iter()
        .filter_map(|(_, field)| {
            let field_name = format_ident!("{}", field.0.name.to_case(Case::UpperCamel));
            match &field.1 {
                Some(Field_Union::Slot(slot)) => {
                    let ty = &slot.r#type;
                    if ty == &Type::Void {
                        Some(quote! {
                            pub #field_name,
                        })
                    } else {
                        let ty = &slot.r#type.rust_declaration(context)?;
                        Some(quote! { #field_name ( #ty ), })
                    }
                }
                Some(Field_Union::Group(group)) => {
                    let node = context.get_node(group.type_id)?;
                    let ty = context.get_full_name(node);
                    let ty = format_ident!("{}", ty);
                    Some(quote!(#field_name (#ty),))
                }
                _ => None,
            }
        })
        .collect();
    let arms: Vec<_> = fields
        .iter()
        .filter_map(|(i, field)| {
            let field_name = format_ident!("{}", field.0.name.to_case(Case::UpperCamel));
            match &field.1 {
                Some(Field_Union::Slot(slot)) => {
                    let ty = &slot.r#type;
                    if ty == &Type::Void {
                        return Some(quote! {
                            #i => Self::#field_name,
                        });
                    };
                    let Some(p) = ty.rust_parser(context, slot.offset, &Value::Void) else {
                        return None;
                    };
                    if ty.rust_boxed() {
                        Some(quote! {
                            #i => Self::#field_name(#p?),
                        })
                    } else {
                        Some(quote! {
                            #i => Self::#field_name(#p),
                        })
                    }
                }
                Some(Field_Union::Group(group)) => {
                    let node = context.get_node(group.type_id)?;
                    let ty = context.get_full_name(node);
                    let ty = format_ident!("{}", ty);
                    Some(quote!( #i => Self::#field_name (#ty::try_from_reader(reader)?),))
                }
                _ => None,
            }
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

fn generate_node_struct(
    context: &CompilerContext,
    name: &str,
    node_struct: &Node__Struct,
) -> TokenStream {
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
        generate_common_struct(context, &total, &common_fields)
    } else if common_fields.is_empty() {
        generate_variant_struct(context, &total, &variant_fields)
    } else {
        let common_name = format_ident!("{}_0", name);
        let common = generate_common_struct(context, &common_name, &common_fields);
        let variant_name = format_ident!("{}_1", name);
        let variant = generate_variant_struct(context, &variant_name, &variant_fields);
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
            output.push(generate_node_struct(&context, &struct_name, node_struct));
        }
    }
    quote! {
        #(#output )*
    }
}
