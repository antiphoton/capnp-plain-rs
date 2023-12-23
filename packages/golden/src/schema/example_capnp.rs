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
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum StandardPaperSize {
    UsLetter,
    A4,
    UnknownEnumerant(u16),
}
impl CapnpPlainEnum for StandardPaperSize {
    fn encode(self) -> u16 {
        match self {
            Self::UsLetter => 0u16,
            Self::A4 => 1u16,
            Self::UnknownEnumerant(x) => x,
        }
    }
    fn decode(x: u16) -> Self {
        match x {
            0u16 => Self::UsLetter,
            1u16 => Self::A4,
            x => Self::UnknownEnumerant(x),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomPaperSize {
    pub width: u16,
    pub height: u16,
}
impl CapnpPlainStruct for CustomPaperSize {
    fn from_node(reader: &CapnpStructNode) -> Self {
        CustomPaperSize {
            width: reader.read_u16(0u32, 0u16),
            height: reader.read_u16(1u32, 0u16),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u16(0u32, self.width, 0u16);
        writer.write_u16(1u32, self.height, 0u16);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PaperSize {
    Standard(StandardPaperSize),
    Custom(CustomPaperSize),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for PaperSize {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(1u32, 0) {
            0u16 => {
                Self::Standard(StandardPaperSize::decode(reader.read_u16(0u32, 0u16)))
            }
            1u16 => {
                Self::Custom(
                    CustomPaperSize::from_node(reader.read_struct(0u32).unwrap()),
                )
            }
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::Standard(value) => {
                writer.write_u16(0u32, (*value).encode(), 0u16);
                0u16
            }
            Self::Custom(value) => 1u16,
            _ => {
                return;
            }
        };
        writer.write_u16(1u32, discriminant_value, 0);
    }
}
