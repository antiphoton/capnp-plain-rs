//! @generated
#![allow(dead_code)]
#![allow(non_camel_case_types)]
use anyhow::Result;
use capnp_plain::pointer::struct_pointer::{CapnpPlainStruct, StructReader};
#[derive(Debug, PartialEq)]
pub struct Field_0 {
    pub code_order: u16,
    pub discriminant_value: u16,
}
#[derive(Debug, PartialEq)]
pub enum Field_1 {
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Field_1 {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0, 0) {
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Field(pub Field_0, pub Field_1);
#[derive(Debug, PartialEq)]
pub struct Node_0 {
    pub display_name_prefix_length: u32,
}
#[derive(Debug, PartialEq)]
pub enum Node_1 {
    File,
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Node_1 {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0, 0) {
            0u16 => Self::File,
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Node(pub Node_0, pub Node_1);
#[derive(Debug, PartialEq)]
pub struct Node__struct {
    pub data_word_count: u16,
    pub pointer_count: u16,
    pub discriminant_count: u16,
    pub discriminant_offset: u32,
}
#[derive(Debug, PartialEq)]
pub struct Enumerant {
    pub code_order: u16,
}
#[derive(Debug, PartialEq)]
pub struct Node__enum {}
#[derive(Debug, PartialEq)]
pub struct Method {
    pub code_order: u16,
}
#[derive(Debug, PartialEq)]
pub struct Superclass {}
#[derive(Debug, PartialEq)]
pub struct Node__interface {}
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
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub enum Value {
    Void,
    Bool(),
    Int8(),
    Int16(),
    Int32(),
    Int64(),
    Uint8(),
    Uint16(u16),
    Uint32(u32),
    Uint64(),
    Float32(),
    Float64(),
    Text(),
    Data(),
    List(),
    Enum(u16),
    Struct(),
    Interface,
    AnyPointer(),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Value {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0, 0) {
            0u16 => Self::Void,
            17u16 => Self::Interface,
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Node__const {}
#[derive(Debug, PartialEq)]
pub struct Node__annotation {}
#[derive(Debug, PartialEq)]
pub struct Node__NestedNode {}
#[derive(Debug, PartialEq)]
pub struct Annotation {}
#[derive(Debug, PartialEq)]
pub struct Node__Parameter {}
#[derive(Debug, PartialEq)]
pub struct Brand {}
#[derive(Debug, PartialEq)]
pub struct Brand__Scope_0 {}
#[derive(Debug, PartialEq)]
pub enum Brand__Scope_1 {
    Bind(),
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
#[derive(Debug, PartialEq)]
pub enum Brand__Binding {
    Unbound,
    Type(),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Brand__Binding {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0, 0) {
            0u16 => Self::Unbound,
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Type__list {}
#[derive(Debug, PartialEq)]
pub struct Type__enum {}
#[derive(Debug, PartialEq)]
pub struct Type__struct {}
#[derive(Debug, PartialEq)]
pub struct Type__interface {}
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
    pub parameter_index: u16,
}
#[derive(Debug, PartialEq)]
pub struct Type__anyPointer__implicitMethodParameter {
    pub parameter_index: u16,
}
#[derive(Debug, PartialEq)]
pub enum Type__anyPointer {
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Type__anyPointer {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0, 0) {
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Field__slot {
    pub offset: u32,
}
#[derive(Debug, PartialEq)]
pub struct Field__group {}
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
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq)]
pub struct Node__SourceInfo__Member {}
#[derive(Debug, PartialEq)]
pub struct Node__SourceInfo {}
#[derive(Debug, PartialEq)]
pub struct CapnpVersion {
    pub major: u16,
}
#[derive(Debug, PartialEq)]
pub struct CodeGeneratorRequest__RequestedFile {}
#[derive(Debug, PartialEq)]
pub struct CodeGeneratorRequest {}
#[derive(Debug, PartialEq)]
pub struct CodeGeneratorRequest__RequestedFile__Import {}
