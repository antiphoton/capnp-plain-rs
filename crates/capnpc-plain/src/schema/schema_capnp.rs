//! @generated
#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused)]
use anyhow::Result;
use capnp_plain::{
    message::tree::struct_node::StructNode as CapnpStructNode, CapnpPlainStruct,
};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, FromPrimitive, Serialize, Deserialize)]
pub enum ElementSize {
    Empty = 0isize,
    Bit = 1isize,
    Byte = 2isize,
    TwoBytes = 3isize,
    FourBytes = 4isize,
    EightBytes = 5isize,
    Pointer = 6isize,
    InlineComposite = 7isize,
    UnknownEnumerant,
}
impl ElementSize {
    pub fn decode(x: u16) -> Self {
        match x {
            0..=7u16 => Self::from_u16(x).unwrap(),
            _ => Self::UnknownEnumerant,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field_0 {
    pub name: String,
    pub code_order: u16,
    pub annotations: Vec<Annotation>,
    pub discriminant_value: u16,
    pub ordinal: Field__Ordinal,
}
impl CapnpPlainStruct for Field_0 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Field_0 {
            name: reader.read_text(0u32),
            code_order: reader.read_u16(0u32, 0u16),
            annotations: reader.read_list(1u32, |r| r.read_struct_children()),
            discriminant_value: reader.read_u16(1u32, 65535u16),
            ordinal: Field__Ordinal::from_node(reader),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Field_1 {
    Slot(Field__Slot),
    Group(Field__Group),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Field_1 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(4u32, 0) {
            0u16 => Self::Slot(Field__Slot::from_node(reader)),
            1u16 => Self::Group(Field__Group::from_node(reader)),
            _ => Self::UnknownDiscriminant,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field(pub Field_0, pub Field_1);
impl CapnpPlainStruct for Field {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Field(Field_0::from_node(reader), Field_1::from_node(reader))
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node_0 {
    pub id: u64,
    pub display_name: String,
    pub display_name_prefix_length: u32,
    pub scope_id: u64,
    pub nested_nodes: Vec<Node__NestedNode>,
    pub annotations: Vec<Annotation>,
    pub parameters: Vec<Node__Parameter>,
    pub is_generic: bool,
}
impl CapnpPlainStruct for Node_0 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Node_0 {
            id: reader.read_u64(0u32, 0u64),
            display_name: reader.read_text(0u32),
            display_name_prefix_length: reader.read_u32(2u32, 0u32),
            scope_id: reader.read_u64(2u32, 0u64),
            nested_nodes: reader.read_list(1u32, |r| r.read_struct_children()),
            annotations: reader.read_list(2u32, |r| r.read_struct_children()),
            parameters: reader.read_list(5u32, |r| r.read_struct_children()),
            is_generic: reader.read_bool(288u32, false),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Node_1 {
    File,
    Struct(Node__Struct),
    Enum(Node__Enum),
    Interface(Node__Interface),
    Const(Node__Const),
    Annotation(Node__Annotation),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Node_1 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(6u32, 0) {
            0u16 => Self::File,
            1u16 => Self::Struct(Node__Struct::from_node(reader)),
            2u16 => Self::Enum(Node__Enum::from_node(reader)),
            3u16 => Self::Interface(Node__Interface::from_node(reader)),
            4u16 => Self::Const(Node__Const::from_node(reader)),
            5u16 => Self::Annotation(Node__Annotation::from_node(reader)),
            _ => Self::UnknownDiscriminant,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node(pub Node_0, pub Node_1);
impl CapnpPlainStruct for Node {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Node(Node_0::from_node(reader), Node_1::from_node(reader))
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node__Struct {
    pub data_word_count: u16,
    pub pointer_count: u16,
    pub preferred_list_encoding: ElementSize,
    pub is_group: bool,
    pub discriminant_count: u16,
    pub discriminant_offset: u32,
    pub fields: Vec<Field>,
}
impl CapnpPlainStruct for Node__Struct {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Node__Struct {
            data_word_count: reader.read_u16(7u32, 0u16),
            pointer_count: reader.read_u16(12u32, 0u16),
            preferred_list_encoding: ElementSize::decode(reader.read_u16(13u32, 0u16)),
            is_group: reader.read_bool(224u32, false),
            discriminant_count: reader.read_u16(15u32, 0u16),
            discriminant_offset: reader.read_u32(8u32, 0u32),
            fields: reader.read_list(3u32, |r| r.read_struct_children()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enumerant {
    pub name: String,
    pub code_order: u16,
    pub annotations: Vec<Annotation>,
}
impl CapnpPlainStruct for Enumerant {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Enumerant {
            name: reader.read_text(0u32),
            code_order: reader.read_u16(0u32, 0u16),
            annotations: reader.read_list(1u32, |r| r.read_struct_children()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node__Enum {
    pub enumerants: Vec<Enumerant>,
}
impl CapnpPlainStruct for Node__Enum {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Node__Enum {
            enumerants: reader.read_list(3u32, |r| r.read_struct_children()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    pub code_order: u16,
    pub param_struct_type: u64,
    pub result_struct_type: u64,
    pub annotations: Vec<Annotation>,
    pub param_brand: Option<Box<Brand>>,
    pub result_brand: Option<Box<Brand>>,
    pub implicit_parameters: Vec<Node__Parameter>,
}
impl CapnpPlainStruct for Method {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Method {
            name: reader.read_text(0u32),
            code_order: reader.read_u16(0u32, 0u16),
            param_struct_type: reader.read_u64(1u32, 0u64),
            result_struct_type: reader.read_u64(2u32, 0u64),
            annotations: reader.read_list(1u32, |r| r.read_struct_children()),
            param_brand: reader.read_struct(2u32).map(|x| Box::new(Brand::from_node(x))),
            result_brand: reader
                .read_struct(3u32)
                .map(|x| Box::new(Brand::from_node(x))),
            implicit_parameters: reader.read_list(4u32, |r| r.read_struct_children()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Superclass {
    pub id: u64,
    pub brand: Option<Box<Brand>>,
}
impl CapnpPlainStruct for Superclass {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Superclass {
            id: reader.read_u64(0u32, 0u64),
            brand: reader.read_struct(0u32).map(|x| Box::new(Brand::from_node(x))),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node__Interface {
    pub methods: Vec<Method>,
    pub superclasses: Vec<Superclass>,
}
impl CapnpPlainStruct for Node__Interface {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Node__Interface {
            methods: reader.read_list(3u32, |r| r.read_struct_children()),
            superclasses: reader.read_list(4u32, |r| r.read_struct_children()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
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
    List(Type__List),
    Enum(Type__Enum),
    Struct(Type__Struct),
    Interface(Type__Interface),
    AnyPointer(Type__AnyPointer),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Type {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(0u32, 0) {
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
            14u16 => Self::List(Type__List::from_node(reader)),
            15u16 => Self::Enum(Type__Enum::from_node(reader)),
            16u16 => Self::Struct(Type__Struct::from_node(reader)),
            17u16 => Self::Interface(Type__Interface::from_node(reader)),
            18u16 => Self::AnyPointer(Type__AnyPointer::from_node(reader)),
            _ => Self::UnknownDiscriminant,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
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
    List(),
    Enum(u16),
    Struct(),
    Interface,
    AnyPointer(),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Value {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(0u32, 0) {
            0u16 => Self::Void,
            1u16 => Self::Bool(reader.read_bool(16u32, false)),
            2u16 => Self::Int8(reader.read_i8(2u32, 0i8)),
            3u16 => Self::Int16(reader.read_i16(1u32, 0i16)),
            4u16 => Self::Int32(reader.read_i32(1u32, 0i32)),
            5u16 => Self::Int64(reader.read_i64(1u32, 0i64)),
            6u16 => Self::Uint8(reader.read_u8(2u32, 0u8)),
            7u16 => Self::Uint16(reader.read_u16(1u32, 0u16)),
            8u16 => Self::Uint32(reader.read_u32(1u32, 0u32)),
            9u16 => Self::Uint64(reader.read_u64(1u32, 0u64)),
            12u16 => Self::Text(reader.read_text(0u32)),
            15u16 => Self::Enum(reader.read_u16(1u32, 0u16)),
            17u16 => Self::Interface,
            _ => Self::UnknownDiscriminant,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node__Const {
    pub r#type: Option<Box<Type>>,
    pub value: Option<Box<Value>>,
}
impl CapnpPlainStruct for Node__Const {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Node__Const {
            r#type: reader.read_struct(3u32).map(|x| Box::new(Type::from_node(x))),
            value: reader.read_struct(4u32).map(|x| Box::new(Value::from_node(x))),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node__Annotation {
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
impl CapnpPlainStruct for Node__Annotation {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Node__Annotation {
            r#type: reader.read_struct(3u32).map(|x| Box::new(Type::from_node(x))),
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
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node__NestedNode {
    pub name: String,
    pub id: u64,
}
impl CapnpPlainStruct for Node__NestedNode {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Node__NestedNode {
            name: reader.read_text(0u32),
            id: reader.read_u64(0u32, 0u64),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Annotation {
    pub id: u64,
    pub value: Option<Box<Value>>,
    pub brand: Option<Box<Brand>>,
}
impl CapnpPlainStruct for Annotation {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Annotation {
            id: reader.read_u64(0u32, 0u64),
            value: reader.read_struct(0u32).map(|x| Box::new(Value::from_node(x))),
            brand: reader.read_struct(1u32).map(|x| Box::new(Brand::from_node(x))),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node__Parameter {
    pub name: String,
}
impl CapnpPlainStruct for Node__Parameter {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Node__Parameter {
            name: reader.read_text(0u32),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Brand {
    pub scopes: Vec<Brand__Scope>,
}
impl CapnpPlainStruct for Brand {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Brand {
            scopes: reader.read_list(0u32, |r| r.read_struct_children()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Brand__Scope_0 {
    pub scope_id: u64,
}
impl CapnpPlainStruct for Brand__Scope_0 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Brand__Scope_0 {
            scope_id: reader.read_u64(0u32, 0u64),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Brand__Scope_1 {
    Bind(Vec<Brand__Binding>),
    Inherit,
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Brand__Scope_1 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(4u32, 0) {
            0u16 => Self::Bind(reader.read_list(0u32, |r| r.read_struct_children())),
            1u16 => Self::Inherit,
            _ => Self::UnknownDiscriminant,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Brand__Scope(pub Brand__Scope_0, pub Brand__Scope_1);
impl CapnpPlainStruct for Brand__Scope {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Brand__Scope(
            Brand__Scope_0::from_node(reader),
            Brand__Scope_1::from_node(reader),
        )
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Brand__Binding {
    Unbound,
    Type(Type),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Brand__Binding {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(0u32, 0) {
            0u16 => Self::Unbound,
            1u16 => Self::Type(Type::from_node(reader.read_struct(0u32).unwrap())),
            _ => Self::UnknownDiscriminant,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type__List {
    pub element_type: Option<Box<Type>>,
}
impl CapnpPlainStruct for Type__List {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Type__List {
            element_type: reader.read_struct(0u32).map(|x| Box::new(Type::from_node(x))),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type__Enum {
    pub type_id: u64,
    pub brand: Option<Box<Brand>>,
}
impl CapnpPlainStruct for Type__Enum {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Type__Enum {
            type_id: reader.read_u64(1u32, 0u64),
            brand: reader.read_struct(0u32).map(|x| Box::new(Brand::from_node(x))),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type__Struct {
    pub type_id: u64,
    pub brand: Option<Box<Brand>>,
}
impl CapnpPlainStruct for Type__Struct {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Type__Struct {
            type_id: reader.read_u64(1u32, 0u64),
            brand: reader.read_struct(0u32).map(|x| Box::new(Brand::from_node(x))),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type__Interface {
    pub type_id: u64,
    pub brand: Option<Box<Brand>>,
}
impl CapnpPlainStruct for Type__Interface {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Type__Interface {
            type_id: reader.read_u64(1u32, 0u64),
            brand: reader.read_struct(0u32).map(|x| Box::new(Brand::from_node(x))),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Type__AnyPointer__Unconstrained {
    AnyKind,
    Struct,
    List,
    Capability,
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Type__AnyPointer__Unconstrained {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(5u32, 0) {
            0u16 => Self::AnyKind,
            1u16 => Self::Struct,
            2u16 => Self::List,
            3u16 => Self::Capability,
            _ => Self::UnknownDiscriminant,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type__AnyPointer__Parameter {
    pub scope_id: u64,
    pub parameter_index: u16,
}
impl CapnpPlainStruct for Type__AnyPointer__Parameter {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Type__AnyPointer__Parameter {
            scope_id: reader.read_u64(2u32, 0u64),
            parameter_index: reader.read_u16(5u32, 0u16),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type__AnyPointer__ImplicitMethodParameter {
    pub parameter_index: u16,
}
impl CapnpPlainStruct for Type__AnyPointer__ImplicitMethodParameter {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Type__AnyPointer__ImplicitMethodParameter {
            parameter_index: reader.read_u16(5u32, 0u16),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Type__AnyPointer {
    Unconstrained(Type__AnyPointer__Unconstrained),
    Parameter(Type__AnyPointer__Parameter),
    ImplicitMethodParameter(Type__AnyPointer__ImplicitMethodParameter),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Type__AnyPointer {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(4u32, 0) {
            0u16 => {
                Self::Unconstrained(Type__AnyPointer__Unconstrained::from_node(reader))
            }
            1u16 => Self::Parameter(Type__AnyPointer__Parameter::from_node(reader)),
            2u16 => {
                Self::ImplicitMethodParameter(
                    Type__AnyPointer__ImplicitMethodParameter::from_node(reader),
                )
            }
            _ => Self::UnknownDiscriminant,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field__Slot {
    pub offset: u32,
    pub r#type: Option<Box<Type>>,
    pub default_value: Option<Box<Value>>,
    pub had_explicit_default: bool,
}
impl CapnpPlainStruct for Field__Slot {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Field__Slot {
            offset: reader.read_u32(1u32, 0u32),
            r#type: reader.read_struct(2u32).map(|x| Box::new(Type::from_node(x))),
            default_value: reader
                .read_struct(3u32)
                .map(|x| Box::new(Value::from_node(x))),
            had_explicit_default: reader.read_bool(128u32, false),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field__Group {
    pub type_id: u64,
}
impl CapnpPlainStruct for Field__Group {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Field__Group {
            type_id: reader.read_u64(2u32, 0u64),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Field__Ordinal {
    Implicit,
    Explicit(u16),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Field__Ordinal {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(5u32, 0) {
            0u16 => Self::Implicit,
            1u16 => Self::Explicit(reader.read_u16(6u32, 0u16)),
            _ => Self::UnknownDiscriminant,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node__SourceInfo__Member {
    pub doc_comment: String,
}
impl CapnpPlainStruct for Node__SourceInfo__Member {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Node__SourceInfo__Member {
            doc_comment: reader.read_text(0u32),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node__SourceInfo {
    pub id: u64,
    pub doc_comment: String,
    pub members: Vec<Node__SourceInfo__Member>,
}
impl CapnpPlainStruct for Node__SourceInfo {
    fn from_node(reader: &CapnpStructNode) -> Self {
        Node__SourceInfo {
            id: reader.read_u64(0u32, 0u64),
            doc_comment: reader.read_text(0u32),
            members: reader.read_list(1u32, |r| r.read_struct_children()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapnpVersion {
    pub major: u16,
    pub minor: u8,
    pub micro: u8,
}
impl CapnpPlainStruct for CapnpVersion {
    fn from_node(reader: &CapnpStructNode) -> Self {
        CapnpVersion {
            major: reader.read_u16(0u32, 0u16),
            minor: reader.read_u8(2u32, 0u8),
            micro: reader.read_u8(3u32, 0u8),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeGeneratorRequest__RequestedFile {
    pub id: u64,
    pub filename: String,
    pub imports: Vec<CodeGeneratorRequest__RequestedFile__Import>,
}
impl CapnpPlainStruct for CodeGeneratorRequest__RequestedFile {
    fn from_node(reader: &CapnpStructNode) -> Self {
        CodeGeneratorRequest__RequestedFile {
            id: reader.read_u64(0u32, 0u64),
            filename: reader.read_text(0u32),
            imports: reader.read_list(1u32, |r| r.read_struct_children()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeGeneratorRequest {
    pub nodes: Vec<Node>,
    pub requested_files: Vec<CodeGeneratorRequest__RequestedFile>,
    pub capnp_version: Option<Box<CapnpVersion>>,
    pub source_info: Vec<Node__SourceInfo>,
}
impl CapnpPlainStruct for CodeGeneratorRequest {
    fn from_node(reader: &CapnpStructNode) -> Self {
        CodeGeneratorRequest {
            nodes: reader.read_list(0u32, |r| r.read_struct_children()),
            requested_files: reader.read_list(1u32, |r| r.read_struct_children()),
            capnp_version: reader
                .read_struct(2u32)
                .map(|x| Box::new(CapnpVersion::from_node(x))),
            source_info: reader.read_list(3u32, |r| r.read_struct_children()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeGeneratorRequest__RequestedFile__Import {
    pub id: u64,
    pub name: String,
}
impl CapnpPlainStruct for CodeGeneratorRequest__RequestedFile__Import {
    fn from_node(reader: &CapnpStructNode) -> Self {
        CodeGeneratorRequest__RequestedFile__Import {
            id: reader.read_u64(0u32, 0u64),
            name: reader.read_text(0u32),
        }
    }
}
