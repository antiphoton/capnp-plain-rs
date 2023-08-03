#![allow(non_camel_case_types)]

use anyhow::Result;
use capnp_plain::pointer::struct_pointer::{CapnpPlainStruct, StructReader};

use super::schema_capnp::{Field__Group, Field__Slot};

#[derive(Debug)]
pub struct Field_Common {
    pub name: String,
    pub discriminant_value: u16,
}

#[derive(Debug)]
pub enum Field_Union {
    Slot(Field__Slot),
    Group(Field__Group),
}

#[derive(Debug)]
pub struct Field(pub Field_Common, pub Option<Field_Union>);

impl CapnpPlainStruct for Field {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let common = Field_Common {
            name: reader.read_pointer(0)?.into_list_reader()?.read_text()?,
            discriminant_value: reader.read_u16(1, 65535),
        };
        let tag = reader.read_u16(4, 0);
        let union = match tag {
            0 => Some(Field_Union::Slot(Field__Slot::try_from_reader(reader)?)),
            1 => Some(Field_Union::Group(Field__Group::try_from_reader(reader)?)),
            _ => None,
        };
        Ok(Field(common, union))
    }
}

#[derive(Debug)]
pub struct Node__Struct {
    pub discriminant_count: u16,
    pub discriminant_offset: u32,
    pub fields: Vec<Field>,
}

impl CapnpPlainStruct for Node__Struct {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        Ok(Self {
            discriminant_count: reader.read_u16(15, 0),
            discriminant_offset: reader.read_u32(8, 0),
            fields: reader
                .read_pointer(3)?
                .into_list_reader()?
                .read_struct_children()?,
        })
    }
}

#[derive(Debug)]
pub struct Node_Common {
    pub id: u64,
    pub display_name: String,
    pub display_name_prefix_length: u32,
    pub scope_id: u64,
}

#[derive(Debug)]
pub enum Node_Union {
    Struct(Node__Struct),
}

#[derive(Debug)]
pub struct Node(pub Node_Common, pub Option<Node_Union>);

impl CapnpPlainStruct for Node {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        let common = Node_Common {
            id: reader.read_u64(0, 0),
            display_name: reader.read_pointer(0)?.into_list_reader()?.read_text()?,
            display_name_prefix_length: reader.read_u32(2, 0),
            scope_id: reader.read_u64(2, 0),
        };
        let tag = reader.read_u16(6, 0);
        let union = match tag {
            1 => Some(Node_Union::Struct(Node__Struct::try_from_reader(reader)?)),
            _ => None,
        };
        Ok(Node(common, union))
    }
}

#[derive(Debug)]
pub struct CodeGeneratorRequest__RequestedFile {
    pub filename: String,
}

impl CapnpPlainStruct for CodeGeneratorRequest__RequestedFile {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        Ok(Self {
            filename: reader.read_pointer(0)?.into_list_reader()?.read_text()?,
        })
    }
}

#[derive(Debug)]
pub struct CodeGeneratorRequest {
    pub nodes: Vec<Node>,
    pub requested_files: Vec<CodeGeneratorRequest__RequestedFile>,
}

impl CapnpPlainStruct for CodeGeneratorRequest {
    fn try_from_reader(reader: StructReader) -> Result<Self> {
        Ok(CodeGeneratorRequest {
            nodes: reader
                .read_pointer(0)?
                .into_list_reader()?
                .read_struct_children()?,
            requested_files: reader
                .read_pointer(1)?
                .into_list_reader()?
                .read_struct_children()?,
        })
    }
}
