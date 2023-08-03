//! @generated
#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused)]
use anyhow::Result;
use capnp_plain::pointer::struct_pointer::{CapnpPlainStruct, StructReader};
#[derive(Debug, Clone, PartialEq)]
pub struct Field_0 {
    pub name: String,
    pub code_order: u16,
    pub annotations: Vec<Annotation>,
    pub discriminant_value: u16,
    pub ordinal: Field__Ordinal,
}
impl CapnpPlainStruct for Field_0 {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Field_0 {
            name: reader.read_text_field(0u32),
            code_order: reader.read_u16(0u32, 0u16),
            annotations: reader.read_list_field(1u32, |r| r.read_struct_children()),
            discriminant_value: reader.read_u16(1u32, 65535u16),
            ordinal: Field__Ordinal::try_from_reader(reader)?,
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Field_1 {
    Slot(Field__Slot),
    Group(Field__Group),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Field_1 {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(4u32, 0) {
            0u16 => Self::Slot(Field__Slot::try_from_reader(reader)?),
            1u16 => Self::Group(Field__Group::try_from_reader(reader)?),
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node_0 {
            id: reader.read_u64(0u32, 0u64),
            display_name: reader.read_text_field(0u32),
            display_name_prefix_length: reader.read_u32(2u32, 0u32),
            scope_id: reader.read_u64(2u32, 0u64),
            nested_nodes: reader.read_list_field(1u32, |r| r.read_struct_children()),
            annotations: reader.read_list_field(2u32, |r| r.read_struct_children()),
            parameters: reader.read_list_field(5u32, |r| r.read_struct_children()),
            is_generic: reader.read_bool(288u32, false),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
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
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(6u32, 0) {
            0u16 => Self::File,
            1u16 => Self::Struct(Node__Struct::try_from_reader(reader)?),
            2u16 => Self::Enum(Node__Enum::try_from_reader(reader)?),
            3u16 => Self::Interface(Node__Interface::try_from_reader(reader)?),
            4u16 => Self::Const(Node__Const::try_from_reader(reader)?),
            5u16 => Self::Annotation(Node__Annotation::try_from_reader(reader)?),
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
pub struct Node__Struct {
    pub data_word_count: u16,
    pub pointer_count: u16,
    pub is_group: bool,
    pub discriminant_count: u16,
    pub discriminant_offset: u32,
    pub fields: Vec<Field>,
}
impl CapnpPlainStruct for Node__Struct {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__Struct {
            data_word_count: reader.read_u16(7u32, 0u16),
            pointer_count: reader.read_u16(12u32, 0u16),
            is_group: reader.read_bool(224u32, false),
            discriminant_count: reader.read_u16(15u32, 0u16),
            discriminant_offset: reader.read_u32(8u32, 0u32),
            fields: reader.read_list_field(3u32, |r| r.read_struct_children()),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Enumerant {
    pub name: String,
    pub code_order: u16,
    pub annotations: Vec<Annotation>,
}
impl CapnpPlainStruct for Enumerant {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Enumerant {
            name: reader.read_text_field(0u32),
            code_order: reader.read_u16(0u32, 0u16),
            annotations: reader.read_list_field(1u32, |r| r.read_struct_children()),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Node__Enum {
    pub enumerants: Vec<Enumerant>,
}
impl CapnpPlainStruct for Node__Enum {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__Enum {
            enumerants: reader.read_list_field(3u32, |r| r.read_struct_children()),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
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
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Method {
            name: reader.read_text_field(0u32),
            code_order: reader.read_u16(0u32, 0u16),
            param_struct_type: reader.read_u64(1u32, 0u64),
            result_struct_type: reader.read_u64(2u32, 0u64),
            annotations: reader.read_list_field(1u32, |r| r.read_struct_children()),
            param_brand: reader.read_struct_child::<Brand>(2u32).ok().map(Box::new),
            result_brand: reader.read_struct_child::<Brand>(3u32).ok().map(Box::new),
            implicit_parameters: reader
                .read_list_field(4u32, |r| r.read_struct_children()),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Superclass {
    pub id: u64,
    pub brand: Option<Box<Brand>>,
}
impl CapnpPlainStruct for Superclass {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Superclass {
            id: reader.read_u64(0u32, 0u64),
            brand: reader.read_struct_child::<Brand>(0u32).ok().map(Box::new),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Node__Interface {
    pub methods: Vec<Method>,
    pub superclasses: Vec<Superclass>,
}
impl CapnpPlainStruct for Node__Interface {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__Interface {
            methods: reader.read_list_field(3u32, |r| r.read_struct_children()),
            superclasses: reader.read_list_field(4u32, |r| r.read_struct_children()),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
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
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0u32, 0) {
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
            14u16 => Self::List(Type__List::try_from_reader(reader)?),
            15u16 => Self::Enum(Type__Enum::try_from_reader(reader)?),
            16u16 => Self::Struct(Type__Struct::try_from_reader(reader)?),
            17u16 => Self::Interface(Type__Interface::try_from_reader(reader)?),
            18u16 => Self::AnyPointer(Type__AnyPointer::try_from_reader(reader)?),
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
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
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0u32, 0) {
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
            12u16 => Self::Text(reader.read_text_field(0u32)),
            15u16 => Self::Enum(reader.read_u16(1u32, 0u16)),
            17u16 => Self::Interface,
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Node__Const {
    pub r#type: Option<Box<Type>>,
    pub value: Option<Box<Value>>,
}
impl CapnpPlainStruct for Node__Const {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__Const {
            r#type: reader.read_struct_child::<Type>(3u32).ok().map(Box::new),
            value: reader.read_struct_child::<Value>(4u32).ok().map(Box::new),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
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
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__Annotation {
            r#type: reader.read_struct_child::<Type>(3u32).ok().map(Box::new),
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
#[derive(Debug, Clone, PartialEq)]
pub struct Node__NestedNode {
    pub name: String,
    pub id: u64,
}
impl CapnpPlainStruct for Node__NestedNode {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__NestedNode {
            name: reader.read_text_field(0u32),
            id: reader.read_u64(0u32, 0u64),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    pub id: u64,
    pub value: Option<Box<Value>>,
    pub brand: Option<Box<Brand>>,
}
impl CapnpPlainStruct for Annotation {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Annotation {
            id: reader.read_u64(0u32, 0u64),
            value: reader.read_struct_child::<Value>(0u32).ok().map(Box::new),
            brand: reader.read_struct_child::<Brand>(1u32).ok().map(Box::new),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Node__Parameter {
    pub name: String,
}
impl CapnpPlainStruct for Node__Parameter {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__Parameter {
            name: reader.read_text_field(0u32),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Brand {
    pub scopes: Vec<Brand__Scope>,
}
impl CapnpPlainStruct for Brand {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Brand {
            scopes: reader.read_list_field(0u32, |r| r.read_struct_children()),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Brand__Scope_0 {
    pub scope_id: u64,
}
impl CapnpPlainStruct for Brand__Scope_0 {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Brand__Scope_0 {
            scope_id: reader.read_u64(0u32, 0u64),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Brand__Scope_1 {
    Bind(Vec<Brand__Binding>),
    Inherit,
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Brand__Scope_1 {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(4u32, 0) {
            0u16 => {
                Self::Bind(reader.read_list_field(0u32, |r| r.read_struct_children()))
            }
            1u16 => Self::Inherit,
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
pub enum Brand__Binding {
    Unbound,
    Type(Type),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Brand__Binding {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(0u32, 0) {
            0u16 => Self::Unbound,
            1u16 => Self::Type(reader.read_struct_child::<Type>(0u32)?),
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Type__List {
    pub element_type: Option<Box<Type>>,
}
impl CapnpPlainStruct for Type__List {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Type__List {
            element_type: reader.read_struct_child::<Type>(0u32).ok().map(Box::new),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Type__Enum {
    pub type_id: u64,
    pub brand: Option<Box<Brand>>,
}
impl CapnpPlainStruct for Type__Enum {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Type__Enum {
            type_id: reader.read_u64(1u32, 0u64),
            brand: reader.read_struct_child::<Brand>(0u32).ok().map(Box::new),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Type__Struct {
    pub type_id: u64,
    pub brand: Option<Box<Brand>>,
}
impl CapnpPlainStruct for Type__Struct {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Type__Struct {
            type_id: reader.read_u64(1u32, 0u64),
            brand: reader.read_struct_child::<Brand>(0u32).ok().map(Box::new),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Type__Interface {
    pub type_id: u64,
    pub brand: Option<Box<Brand>>,
}
impl CapnpPlainStruct for Type__Interface {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Type__Interface {
            type_id: reader.read_u64(1u32, 0u64),
            brand: reader.read_struct_child::<Brand>(0u32).ok().map(Box::new),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Type__AnyPointer__Unconstrained {
    AnyKind,
    Struct,
    List,
    Capability,
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Type__AnyPointer__Unconstrained {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(5u32, 0) {
            0u16 => Self::AnyKind,
            1u16 => Self::Struct,
            2u16 => Self::List,
            3u16 => Self::Capability,
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Type__AnyPointer__Parameter {
    pub scope_id: u64,
    pub parameter_index: u16,
}
impl CapnpPlainStruct for Type__AnyPointer__Parameter {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Type__AnyPointer__Parameter {
            scope_id: reader.read_u64(2u32, 0u64),
            parameter_index: reader.read_u16(5u32, 0u16),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Type__AnyPointer__ImplicitMethodParameter {
    pub parameter_index: u16,
}
impl CapnpPlainStruct for Type__AnyPointer__ImplicitMethodParameter {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Type__AnyPointer__ImplicitMethodParameter {
            parameter_index: reader.read_u16(5u32, 0u16),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Type__AnyPointer {
    Unconstrained(Type__AnyPointer__Unconstrained),
    Parameter(Type__AnyPointer__Parameter),
    ImplicitMethodParameter(Type__AnyPointer__ImplicitMethodParameter),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Type__AnyPointer {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(4u32, 0) {
            0u16 => {
                Self::Unconstrained(
                    Type__AnyPointer__Unconstrained::try_from_reader(reader)?,
                )
            }
            1u16 => {
                Self::Parameter(Type__AnyPointer__Parameter::try_from_reader(reader)?)
            }
            2u16 => {
                Self::ImplicitMethodParameter(
                    Type__AnyPointer__ImplicitMethodParameter::try_from_reader(reader)?,
                )
            }
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Field__Slot {
    pub offset: u32,
    pub r#type: Option<Box<Type>>,
    pub default_value: Option<Box<Value>>,
    pub had_explicit_default: bool,
}
impl CapnpPlainStruct for Field__Slot {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Field__Slot {
            offset: reader.read_u32(1u32, 0u32),
            r#type: reader.read_struct_child::<Type>(2u32).ok().map(Box::new),
            default_value: reader.read_struct_child::<Value>(3u32).ok().map(Box::new),
            had_explicit_default: reader.read_bool(128u32, false),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Field__Group {
    pub type_id: u64,
}
impl CapnpPlainStruct for Field__Group {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Field__Group {
            type_id: reader.read_u64(2u32, 0u64),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Field__Ordinal {
    Implicit,
    Explicit(u16),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for Field__Ordinal {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = match reader.read_u16(5u32, 0) {
            0u16 => Self::Implicit,
            1u16 => Self::Explicit(reader.read_u16(6u32, 0u16)),
            _ => Self::UnknownDiscriminant,
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Node__SourceInfo__Member {
    pub doc_comment: String,
}
impl CapnpPlainStruct for Node__SourceInfo__Member {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__SourceInfo__Member {
            doc_comment: reader.read_text_field(0u32),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Node__SourceInfo {
    pub id: u64,
    pub doc_comment: String,
    pub members: Vec<Node__SourceInfo__Member>,
}
impl CapnpPlainStruct for Node__SourceInfo {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = Node__SourceInfo {
            id: reader.read_u64(0u32, 0u64),
            doc_comment: reader.read_text_field(0u32),
            members: reader.read_list_field(1u32, |r| r.read_struct_children()),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CapnpVersion {
    pub major: u16,
    pub minor: u8,
    pub micro: u8,
}
impl CapnpPlainStruct for CapnpVersion {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = CapnpVersion {
            major: reader.read_u16(0u32, 0u16),
            minor: reader.read_u8(2u32, 0u8),
            micro: reader.read_u8(3u32, 0u8),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CodeGeneratorRequest__RequestedFile {
    pub id: u64,
    pub filename: String,
    pub imports: Vec<CodeGeneratorRequest__RequestedFile__Import>,
}
impl CapnpPlainStruct for CodeGeneratorRequest__RequestedFile {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = CodeGeneratorRequest__RequestedFile {
            id: reader.read_u64(0u32, 0u64),
            filename: reader.read_text_field(0u32),
            imports: reader.read_list_field(1u32, |r| r.read_struct_children()),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CodeGeneratorRequest {
    pub nodes: Vec<Node>,
    pub requested_files: Vec<CodeGeneratorRequest__RequestedFile>,
    pub capnp_version: Option<Box<CapnpVersion>>,
    pub source_info: Vec<Node__SourceInfo>,
}
impl CapnpPlainStruct for CodeGeneratorRequest {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = CodeGeneratorRequest {
            nodes: reader.read_list_field(0u32, |r| r.read_struct_children()),
            requested_files: reader.read_list_field(1u32, |r| r.read_struct_children()),
            capnp_version: reader
                .read_struct_child::<CapnpVersion>(2u32)
                .ok()
                .map(Box::new),
            source_info: reader.read_list_field(3u32, |r| r.read_struct_children()),
        };
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CodeGeneratorRequest__RequestedFile__Import {
    pub id: u64,
    pub name: String,
}
impl CapnpPlainStruct for CodeGeneratorRequest__RequestedFile__Import {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let value = CodeGeneratorRequest__RequestedFile__Import {
            id: reader.read_u64(0u32, 0u64),
            name: reader.read_text_field(0u32),
        };
        Ok(value)
    }
}
