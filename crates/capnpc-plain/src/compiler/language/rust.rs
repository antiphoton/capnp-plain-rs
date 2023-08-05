mod keyword;

use std::collections::BTreeMap;

use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::{
    compiler::context::CompilerContext,
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
        Type::Bool => quote!(|r| r.read_bool_children()),
        Type::Int8 => quote!(|r| r.read_i8_children()),
        Type::Uint8 => quote!(|r| r.read_u8_children()),
        Type::Struct(_) => {
            quote!(|r| r.read_struct_children())
        }
        _ => return None,
    };
    let reader = format_ident!("reader");
    let r = quote!(#reader.read_list_field(#offset, #callback));
    Some(r)
}

fn read_slot(context: &CompilerContext, slot: &Field__Slot, is_box: bool) -> Option<TokenStream> {
    let Some(ty) = slot.r#type.as_deref() else {
        return None;
    };
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
        (Type::Text, _) => {
            quote!(#reader.read_text_field(#offset))
        }
        (Type::Struct(type_struct), _) => {
            let Some(node) = context.get_node(type_struct.type_id) else {
                return None;
            };
            let name = format_ident!("{}", context.get_full_name(node));
            let r = quote!(#reader.read_struct_child::<#name>(#offset));
            if is_box {
                quote!(#r.ok().map(Box::new))
            } else {
                quote!(#r?)
            }
        }
        (Type::List(type_list), _) => {
            let Some(ty) = type_list.element_type.as_deref() else {
                return None;
            };
            read_list(context, offset, ty)?
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
                        return None
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
                    Some(quote!(#name: #ty::try_from_reader(reader)?))
                }
                _ => None,
            }
        })
        .collect();
    let delcaration = quote! {
        #[derive(Debug, Clone, PartialEq)]
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
    discriminant_offset: u32,
) -> TokenStream {
    let definitions: Vec<_> = fields
        .iter()
        .filter_map(|(_, field)| {
            let field_name = format_ident!("{}", field.0.name.to_case(Case::UpperCamel));
            match &field.1 {
                Field_1::Slot(slot) => {
                    let Some(ty) = slot.r#type.as_deref() else {
                        return None
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
    let arms: Vec<_> = fields
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
                    Some(quote!( #i => Self::#field_name (#ty::try_from_reader(reader)?),))
                }
                _ => None,
            }
        })
        .collect();
    let name = format_ident!("{}", name);
    let declaration = quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub enum #name {
            #(#definitions)*
            UnknownDiscriminant,
        }

        impl CapnpPlainStruct for #name {
            fn try_from_reader(reader: StructReader) -> Result<Self> {
                let value = match reader.read_u16(#discriminant_offset, 0) {
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

            #[derive(Debug, Clone, PartialEq)]
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
        #[derive(Debug, Clone, PartialEq, FromPrimitive)]
        pub enum #name {
            #(#definitions)*
            UnknownEnumerant,
        }

        impl #name {
            pub fn decode(x: u16) -> Self {
                match x {
                    0..=#max => Self::from_u16(x).unwrap(),
                    _ => Self::UnknownEnumerant,
                }
            }
        }
    )
}

pub fn generate_code(code_generator_request: &CodeGeneratorRequest) -> TokenStream {
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
