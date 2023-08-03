//! @generated
#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused)]
use anyhow::Result;
use capnp_plain::pointer::struct_pointer::{CapnpPlainStruct, StructReader};
#[derive(Debug, PartialEq)]
pub struct Field_0 {
    pub name: String,
    pub code_order: u16,
    pub discriminant_value: u16,
    pub ordinal: Field__ordinal,
}
impl CapnpPlainStruct for Field_0 {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Field_0 {
            name: reader.read_pointer(0u32)?.into_list_reader()?.read_text()?,
            code_order: reader.read_u16(0u32, 0),
            discriminant_value: reader.read_u16(1u32, 0),
            ordinal: Field__ordinal::try_from_reader(reader)?,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub enum Field_1 {
    Slot(Field__slot),
    Group(Field__group),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Field_1 {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0, 0) {
            0u16 => Self::Slot(Field__slot::try_from_reader(reader)?),
            1u16 => Self::Group(Field__group::try_from_reader(reader)?),
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Field(pub Field_0, pub Field_1);
impl CapnpPlainStruct for Field {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        Ok(
            Field(
                Field_0::try_from_reader(reader.clone())?,
                Field_1::try_from_reader(reader)?,
            ),
        )
    }
}
#[derive(Debug, PartialEq)]
pub struct Node_0 {
    pub id: u64,
    pub display_name: String,
    pub display_name_prefix_length: u32,
    pub scope_id: u64,
    pub is_generic: bool,
}
impl CapnpPlainStruct for Node_0 {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node_0 {
            id: reader.read_u64(0u32, 0),
            display_name: reader.read_pointer(0u32)?.into_list_reader()?.read_text()?,
            display_name_prefix_length: reader.read_u32(2u32, 0),
            scope_id: reader.read_u64(2u32, 0),
            is_generic: reader.read_bool(288u32, false),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub enum Node_1 {
    File,
    Struct(Node__struct),
    Enum(Node__enum),
    Interface(Node__interface),
    Const(Node__const),
    Annotation(Node__annotation),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Node_1 {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0, 0) {
            0u16 => Self::File,
            1u16 => Self::Struct(Node__struct::try_from_reader(reader)?),
            2u16 => Self::Enum(Node__enum::try_from_reader(reader)?),
            3u16 => Self::Interface(Node__interface::try_from_reader(reader)?),
            4u16 => Self::Const(Node__const::try_from_reader(reader)?),
            5u16 => Self::Annotation(Node__annotation::try_from_reader(reader)?),
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Node(pub Node_0, pub Node_1);
impl CapnpPlainStruct for Node {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        Ok(
            Node(
                Node_0::try_from_reader(reader.clone())?,
                Node_1::try_from_reader(reader)?,
            ),
        )
    }
}
#[derive(Debug, PartialEq)]
pub struct Node__struct {
    pub data_word_count: u16,
    pub pointer_count: u16,
    pub is_group: bool,
    pub discriminant_count: u16,
    pub discriminant_offset: u32,
}
impl CapnpPlainStruct for Node__struct {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__struct {
            data_word_count: reader.read_u16(7u32, 0),
            pointer_count: reader.read_u16(12u32, 0),
            is_group: reader.read_bool(224u32, false),
            discriminant_count: reader.read_u16(15u32, 0),
            discriminant_offset: reader.read_u32(8u32, 0),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Enumerant {
    pub name: String,
    pub code_order: u16,
}
impl CapnpPlainStruct for Enumerant {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Enumerant {
            name: reader.read_pointer(0u32)?.into_list_reader()?.read_text()?,
            code_order: reader.read_u16(0u32, 0),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Node__enum {}
impl CapnpPlainStruct for Node__enum {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__enum {};
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Method {
    pub name: String,
    pub code_order: u16,
    pub param_struct_type: u64,
    pub result_struct_type: u64,
    pub param_brand: Option<Box<Brand>>,
    pub result_brand: Option<Box<Brand>>,
}
impl CapnpPlainStruct for Method {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Method {
            name: reader.read_pointer(0u32)?.into_list_reader()?.read_text()?,
            code_order: reader.read_u16(0u32, 0),
            param_struct_type: reader.read_u64(1u32, 0),
            result_struct_type: reader.read_u64(2u32, 0),
            param_brand: Some(
                Box::new(
                    Brand::try_from_reader(
                        reader.read_pointer(2u32)?.into_struct_reader()?,
                    )?,
                ),
            ),
            result_brand: Some(
                Box::new(
                    Brand::try_from_reader(
                        reader.read_pointer(3u32)?.into_struct_reader()?,
                    )?,
                ),
            ),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Superclass {
    pub id: u64,
    pub brand: Option<Box<Brand>>,
}
impl CapnpPlainStruct for Superclass {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Superclass {
            id: reader.read_u64(0u32, 0),
            brand: Some(
                Box::new(
                    Brand::try_from_reader(
                        reader.read_pointer(0u32)?.into_struct_reader()?,
                    )?,
                ),
            ),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Node__interface {}
impl CapnpPlainStruct for Node__interface {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__interface {};
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub enum Type {
    Void,
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float32,
    Float64,
    Text,
    Data,
    List(Type__list),
    Enum(Type__enum),
    Struct(Type__struct),
    Interface(Type__interface),
    AnyPointer(Type__anyPointer),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Type {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0, 0) {
            0u16 => Self::Void,
            1u16 => Self::Bool,
            2u16 => Self::Int8,
            3u16 => Self::Int16,
            4u16 => Self::Int32,
            5u16 => Self::Int64,
            6u16 => Self::Uint8,
            7u16 => Self::Uint16,
            8u16 => Self::Uint32,
            9u16 => Self::Uint64,
            10u16 => Self::Float32,
            11u16 => Self::Float64,
            12u16 => Self::Text,
            13u16 => Self::Data,
            14u16 => Self::List(Type__list::try_from_reader(reader)?),
            15u16 => Self::Enum(Type__enum::try_from_reader(reader)?),
            16u16 => Self::Struct(Type__struct::try_from_reader(reader)?),
            17u16 => Self::Interface(Type__interface::try_from_reader(reader)?),
            18u16 => Self::AnyPointer(Type__anyPointer::try_from_reader(reader)?),
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub enum Value {
    Void,
    Bool(bool),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Float32(f32),
    Float64(f64),
    Text(String),
    Data(Vec<u8>),
    Enum(u16),
    Interface,
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Value {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0, 0) {
            0u16 => Self::Void,
            1u16 => Self::Bool(reader.read_bool(16u32, false)),
            2u16 => Self::Int8(reader.read_i8(2u32, 0)),
            3u16 => Self::Int16(reader.read_i16(1u32, 0)),
            4u16 => Self::Int32(reader.read_i32(1u32, 0)),
            5u16 => Self::Int64(reader.read_i64(1u32, 0)),
            6u16 => Self::Uint8(reader.read_u8(2u32, 0)),
            7u16 => Self::Uint16(reader.read_u16(1u32, 0)),
            8u16 => Self::Uint32(reader.read_u32(1u32, 0)),
            9u16 => Self::Uint64(reader.read_u64(1u32, 0)),
            12u16 => {
                Self::Text(reader.read_pointer(0u32)?.into_list_reader()?.read_text()?)
            }
            15u16 => Self::Enum(reader.read_u16(1u32, 0)),
            17u16 => Self::Interface,
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Node__const {
    pub r#type: Option<Box<Type>>,
    pub value: Option<Box<Value>>,
}
impl CapnpPlainStruct for Node__const {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__const {
            r#type: Some(
                Box::new(
                    Type::try_from_reader(
                        reader.read_pointer(3u32)?.into_struct_reader()?,
                    )?,
                ),
            ),
            value: Some(
                Box::new(
                    Value::try_from_reader(
                        reader.read_pointer(4u32)?.into_struct_reader()?,
                    )?,
                ),
            ),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Node__annotation {
    pub r#type: Option<Box<Type>>,
    pub targets_file: bool,
    pub targets_const: bool,
    pub targets_enum: bool,
    pub targets_enumerant: bool,
    pub targets_struct: bool,
    pub targets_field: bool,
    pub targets_union: bool,
    pub targets_group: bool,
    pub targets_interface: bool,
    pub targets_method: bool,
    pub targets_param: bool,
    pub targets_annotation: bool,
}
impl CapnpPlainStruct for Node__annotation {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__annotation {
            r#type: Some(
                Box::new(
                    Type::try_from_reader(
                        reader.read_pointer(3u32)?.into_struct_reader()?,
                    )?,
                ),
            ),
            targets_file: reader.read_bool(112u32, false),
            targets_const: reader.read_bool(113u32, false),
            targets_enum: reader.read_bool(114u32, false),
            targets_enumerant: reader.read_bool(115u32, false),
            targets_struct: reader.read_bool(116u32, false),
            targets_field: reader.read_bool(117u32, false),
            targets_union: reader.read_bool(118u32, false),
            targets_group: reader.read_bool(119u32, false),
            targets_interface: reader.read_bool(120u32, false),
            targets_method: reader.read_bool(121u32, false),
            targets_param: reader.read_bool(122u32, false),
            targets_annotation: reader.read_bool(123u32, false),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Node__NestedNode {
    pub name: String,
    pub id: u64,
}
impl CapnpPlainStruct for Node__NestedNode {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__NestedNode {
            name: reader.read_pointer(0u32)?.into_list_reader()?.read_text()?,
            id: reader.read_u64(0u32, 0),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Annotation {
    pub id: u64,
    pub value: Option<Box<Value>>,
    pub brand: Option<Box<Brand>>,
}
impl CapnpPlainStruct for Annotation {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Annotation {
            id: reader.read_u64(0u32, 0),
            value: Some(
                Box::new(
                    Value::try_from_reader(
                        reader.read_pointer(0u32)?.into_struct_reader()?,
                    )?,
                ),
            ),
            brand: Some(
                Box::new(
                    Brand::try_from_reader(
                        reader.read_pointer(1u32)?.into_struct_reader()?,
                    )?,
                ),
            ),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Node__Parameter {
    pub name: String,
}
impl CapnpPlainStruct for Node__Parameter {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__Parameter {
            name: reader.read_pointer(0u32)?.into_list_reader()?.read_text()?,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Brand {}
impl CapnpPlainStruct for Brand {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Brand {};
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Brand__Scope_0 {
    pub scope_id: u64,
}
impl CapnpPlainStruct for Brand__Scope_0 {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Brand__Scope_0 {
            scope_id: reader.read_u64(0u32, 0),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub enum Brand__Scope_1 {
    Inherit,
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Brand__Scope_1 {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0, 0) {
            1u16 => Self::Inherit,
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Brand__Scope(pub Brand__Scope_0, pub Brand__Scope_1);
impl CapnpPlainStruct for Brand__Scope {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        Ok(
            Brand__Scope(
                Brand__Scope_0::try_from_reader(reader.clone())?,
                Brand__Scope_1::try_from_reader(reader)?,
            ),
        )
    }
}
#[derive(Debug, PartialEq)]
pub enum Brand__Binding {
    Unbound,
    Type(Type),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Brand__Binding {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0, 0) {
            0u16 => Self::Unbound,
            1u16 => {
                Self::Type(
                    Type::try_from_reader(
                        reader.read_pointer(0u32)?.into_struct_reader()?,
                    )?,
                )
            }
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Type__list {
    pub element_type: Option<Box<Type>>,
}
impl CapnpPlainStruct for Type__list {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Type__list {
            element_type: Some(
                Box::new(
                    Type::try_from_reader(
                        reader.read_pointer(0u32)?.into_struct_reader()?,
                    )?,
                ),
            ),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Type__enum {
    pub type_id: u64,
    pub brand: Option<Box<Brand>>,
}
impl CapnpPlainStruct for Type__enum {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Type__enum {
            type_id: reader.read_u64(1u32, 0),
            brand: Some(
                Box::new(
                    Brand::try_from_reader(
                        reader.read_pointer(0u32)?.into_struct_reader()?,
                    )?,
                ),
            ),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Type__struct {
    pub type_id: u64,
    pub brand: Option<Box<Brand>>,
}
impl CapnpPlainStruct for Type__struct {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Type__struct {
            type_id: reader.read_u64(1u32, 0),
            brand: Some(
                Box::new(
                    Brand::try_from_reader(
                        reader.read_pointer(0u32)?.into_struct_reader()?,
                    )?,
                ),
            ),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Type__interface {
    pub type_id: u64,
    pub brand: Option<Box<Brand>>,
}
impl CapnpPlainStruct for Type__interface {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Type__interface {
            type_id: reader.read_u64(1u32, 0),
            brand: Some(
                Box::new(
                    Brand::try_from_reader(
                        reader.read_pointer(0u32)?.into_struct_reader()?,
                    )?,
                ),
            ),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub enum Type__anyPointer__unconstrained {
    AnyKind,
    Struct,
    List,
    Capability,
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Type__anyPointer__unconstrained {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0, 0) {
            0u16 => Self::AnyKind,
            1u16 => Self::Struct,
            2u16 => Self::List,
            3u16 => Self::Capability,
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Type__anyPointer__parameter {
    pub scope_id: u64,
    pub parameter_index: u16,
}
impl CapnpPlainStruct for Type__anyPointer__parameter {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Type__anyPointer__parameter {
            scope_id: reader.read_u64(2u32, 0),
            parameter_index: reader.read_u16(5u32, 0),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Type__anyPointer__implicitMethodParameter {
    pub parameter_index: u16,
}
impl CapnpPlainStruct for Type__anyPointer__implicitMethodParameter {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Type__anyPointer__implicitMethodParameter {
            parameter_index: reader.read_u16(5u32, 0),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub enum Type__anyPointer {
    Unconstrained(Type__anyPointer__unconstrained),
    Parameter(Type__anyPointer__parameter),
    ImplicitMethodParameter(Type__anyPointer__implicitMethodParameter),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Type__anyPointer {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0, 0) {
            0u16 => {
                Self::Unconstrained(
                    Type__anyPointer__unconstrained::try_from_reader(reader)?,
                )
            }
            1u16 => {
                Self::Parameter(Type__anyPointer__parameter::try_from_reader(reader)?)
            }
            2u16 => {
                Self::ImplicitMethodParameter(
                    Type__anyPointer__implicitMethodParameter::try_from_reader(reader)?,
                )
            }
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Field__slot {
    pub offset: u32,
    pub r#type: Option<Box<Type>>,
    pub default_value: Option<Box<Value>>,
    pub had_explicit_default: bool,
}
impl CapnpPlainStruct for Field__slot {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Field__slot {
            offset: reader.read_u32(1u32, 0),
            r#type: Some(
                Box::new(
                    Type::try_from_reader(
                        reader.read_pointer(2u32)?.into_struct_reader()?,
                    )?,
                ),
            ),
            default_value: Some(
                Box::new(
                    Value::try_from_reader(
                        reader.read_pointer(3u32)?.into_struct_reader()?,
                    )?,
                ),
            ),
            had_explicit_default: reader.read_bool(128u32, false),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Field__group {
    pub type_id: u64,
}
impl CapnpPlainStruct for Field__group {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Field__group {
            type_id: reader.read_u64(2u32, 0),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub enum Field__ordinal {
    Implicit,
    Explicit(u16),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Field__ordinal {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0, 0) {
            0u16 => Self::Implicit,
            1u16 => Self::Explicit(reader.read_u16(6u32, 0)),
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Node__SourceInfo__Member {
    pub doc_comment: String,
}
impl CapnpPlainStruct for Node__SourceInfo__Member {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__SourceInfo__Member {
            doc_comment: reader.read_pointer(0u32)?.into_list_reader()?.read_text()?,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Node__SourceInfo {
    pub id: u64,
    pub doc_comment: String,
}
impl CapnpPlainStruct for Node__SourceInfo {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__SourceInfo {
            id: reader.read_u64(0u32, 0),
            doc_comment: reader.read_pointer(0u32)?.into_list_reader()?.read_text()?,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct CapnpVersion {
    pub major: u16,
    pub minor: u8,
    pub micro: u8,
}
impl CapnpPlainStruct for CapnpVersion {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = CapnpVersion {
            major: reader.read_u16(0u32, 0),
            minor: reader.read_u8(2u32, 0),
            micro: reader.read_u8(3u32, 0),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct CodeGeneratorRequest__RequestedFile {
    pub id: u64,
    pub filename: String,
}
impl CapnpPlainStruct for CodeGeneratorRequest__RequestedFile {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = CodeGeneratorRequest__RequestedFile {
            id: reader.read_u64(0u32, 0),
            filename: reader.read_pointer(0u32)?.into_list_reader()?.read_text()?,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct CodeGeneratorRequest {
    pub capnp_version: Option<Box<CapnpVersion>>,
}
impl CapnpPlainStruct for CodeGeneratorRequest {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = CodeGeneratorRequest {
            capnp_version: Some(
                Box::new(
                    CapnpVersion::try_from_reader(
                        reader.read_pointer(2u32)?.into_struct_reader()?,
                    )?,
                ),
            ),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct CodeGeneratorRequest__RequestedFile__Import {
    pub id: u64,
    pub name: String,
}
impl CapnpPlainStruct for CodeGeneratorRequest__RequestedFile__Import {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = CodeGeneratorRequest__RequestedFile__Import {
            id: reader.read_u64(0u32, 0),
            name: reader.read_pointer(0u32)?.into_list_reader()?.read_text()?,
        };
        Ok(value)
    }
}
