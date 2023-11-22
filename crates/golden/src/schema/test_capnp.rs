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
    CapnpPlainStruct,
};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, Serialize, Deserialize)]
pub enum TestEnum {
    Foo = 0isize,
    Bar = 1isize,
    Baz = 2isize,
    Qux = 3isize,
    Quux = 4isize,
    Corge = 5isize,
    Grault = 6isize,
    Garply = 7isize,
    UnknownEnumerant,
}
impl TestEnum {
    pub fn decode(x: u16) -> Self {
        match x {
            0..=7u16 => Self::from_u16(x).unwrap(),
            _ => Self::UnknownEnumerant,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestAllTypes {
    pub void_field: (),
    pub bool_field: bool,
    pub int_8_field: i8,
    pub int_16_field: i16,
    pub int_32_field: i32,
    pub int_64_field: i64,
    pub u_int_8_field: u8,
    pub u_int_16_field: u16,
    pub u_int_32_field: u32,
    pub u_int_64_field: u64,
    pub float_32_field: f32,
    pub float_64_field: f64,
    pub text_field: String,
    pub data_field: Vec<u8>,
    pub struct_field: Option<Box<TestAllTypes>>,
    pub enum_field: TestEnum,
    pub interface_field: (),
    pub void_list: Vec<()>,
    pub bool_list: Vec<bool>,
    pub int_8_list: Vec<i8>,
    pub int_16_list: Vec<i16>,
    pub int_32_list: Vec<i32>,
    pub int_64_list: Vec<i64>,
    pub u_int_8_list: Vec<u8>,
    pub u_int_16_list: Vec<u16>,
    pub u_int_32_list: Vec<u32>,
    pub u_int_64_list: Vec<u64>,
    pub float_32_list: Vec<f32>,
    pub float_64_list: Vec<f64>,
    pub text_list: Vec<String>,
    pub data_list: Vec<Vec<u8>>,
    pub struct_list: Vec<TestAllTypes>,
    pub enum_list: Vec<TestEnum>,
    pub interface_list: Vec<()>,
}
impl CapnpPlainStruct for TestAllTypes {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestAllTypes {
            void_field: (),
            bool_field: reader.read_bool(0u32, false),
            int_8_field: reader.read_i8(1u32, 0i8),
            int_16_field: reader.read_i16(1u32, 0i16),
            int_32_field: reader.read_i32(1u32, 0i32),
            int_64_field: reader.read_i64(1u32, 0i64),
            u_int_8_field: reader.read_u8(16u32, 0u8),
            u_int_16_field: reader.read_u16(9u32, 0u16),
            u_int_32_field: reader.read_u32(5u32, 0u32),
            u_int_64_field: reader.read_u64(3u32, 0u64),
            float_32_field: reader.read_f32(8u32, 0.0),
            float_64_field: reader.read_f64(5u32, 0.0),
            text_field: reader.read_text(0u32),
            data_field: reader.read_data(1u32),
            struct_field: reader
                .read_struct(2u32)
                .map(|x| Box::new(TestAllTypes::from_node(x))),
            enum_field: TestEnum::decode(reader.read_u16(18u32, 0u16)),
            interface_field: (),
            void_list: vec![],
            bool_list: reader.read_list(4u32, |r| r.read_bool_children()),
            int_8_list: reader.read_list(5u32, |r| r.read_i8_children()),
            int_16_list: reader.read_list(6u32, |r| r.read_i16_children()),
            int_32_list: reader.read_list(7u32, |r| r.read_i32_children()),
            int_64_list: reader.read_list(8u32, |r| r.read_i64_children()),
            u_int_8_list: reader.read_list(9u32, |r| r.read_u8_children()),
            u_int_16_list: reader.read_list(10u32, |r| r.read_u16_children()),
            u_int_32_list: reader.read_list(11u32, |r| r.read_u32_children()),
            u_int_64_list: reader.read_list(12u32, |r| r.read_u64_children()),
            float_32_list: vec![],
            float_64_list: vec![],
            text_list: vec![],
            data_list: vec![],
            struct_list: reader.read_list(17u32, |r| r.read_struct_children()),
            enum_list: vec![],
            interface_list: vec![],
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_bool(0u32, self.bool_field, false);
        writer.write_i8(1u32, self.int_8_field, 0i8);
        writer.write_i16(1u32, self.int_16_field, 0i16);
        writer.write_i32(1u32, self.int_32_field, 0i32);
        writer.write_i64(1u32, self.int_64_field, 0i64);
        writer.write_u8(16u32, self.u_int_8_field, 0u8);
        writer.write_u16(9u32, self.u_int_16_field, 0u16);
        writer.write_u32(5u32, self.u_int_32_field, 0u32);
        writer.write_u64(3u32, self.u_int_64_field, 0u64);
        writer.write_text(0u32, &self.text_field);
        if let Some(child) = &self.struct_field {
            writer.write_child(2u32, CapnpNode::Struct(child.to_node()));
        }
        writer.write_u16(18u32, self.enum_field as u16, 0u16);
        writer
            .write_child(
                17u32,
                CapnpNode::List(CapnpListNode::write_struct_children(&self.struct_list)),
            );
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestDefaults {
    pub void_field: (),
    pub bool_field: bool,
    pub int_8_field: i8,
    pub int_16_field: i16,
    pub int_32_field: i32,
    pub int_64_field: i64,
    pub u_int_8_field: u8,
    pub u_int_16_field: u16,
    pub u_int_32_field: u32,
    pub u_int_64_field: u64,
    pub float_32_field: f32,
    pub float_64_field: f64,
    pub text_field: String,
    pub data_field: Vec<u8>,
    pub struct_field: Option<Box<TestAllTypes>>,
    pub enum_field: TestEnum,
    pub interface_field: (),
    pub void_list: Vec<()>,
    pub bool_list: Vec<bool>,
    pub int_8_list: Vec<i8>,
    pub int_16_list: Vec<i16>,
    pub int_32_list: Vec<i32>,
    pub int_64_list: Vec<i64>,
    pub u_int_8_list: Vec<u8>,
    pub u_int_16_list: Vec<u16>,
    pub u_int_32_list: Vec<u32>,
    pub u_int_64_list: Vec<u64>,
    pub float_32_list: Vec<f32>,
    pub float_64_list: Vec<f64>,
    pub text_list: Vec<String>,
    pub data_list: Vec<Vec<u8>>,
    pub struct_list: Vec<TestAllTypes>,
    pub enum_list: Vec<TestEnum>,
    pub interface_list: Vec<()>,
}
impl CapnpPlainStruct for TestDefaults {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestDefaults {
            void_field: (),
            bool_field: reader.read_bool(0u32, true),
            int_8_field: reader.read_i8(1u32, -123i8),
            int_16_field: reader.read_i16(1u32, -12345i16),
            int_32_field: reader.read_i32(1u32, -12345678i32),
            int_64_field: reader.read_i64(1u32, -123456789012345i64),
            u_int_8_field: reader.read_u8(16u32, 234u8),
            u_int_16_field: reader.read_u16(9u32, 45678u16),
            u_int_32_field: reader.read_u32(5u32, 3456789012u32),
            u_int_64_field: reader.read_u64(3u32, 12345678901234567890u64),
            float_32_field: reader.read_f32(8u32, 0.0),
            float_64_field: reader.read_f64(5u32, 0.0),
            text_field: reader.read_text(0u32),
            data_field: reader.read_data(1u32),
            struct_field: reader
                .read_struct(2u32)
                .map(|x| Box::new(TestAllTypes::from_node(x))),
            enum_field: TestEnum::decode(reader.read_u16(18u32, 5u16)),
            interface_field: (),
            void_list: vec![],
            bool_list: reader.read_list(4u32, |r| r.read_bool_children()),
            int_8_list: reader.read_list(5u32, |r| r.read_i8_children()),
            int_16_list: reader.read_list(6u32, |r| r.read_i16_children()),
            int_32_list: reader.read_list(7u32, |r| r.read_i32_children()),
            int_64_list: reader.read_list(8u32, |r| r.read_i64_children()),
            u_int_8_list: reader.read_list(9u32, |r| r.read_u8_children()),
            u_int_16_list: reader.read_list(10u32, |r| r.read_u16_children()),
            u_int_32_list: reader.read_list(11u32, |r| r.read_u32_children()),
            u_int_64_list: reader.read_list(12u32, |r| r.read_u64_children()),
            float_32_list: vec![],
            float_64_list: vec![],
            text_list: vec![],
            data_list: vec![],
            struct_list: reader.read_list(17u32, |r| r.read_struct_children()),
            enum_list: vec![],
            interface_list: vec![],
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_bool(0u32, self.bool_field, true);
        writer.write_i8(1u32, self.int_8_field, -123i8);
        writer.write_i16(1u32, self.int_16_field, -12345i16);
        writer.write_i32(1u32, self.int_32_field, -12345678i32);
        writer.write_i64(1u32, self.int_64_field, -123456789012345i64);
        writer.write_u8(16u32, self.u_int_8_field, 234u8);
        writer.write_u16(9u32, self.u_int_16_field, 45678u16);
        writer.write_u32(5u32, self.u_int_32_field, 3456789012u32);
        writer.write_u64(3u32, self.u_int_64_field, 12345678901234567890u64);
        writer.write_text(0u32, &self.text_field);
        if let Some(child) = &self.struct_field {
            writer.write_child(2u32, CapnpNode::Struct(child.to_node()));
        }
        writer.write_u16(18u32, self.enum_field as u16, 5u16);
        writer
            .write_child(
                17u32,
                CapnpNode::List(CapnpListNode::write_struct_children(&self.struct_list)),
            );
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestAnyPointer {}
impl CapnpPlainStruct for TestAnyPointer {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestAnyPointer {}
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestAnyOthers {}
impl CapnpPlainStruct for TestAnyOthers {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestAnyOthers {}
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestOutOfOrder {
    pub qux: String,
    pub grault: String,
    pub bar: String,
    pub foo: String,
    pub corge: String,
    pub waldo: String,
    pub quux: String,
    pub garply: String,
    pub baz: String,
}
impl CapnpPlainStruct for TestOutOfOrder {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestOutOfOrder {
            qux: reader.read_text(0u32),
            grault: reader.read_text(1u32),
            bar: reader.read_text(2u32),
            foo: reader.read_text(3u32),
            corge: reader.read_text(4u32),
            waldo: reader.read_text(5u32),
            quux: reader.read_text(6u32),
            garply: reader.read_text(7u32),
            baz: reader.read_text(8u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_text(0u32, &self.qux);
        writer.write_text(1u32, &self.grault);
        writer.write_text(2u32, &self.bar);
        writer.write_text(3u32, &self.foo);
        writer.write_text(4u32, &self.corge);
        writer.write_text(5u32, &self.waldo);
        writer.write_text(6u32, &self.quux);
        writer.write_text(7u32, &self.garply);
        writer.write_text(8u32, &self.baz);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestUnion {
    pub union_0: TestUnion__Union0,
    pub union_1: TestUnion__Union1,
    pub union_2: TestUnion__Union2,
    pub union_3: TestUnion__Union3,
    pub bit_0: bool,
    pub bit_2: bool,
    pub bit_3: bool,
    pub bit_4: bool,
    pub bit_5: bool,
    pub bit_6: bool,
    pub bit_7: bool,
    pub byte_0: u8,
}
impl CapnpPlainStruct for TestUnion {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestUnion {
            union_0: TestUnion__Union0::from_node(reader),
            union_1: TestUnion__Union1::from_node(reader),
            union_2: TestUnion__Union2::from_node(reader),
            union_3: TestUnion__Union3::from_node(reader),
            bit_0: reader.read_bool(128u32, false),
            bit_2: reader.read_bool(130u32, false),
            bit_3: reader.read_bool(131u32, false),
            bit_4: reader.read_bool(132u32, false),
            bit_5: reader.read_bool(133u32, false),
            bit_6: reader.read_bool(134u32, false),
            bit_7: reader.read_bool(135u32, false),
            byte_0: reader.read_u8(35u32, 0u8),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_bool(128u32, self.bit_0, false);
        writer.write_bool(130u32, self.bit_2, false);
        writer.write_bool(131u32, self.bit_3, false);
        writer.write_bool(132u32, self.bit_4, false);
        writer.write_bool(133u32, self.bit_5, false);
        writer.write_bool(134u32, self.bit_6, false);
        writer.write_bool(135u32, self.bit_7, false);
        writer.write_u8(35u32, self.byte_0, 0u8);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestUnion__Union0 {
    U0F0S0,
    U0F0S1(bool),
    U0F0S8(i8),
    U0F0S16(i16),
    U0F0S32(i32),
    U0F0S64(i64),
    U0F0Sp(String),
    U0F1S0,
    U0F1S1(bool),
    U0F1S8(i8),
    U0F1S16(i16),
    U0F1S32(i32),
    U0F1S64(i64),
    U0F1Sp(String),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestUnion__Union0 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(0u32, 0) {
            0u16 => Self::U0F0S0,
            1u16 => Self::U0F0S1(reader.read_bool(64u32, false)),
            2u16 => Self::U0F0S8(reader.read_i8(8u32, 0i8)),
            3u16 => Self::U0F0S16(reader.read_i16(4u32, 0i16)),
            4u16 => Self::U0F0S32(reader.read_i32(2u32, 0i32)),
            5u16 => Self::U0F0S64(reader.read_i64(1u32, 0i64)),
            6u16 => Self::U0F0Sp(reader.read_text(0u32)),
            7u16 => Self::U0F1S0,
            8u16 => Self::U0F1S1(reader.read_bool(64u32, false)),
            9u16 => Self::U0F1S8(reader.read_i8(8u32, 0i8)),
            10u16 => Self::U0F1S16(reader.read_i16(4u32, 0i16)),
            11u16 => Self::U0F1S32(reader.read_i32(2u32, 0i32)),
            12u16 => Self::U0F1S64(reader.read_i64(1u32, 0i64)),
            13u16 => Self::U0F1Sp(reader.read_text(0u32)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::U0F0S0 => 0u16,
            Self::U0F0S1(value) => {
                writer.write_bool(64u32, *value, false);
                1u16
            }
            Self::U0F0S8(value) => {
                writer.write_i8(8u32, *value, 0i8);
                2u16
            }
            Self::U0F0S16(value) => {
                writer.write_i16(4u32, *value, 0i16);
                3u16
            }
            Self::U0F0S32(value) => {
                writer.write_i32(2u32, *value, 0i32);
                4u16
            }
            Self::U0F0S64(value) => {
                writer.write_i64(1u32, *value, 0i64);
                5u16
            }
            Self::U0F0Sp(value) => {
                writer.write_text(0u32, &*value);
                6u16
            }
            Self::U0F1S0 => 7u16,
            Self::U0F1S1(value) => {
                writer.write_bool(64u32, *value, false);
                8u16
            }
            Self::U0F1S8(value) => {
                writer.write_i8(8u32, *value, 0i8);
                9u16
            }
            Self::U0F1S16(value) => {
                writer.write_i16(4u32, *value, 0i16);
                10u16
            }
            Self::U0F1S32(value) => {
                writer.write_i32(2u32, *value, 0i32);
                11u16
            }
            Self::U0F1S64(value) => {
                writer.write_i64(1u32, *value, 0i64);
                12u16
            }
            Self::U0F1Sp(value) => {
                writer.write_text(0u32, &*value);
                13u16
            }
            _ => {
                return;
            }
        };
        writer.write_u16(0u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestUnion__Union1 {
    U1F0S0,
    U1F0S1(bool),
    U1F1S1(bool),
    U1F0S8(i8),
    U1F1S8(i8),
    U1F0S16(i16),
    U1F1S16(i16),
    U1F0S32(i32),
    U1F1S32(i32),
    U1F0S64(i64),
    U1F1S64(i64),
    U1F0Sp(String),
    U1F1Sp(String),
    U1F2S0,
    U1F2S1(bool),
    U1F2S8(i8),
    U1F2S16(i16),
    U1F2S32(i32),
    U1F2S64(i64),
    U1F2Sp(String),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestUnion__Union1 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(1u32, 0) {
            0u16 => Self::U1F0S0,
            1u16 => Self::U1F0S1(reader.read_bool(129u32, false)),
            2u16 => Self::U1F1S1(reader.read_bool(129u32, false)),
            3u16 => Self::U1F0S8(reader.read_i8(17u32, 0i8)),
            4u16 => Self::U1F1S8(reader.read_i8(17u32, 0i8)),
            5u16 => Self::U1F0S16(reader.read_i16(9u32, 0i16)),
            6u16 => Self::U1F1S16(reader.read_i16(9u32, 0i16)),
            7u16 => Self::U1F0S32(reader.read_i32(5u32, 0i32)),
            8u16 => Self::U1F1S32(reader.read_i32(5u32, 0i32)),
            9u16 => Self::U1F0S64(reader.read_i64(3u32, 0i64)),
            10u16 => Self::U1F1S64(reader.read_i64(3u32, 0i64)),
            11u16 => Self::U1F0Sp(reader.read_text(1u32)),
            12u16 => Self::U1F1Sp(reader.read_text(1u32)),
            13u16 => Self::U1F2S0,
            14u16 => Self::U1F2S1(reader.read_bool(129u32, false)),
            15u16 => Self::U1F2S8(reader.read_i8(17u32, 0i8)),
            16u16 => Self::U1F2S16(reader.read_i16(9u32, 0i16)),
            17u16 => Self::U1F2S32(reader.read_i32(5u32, 0i32)),
            18u16 => Self::U1F2S64(reader.read_i64(3u32, 0i64)),
            19u16 => Self::U1F2Sp(reader.read_text(1u32)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::U1F0S0 => 0u16,
            Self::U1F0S1(value) => {
                writer.write_bool(129u32, *value, false);
                1u16
            }
            Self::U1F1S1(value) => {
                writer.write_bool(129u32, *value, false);
                2u16
            }
            Self::U1F0S8(value) => {
                writer.write_i8(17u32, *value, 0i8);
                3u16
            }
            Self::U1F1S8(value) => {
                writer.write_i8(17u32, *value, 0i8);
                4u16
            }
            Self::U1F0S16(value) => {
                writer.write_i16(9u32, *value, 0i16);
                5u16
            }
            Self::U1F1S16(value) => {
                writer.write_i16(9u32, *value, 0i16);
                6u16
            }
            Self::U1F0S32(value) => {
                writer.write_i32(5u32, *value, 0i32);
                7u16
            }
            Self::U1F1S32(value) => {
                writer.write_i32(5u32, *value, 0i32);
                8u16
            }
            Self::U1F0S64(value) => {
                writer.write_i64(3u32, *value, 0i64);
                9u16
            }
            Self::U1F1S64(value) => {
                writer.write_i64(3u32, *value, 0i64);
                10u16
            }
            Self::U1F0Sp(value) => {
                writer.write_text(1u32, &*value);
                11u16
            }
            Self::U1F1Sp(value) => {
                writer.write_text(1u32, &*value);
                12u16
            }
            Self::U1F2S0 => 13u16,
            Self::U1F2S1(value) => {
                writer.write_bool(129u32, *value, false);
                14u16
            }
            Self::U1F2S8(value) => {
                writer.write_i8(17u32, *value, 0i8);
                15u16
            }
            Self::U1F2S16(value) => {
                writer.write_i16(9u32, *value, 0i16);
                16u16
            }
            Self::U1F2S32(value) => {
                writer.write_i32(5u32, *value, 0i32);
                17u16
            }
            Self::U1F2S64(value) => {
                writer.write_i64(3u32, *value, 0i64);
                18u16
            }
            Self::U1F2Sp(value) => {
                writer.write_text(1u32, &*value);
                19u16
            }
            _ => {
                return;
            }
        };
        writer.write_u16(1u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestUnion__Union2 {
    U2F0S1(bool),
    U2F0S8(i8),
    U2F0S16(i16),
    U2F0S32(i32),
    U2F0S64(i64),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestUnion__Union2 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(2u32, 0) {
            0u16 => Self::U2F0S1(reader.read_bool(256u32, false)),
            1u16 => Self::U2F0S8(reader.read_i8(33u32, 0i8)),
            2u16 => Self::U2F0S16(reader.read_i16(18u32, 0i16)),
            3u16 => Self::U2F0S32(reader.read_i32(10u32, 0i32)),
            4u16 => Self::U2F0S64(reader.read_i64(6u32, 0i64)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::U2F0S1(value) => {
                writer.write_bool(256u32, *value, false);
                0u16
            }
            Self::U2F0S8(value) => {
                writer.write_i8(33u32, *value, 0i8);
                1u16
            }
            Self::U2F0S16(value) => {
                writer.write_i16(18u32, *value, 0i16);
                2u16
            }
            Self::U2F0S32(value) => {
                writer.write_i32(10u32, *value, 0i32);
                3u16
            }
            Self::U2F0S64(value) => {
                writer.write_i64(6u32, *value, 0i64);
                4u16
            }
            _ => {
                return;
            }
        };
        writer.write_u16(2u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestUnion__Union3 {
    U3F0S1(bool),
    U3F0S8(i8),
    U3F0S16(i16),
    U3F0S32(i32),
    U3F0S64(i64),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestUnion__Union3 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(3u32, 0) {
            0u16 => Self::U3F0S1(reader.read_bool(257u32, false)),
            1u16 => Self::U3F0S8(reader.read_i8(34u32, 0i8)),
            2u16 => Self::U3F0S16(reader.read_i16(19u32, 0i16)),
            3u16 => Self::U3F0S32(reader.read_i32(11u32, 0i32)),
            4u16 => Self::U3F0S64(reader.read_i64(7u32, 0i64)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::U3F0S1(value) => {
                writer.write_bool(257u32, *value, false);
                0u16
            }
            Self::U3F0S8(value) => {
                writer.write_i8(34u32, *value, 0i8);
                1u16
            }
            Self::U3F0S16(value) => {
                writer.write_i16(19u32, *value, 0i16);
                2u16
            }
            Self::U3F0S32(value) => {
                writer.write_i32(11u32, *value, 0i32);
                3u16
            }
            Self::U3F0S64(value) => {
                writer.write_i64(7u32, *value, 0i64);
                4u16
            }
            _ => {
                return;
            }
        };
        writer.write_u16(3u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestUnnamedUnion_0 {
    pub before: String,
    pub middle: u16,
    pub after: String,
}
impl CapnpPlainStruct for TestUnnamedUnion_0 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestUnnamedUnion_0 {
            before: reader.read_text(0u32),
            middle: reader.read_u16(1u32, 0u16),
            after: reader.read_text(1u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_text(0u32, &self.before);
        writer.write_u16(1u32, self.middle, 0u16);
        writer.write_text(1u32, &self.after);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestUnnamedUnion_1 {
    Foo(u16),
    Bar(u32),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestUnnamedUnion_1 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(2u32, 0) {
            0u16 => Self::Foo(reader.read_u16(0u32, 0u16)),
            1u16 => Self::Bar(reader.read_u32(2u32, 0u32)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::Foo(value) => {
                writer.write_u16(0u32, *value, 0u16);
                0u16
            }
            Self::Bar(value) => {
                writer.write_u32(2u32, *value, 0u32);
                1u16
            }
            _ => {
                return;
            }
        };
        writer.write_u16(2u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestUnnamedUnion(pub TestUnnamedUnion_0, pub TestUnnamedUnion_1);
impl CapnpPlainStruct for TestUnnamedUnion {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestUnnamedUnion(
            TestUnnamedUnion_0::from_node(reader),
            TestUnnamedUnion_1::from_node(reader),
        )
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        self.0.update_node(writer);
        self.1.update_node(writer);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestUnionInUnion__Outer__Inner {
    Foo(i32),
    Bar(i32),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestUnionInUnion__Outer__Inner {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(2u32, 0) {
            0u16 => Self::Foo(reader.read_i32(0u32, 0i32)),
            1u16 => Self::Bar(reader.read_i32(0u32, 0i32)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::Foo(value) => {
                writer.write_i32(0u32, *value, 0i32);
                0u16
            }
            Self::Bar(value) => {
                writer.write_i32(0u32, *value, 0i32);
                1u16
            }
            _ => {
                return;
            }
        };
        writer.write_u16(2u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestUnionInUnion {
    pub outer: TestUnionInUnion__Outer,
}
impl CapnpPlainStruct for TestUnionInUnion {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestUnionInUnion {
            outer: TestUnionInUnion__Outer::from_node(reader),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestUnionInUnion__Outer {
    Inner(TestUnionInUnion__Outer__Inner),
    Baz(i32),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestUnionInUnion__Outer {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(4u32, 0) {
            0u16 => Self::Inner(TestUnionInUnion__Outer__Inner::from_node(reader)),
            1u16 => Self::Baz(reader.read_i32(0u32, 0i32)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::Inner(value) => {
                value.update_node(writer);
                0u16
            }
            Self::Baz(value) => {
                writer.write_i32(0u32, *value, 0i32);
                1u16
            }
            _ => {
                return;
            }
        };
        writer.write_u16(4u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestGroups__Groups__Foo {
    pub corge: i32,
    pub grault: i64,
    pub garply: String,
}
impl CapnpPlainStruct for TestGroups__Groups__Foo {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestGroups__Groups__Foo {
            corge: reader.read_i32(0u32, 0i32),
            grault: reader.read_i64(1u32, 0i64),
            garply: reader.read_text(0u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_i32(0u32, self.corge, 0i32);
        writer.write_i64(1u32, self.grault, 0i64);
        writer.write_text(0u32, &self.garply);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestGroups__Groups__Baz {
    pub corge: i32,
    pub grault: String,
    pub garply: String,
}
impl CapnpPlainStruct for TestGroups__Groups__Baz {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestGroups__Groups__Baz {
            corge: reader.read_i32(0u32, 0i32),
            grault: reader.read_text(0u32),
            garply: reader.read_text(1u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_i32(0u32, self.corge, 0i32);
        writer.write_text(0u32, &self.grault);
        writer.write_text(1u32, &self.garply);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestGroups__Groups__Bar {
    pub corge: i32,
    pub grault: String,
    pub garply: i64,
}
impl CapnpPlainStruct for TestGroups__Groups__Bar {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestGroups__Groups__Bar {
            corge: reader.read_i32(0u32, 0i32),
            grault: reader.read_text(0u32),
            garply: reader.read_i64(1u32, 0i64),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_i32(0u32, self.corge, 0i32);
        writer.write_text(0u32, &self.grault);
        writer.write_i64(1u32, self.garply, 0i64);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestGroups {
    pub groups: TestGroups__Groups,
}
impl CapnpPlainStruct for TestGroups {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestGroups {
            groups: TestGroups__Groups::from_node(reader),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestGroups__Groups {
    Foo(TestGroups__Groups__Foo),
    Baz(TestGroups__Groups__Baz),
    Bar(TestGroups__Groups__Bar),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestGroups__Groups {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(2u32, 0) {
            0u16 => Self::Foo(TestGroups__Groups__Foo::from_node(reader)),
            1u16 => Self::Baz(TestGroups__Groups__Baz::from_node(reader)),
            2u16 => Self::Bar(TestGroups__Groups__Bar::from_node(reader)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::Foo(value) => {
                value.update_node(writer);
                0u16
            }
            Self::Baz(value) => {
                value.update_node(writer);
                1u16
            }
            Self::Bar(value) => {
                value.update_node(writer);
                2u16
            }
            _ => {
                return;
            }
        };
        writer.write_u16(2u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestInterleavedGroups__Group1__Corge {
    pub grault: u64,
    pub garply: u16,
    pub plugh: String,
    pub xyzzy: String,
}
impl CapnpPlainStruct for TestInterleavedGroups__Group1__Corge {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestInterleavedGroups__Group1__Corge {
            grault: reader.read_u64(4u32, 0u64),
            garply: reader.read_u16(12u32, 0u16),
            plugh: reader.read_text(2u32),
            xyzzy: reader.read_text(4u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u64(4u32, self.grault, 0u64);
        writer.write_u16(12u32, self.garply, 0u16);
        writer.write_text(2u32, &self.plugh);
        writer.write_text(4u32, &self.xyzzy);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestInterleavedGroups {
    pub group_1: TestInterleavedGroups__Group1,
    pub group_2: TestInterleavedGroups__Group2,
}
impl CapnpPlainStruct for TestInterleavedGroups {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestInterleavedGroups {
            group_1: TestInterleavedGroups__Group1::from_node(reader),
            group_2: TestInterleavedGroups__Group2::from_node(reader),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestInterleavedGroups__Group1_0 {
    pub foo: u32,
    pub bar: u64,
    pub waldo: String,
}
impl CapnpPlainStruct for TestInterleavedGroups__Group1_0 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestInterleavedGroups__Group1_0 {
            foo: reader.read_u32(0u32, 0u32),
            bar: reader.read_u64(1u32, 0u64),
            waldo: reader.read_text(0u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u32(0u32, self.foo, 0u32);
        writer.write_u64(1u32, self.bar, 0u64);
        writer.write_text(0u32, &self.waldo);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestInterleavedGroups__Group1_1 {
    Qux(u16),
    Corge(TestInterleavedGroups__Group1__Corge),
    Fred(String),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestInterleavedGroups__Group1_1 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(14u32, 0) {
            0u16 => Self::Qux(reader.read_u16(12u32, 0u16)),
            1u16 => Self::Corge(TestInterleavedGroups__Group1__Corge::from_node(reader)),
            2u16 => Self::Fred(reader.read_text(2u32)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::Qux(value) => {
                writer.write_u16(12u32, *value, 0u16);
                0u16
            }
            Self::Corge(value) => {
                value.update_node(writer);
                1u16
            }
            Self::Fred(value) => {
                writer.write_text(2u32, &*value);
                2u16
            }
            _ => {
                return;
            }
        };
        writer.write_u16(14u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestInterleavedGroups__Group1(
    pub TestInterleavedGroups__Group1_0,
    pub TestInterleavedGroups__Group1_1,
);
impl CapnpPlainStruct for TestInterleavedGroups__Group1 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestInterleavedGroups__Group1(
            TestInterleavedGroups__Group1_0::from_node(reader),
            TestInterleavedGroups__Group1_1::from_node(reader),
        )
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        self.0.update_node(writer);
        self.1.update_node(writer);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestInterleavedGroups__Group2__Corge {
    pub grault: u64,
    pub garply: u16,
    pub plugh: String,
    pub xyzzy: String,
}
impl CapnpPlainStruct for TestInterleavedGroups__Group2__Corge {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestInterleavedGroups__Group2__Corge {
            grault: reader.read_u64(5u32, 0u64),
            garply: reader.read_u16(13u32, 0u16),
            plugh: reader.read_text(3u32),
            xyzzy: reader.read_text(5u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u64(5u32, self.grault, 0u64);
        writer.write_u16(13u32, self.garply, 0u16);
        writer.write_text(3u32, &self.plugh);
        writer.write_text(5u32, &self.xyzzy);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestInterleavedGroups__Group2_0 {
    pub foo: u32,
    pub bar: u64,
    pub waldo: String,
}
impl CapnpPlainStruct for TestInterleavedGroups__Group2_0 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestInterleavedGroups__Group2_0 {
            foo: reader.read_u32(1u32, 0u32),
            bar: reader.read_u64(2u32, 0u64),
            waldo: reader.read_text(1u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u32(1u32, self.foo, 0u32);
        writer.write_u64(2u32, self.bar, 0u64);
        writer.write_text(1u32, &self.waldo);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestInterleavedGroups__Group2_1 {
    Qux(u16),
    Corge(TestInterleavedGroups__Group2__Corge),
    Fred(String),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestInterleavedGroups__Group2_1 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(15u32, 0) {
            0u16 => Self::Qux(reader.read_u16(13u32, 0u16)),
            1u16 => Self::Corge(TestInterleavedGroups__Group2__Corge::from_node(reader)),
            2u16 => Self::Fred(reader.read_text(3u32)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::Qux(value) => {
                writer.write_u16(13u32, *value, 0u16);
                0u16
            }
            Self::Corge(value) => {
                value.update_node(writer);
                1u16
            }
            Self::Fred(value) => {
                writer.write_text(3u32, &*value);
                2u16
            }
            _ => {
                return;
            }
        };
        writer.write_u16(15u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestInterleavedGroups__Group2(
    pub TestInterleavedGroups__Group2_0,
    pub TestInterleavedGroups__Group2_1,
);
impl CapnpPlainStruct for TestInterleavedGroups__Group2 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestInterleavedGroups__Group2(
            TestInterleavedGroups__Group2_0::from_node(reader),
            TestInterleavedGroups__Group2_1::from_node(reader),
        )
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        self.0.update_node(writer);
        self.1.update_node(writer);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestUnionDefaults {
    pub s_16_s_8_s_64_s_8_set: Option<Box<TestUnion>>,
    pub s_0_sps_1_s_32_set: Option<Box<TestUnion>>,
    pub unnamed_1: Option<Box<TestUnnamedUnion>>,
    pub unnamed_2: Option<Box<TestUnnamedUnion>>,
}
impl CapnpPlainStruct for TestUnionDefaults {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestUnionDefaults {
            s_16_s_8_s_64_s_8_set: reader
                .read_struct(0u32)
                .map(|x| Box::new(TestUnion::from_node(x))),
            s_0_sps_1_s_32_set: reader
                .read_struct(1u32)
                .map(|x| Box::new(TestUnion::from_node(x))),
            unnamed_1: reader
                .read_struct(2u32)
                .map(|x| Box::new(TestUnnamedUnion::from_node(x))),
            unnamed_2: reader
                .read_struct(3u32)
                .map(|x| Box::new(TestUnnamedUnion::from_node(x))),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        if let Some(child) = &self.s_16_s_8_s_64_s_8_set {
            writer.write_child(0u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.s_0_sps_1_s_32_set {
            writer.write_child(1u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.unnamed_1 {
            writer.write_child(2u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.unnamed_2 {
            writer.write_child(3u32, CapnpNode::Struct(child.to_node()));
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestNestedTypes__NestedStruct {
    pub outer_nested_enum: TestNestedTypes__NestedEnum,
    pub inner_nested_enum: TestNestedTypes__NestedStruct__NestedEnum,
}
impl CapnpPlainStruct for TestNestedTypes__NestedStruct {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestNestedTypes__NestedStruct {
            outer_nested_enum: TestNestedTypes__NestedEnum::decode(
                reader.read_u16(0u32, 1u16),
            ),
            inner_nested_enum: TestNestedTypes__NestedStruct__NestedEnum::decode(
                reader.read_u16(1u32, 2u16),
            ),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u16(0u32, self.outer_nested_enum as u16, 1u16);
        writer.write_u16(1u32, self.inner_nested_enum as u16, 2u16);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, Serialize, Deserialize)]
pub enum TestNestedTypes__NestedEnum {
    Foo = 0isize,
    Bar = 1isize,
    UnknownEnumerant,
}
impl TestNestedTypes__NestedEnum {
    pub fn decode(x: u16) -> Self {
        match x {
            0..=1u16 => Self::from_u16(x).unwrap(),
            _ => Self::UnknownEnumerant,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, Serialize, Deserialize)]
pub enum TestNestedTypes__NestedStruct__NestedEnum {
    Baz = 0isize,
    Qux = 1isize,
    Quux = 2isize,
    UnknownEnumerant,
}
impl TestNestedTypes__NestedStruct__NestedEnum {
    pub fn decode(x: u16) -> Self {
        match x {
            0..=2u16 => Self::from_u16(x).unwrap(),
            _ => Self::UnknownEnumerant,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestNestedTypes {
    pub nested_struct: Option<Box<TestNestedTypes__NestedStruct>>,
    pub outer_nested_enum: TestNestedTypes__NestedEnum,
    pub inner_nested_enum: TestNestedTypes__NestedStruct__NestedEnum,
}
impl CapnpPlainStruct for TestNestedTypes {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestNestedTypes {
            nested_struct: reader
                .read_struct(0u32)
                .map(|x| Box::new(TestNestedTypes__NestedStruct::from_node(x))),
            outer_nested_enum: TestNestedTypes__NestedEnum::decode(
                reader.read_u16(0u32, 1u16),
            ),
            inner_nested_enum: TestNestedTypes__NestedStruct__NestedEnum::decode(
                reader.read_u16(1u32, 2u16),
            ),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        if let Some(child) = &self.nested_struct {
            writer.write_child(0u32, CapnpNode::Struct(child.to_node()));
        }
        writer.write_u16(0u32, self.outer_nested_enum as u16, 1u16);
        writer.write_u16(1u32, self.inner_nested_enum as u16, 2u16);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestUsing {
    pub inner_nested_enum: TestNestedTypes__NestedStruct__NestedEnum,
    pub outer_nested_enum: TestNestedTypes__NestedEnum,
}
impl CapnpPlainStruct for TestUsing {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestUsing {
            inner_nested_enum: TestNestedTypes__NestedStruct__NestedEnum::decode(
                reader.read_u16(0u32, 2u16),
            ),
            outer_nested_enum: TestNestedTypes__NestedEnum::decode(
                reader.read_u16(1u32, 1u16),
            ),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u16(0u32, self.inner_nested_enum as u16, 2u16);
        writer.write_u16(1u32, self.outer_nested_enum as u16, 1u16);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestLists__Struct0 {
    pub f: (),
}
impl CapnpPlainStruct for TestLists__Struct0 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestLists__Struct0 { f: () }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestLists__Struct1 {
    pub f: bool,
}
impl CapnpPlainStruct for TestLists__Struct1 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestLists__Struct1 {
            f: reader.read_bool(0u32, false),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_bool(0u32, self.f, false);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestLists__Struct8 {
    pub f: u8,
}
impl CapnpPlainStruct for TestLists__Struct8 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestLists__Struct8 {
            f: reader.read_u8(0u32, 0u8),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u8(0u32, self.f, 0u8);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestLists__Struct16 {
    pub f: u16,
}
impl CapnpPlainStruct for TestLists__Struct16 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestLists__Struct16 {
            f: reader.read_u16(0u32, 0u16),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u16(0u32, self.f, 0u16);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestLists__Struct32 {
    pub f: u32,
}
impl CapnpPlainStruct for TestLists__Struct32 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestLists__Struct32 {
            f: reader.read_u32(0u32, 0u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u32(0u32, self.f, 0u32);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestLists__Struct64 {
    pub f: u64,
}
impl CapnpPlainStruct for TestLists__Struct64 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestLists__Struct64 {
            f: reader.read_u64(0u32, 0u64),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u64(0u32, self.f, 0u64);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestLists__StructP {
    pub f: String,
}
impl CapnpPlainStruct for TestLists__StructP {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestLists__StructP {
            f: reader.read_text(0u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_text(0u32, &self.f);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestLists {
    pub list_0: Vec<TestLists__Struct0>,
    pub list_1: Vec<TestLists__Struct1>,
    pub list_8: Vec<TestLists__Struct8>,
    pub list_16: Vec<TestLists__Struct16>,
    pub list_32: Vec<TestLists__Struct32>,
    pub list_64: Vec<TestLists__Struct64>,
    pub list_p: Vec<TestLists__StructP>,
    pub int_32_list_list: Vec<Vec<i32>>,
    pub text_list_list: Vec<Vec<String>>,
    pub struct_list_list: Vec<Vec<TestAllTypes>>,
}
impl CapnpPlainStruct for TestLists {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestLists {
            list_0: reader.read_list(0u32, |r| r.read_struct_children()),
            list_1: reader.read_list(1u32, |r| r.read_struct_children()),
            list_8: reader.read_list(2u32, |r| r.read_struct_children()),
            list_16: reader.read_list(3u32, |r| r.read_struct_children()),
            list_32: reader.read_list(4u32, |r| r.read_struct_children()),
            list_64: reader.read_list(5u32, |r| r.read_struct_children()),
            list_p: reader.read_list(6u32, |r| r.read_struct_children()),
            int_32_list_list: vec![],
            text_list_list: vec![],
            struct_list_list: vec![],
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer
            .write_child(
                0u32,
                CapnpNode::List(CapnpListNode::write_struct_children(&self.list_0)),
            );
        writer
            .write_child(
                1u32,
                CapnpNode::List(CapnpListNode::write_struct_children(&self.list_1)),
            );
        writer
            .write_child(
                2u32,
                CapnpNode::List(CapnpListNode::write_struct_children(&self.list_8)),
            );
        writer
            .write_child(
                3u32,
                CapnpNode::List(CapnpListNode::write_struct_children(&self.list_16)),
            );
        writer
            .write_child(
                4u32,
                CapnpNode::List(CapnpListNode::write_struct_children(&self.list_32)),
            );
        writer
            .write_child(
                5u32,
                CapnpNode::List(CapnpListNode::write_struct_children(&self.list_64)),
            );
        writer
            .write_child(
                6u32,
                CapnpNode::List(CapnpListNode::write_struct_children(&self.list_p)),
            );
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestLists__Struct0C {
    pub f: (),
    pub pad: String,
}
impl CapnpPlainStruct for TestLists__Struct0C {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestLists__Struct0C {
            f: (),
            pad: reader.read_text(0u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_text(0u32, &self.pad);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestLists__Struct1C {
    pub f: bool,
    pub pad: String,
}
impl CapnpPlainStruct for TestLists__Struct1C {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestLists__Struct1C {
            f: reader.read_bool(0u32, false),
            pad: reader.read_text(0u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_bool(0u32, self.f, false);
        writer.write_text(0u32, &self.pad);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestLists__Struct8C {
    pub f: u8,
    pub pad: String,
}
impl CapnpPlainStruct for TestLists__Struct8C {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestLists__Struct8C {
            f: reader.read_u8(0u32, 0u8),
            pad: reader.read_text(0u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u8(0u32, self.f, 0u8);
        writer.write_text(0u32, &self.pad);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestLists__Struct16C {
    pub f: u16,
    pub pad: String,
}
impl CapnpPlainStruct for TestLists__Struct16C {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestLists__Struct16C {
            f: reader.read_u16(0u32, 0u16),
            pad: reader.read_text(0u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u16(0u32, self.f, 0u16);
        writer.write_text(0u32, &self.pad);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestLists__Struct32C {
    pub f: u32,
    pub pad: String,
}
impl CapnpPlainStruct for TestLists__Struct32C {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestLists__Struct32C {
            f: reader.read_u32(0u32, 0u32),
            pad: reader.read_text(0u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u32(0u32, self.f, 0u32);
        writer.write_text(0u32, &self.pad);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestLists__Struct64C {
    pub f: u64,
    pub pad: String,
}
impl CapnpPlainStruct for TestLists__Struct64C {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestLists__Struct64C {
            f: reader.read_u64(0u32, 0u64),
            pad: reader.read_text(0u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u64(0u32, self.f, 0u64);
        writer.write_text(0u32, &self.pad);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestLists__StructPc {
    pub f: String,
    pub pad: u64,
}
impl CapnpPlainStruct for TestLists__StructPc {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestLists__StructPc {
            f: reader.read_text(0u32),
            pad: reader.read_u64(0u32, 0u64),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_text(0u32, &self.f);
        writer.write_u64(0u32, self.pad, 0u64);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestFieldZeroIsBit {
    pub bit: bool,
    pub second_bit: bool,
    pub third_field: u8,
}
impl CapnpPlainStruct for TestFieldZeroIsBit {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestFieldZeroIsBit {
            bit: reader.read_bool(0u32, false),
            second_bit: reader.read_bool(1u32, true),
            third_field: reader.read_u8(1u32, 123u8),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_bool(0u32, self.bit, false);
        writer.write_bool(1u32, self.second_bit, true);
        writer.write_u8(1u32, self.third_field, 123u8);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestListDefaults {
    pub lists: Option<Box<TestLists>>,
}
impl CapnpPlainStruct for TestListDefaults {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestListDefaults {
            lists: reader.read_struct(0u32).map(|x| Box::new(TestLists::from_node(x))),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        if let Some(child) = &self.lists {
            writer.write_child(0u32, CapnpNode::Struct(child.to_node()));
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestLateUnion {
    pub foo: i32,
    pub bar: String,
    pub baz: i16,
    pub the_union: TestLateUnion__TheUnion,
    pub another_union: TestLateUnion__AnotherUnion,
}
impl CapnpPlainStruct for TestLateUnion {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestLateUnion {
            foo: reader.read_i32(0u32, 0i32),
            bar: reader.read_text(0u32),
            baz: reader.read_i16(2u32, 0i16),
            the_union: TestLateUnion__TheUnion::from_node(reader),
            another_union: TestLateUnion__AnotherUnion::from_node(reader),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_i32(0u32, self.foo, 0i32);
        writer.write_text(0u32, &self.bar);
        writer.write_i16(2u32, self.baz, 0i16);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestLateUnion__TheUnion {
    Qux(String),
    Corge(Vec<i32>),
    Grault(f32),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestLateUnion__TheUnion {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(3u32, 0) {
            0u16 => Self::Qux(reader.read_text(1u32)),
            1u16 => Self::Corge(reader.read_list(1u32, |r| r.read_i32_children())),
            2u16 => Self::Grault(reader.read_f32(2u32, 0.0)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::Qux(value) => {
                writer.write_text(1u32, &*value);
                0u16
            }
            Self::Corge(..) => 1u16,
            Self::Grault(..) => 2u16,
            _ => {
                return;
            }
        };
        writer.write_u16(3u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestLateUnion__AnotherUnion {
    Qux(String),
    Corge(Vec<i32>),
    Grault(f32),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestLateUnion__AnotherUnion {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(6u32, 0) {
            0u16 => Self::Qux(reader.read_text(2u32)),
            1u16 => Self::Corge(reader.read_list(2u32, |r| r.read_i32_children())),
            2u16 => Self::Grault(reader.read_f32(4u32, 0.0)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::Qux(value) => {
                writer.write_text(2u32, &*value);
                0u16
            }
            Self::Corge(..) => 1u16,
            Self::Grault(..) => 2u16,
            _ => {
                return;
            }
        };
        writer.write_u16(6u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestOldVersion {
    pub old_1: i64,
    pub old_2: String,
    pub old_3: Option<Box<TestOldVersion>>,
}
impl CapnpPlainStruct for TestOldVersion {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestOldVersion {
            old_1: reader.read_i64(0u32, 0i64),
            old_2: reader.read_text(0u32),
            old_3: reader
                .read_struct(1u32)
                .map(|x| Box::new(TestOldVersion::from_node(x))),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_i64(0u32, self.old_1, 0i64);
        writer.write_text(0u32, &self.old_2);
        if let Some(child) = &self.old_3 {
            writer.write_child(1u32, CapnpNode::Struct(child.to_node()));
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestNewVersion {
    pub old_1: i64,
    pub old_2: String,
    pub old_3: Option<Box<TestNewVersion>>,
    pub new_1: i64,
    pub new_2: String,
}
impl CapnpPlainStruct for TestNewVersion {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestNewVersion {
            old_1: reader.read_i64(0u32, 0i64),
            old_2: reader.read_text(0u32),
            old_3: reader
                .read_struct(1u32)
                .map(|x| Box::new(TestNewVersion::from_node(x))),
            new_1: reader.read_i64(1u32, 987i64),
            new_2: reader.read_text(2u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_i64(0u32, self.old_1, 0i64);
        writer.write_text(0u32, &self.old_2);
        if let Some(child) = &self.old_3 {
            writer.write_child(1u32, CapnpNode::Struct(child.to_node()));
        }
        writer.write_i64(1u32, self.new_1, 987i64);
        writer.write_text(2u32, &self.new_2);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestOldUnionVersion {
    A,
    B(u64),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestOldUnionVersion {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(0u32, 0) {
            0u16 => Self::A,
            1u16 => Self::B(reader.read_u64(1u32, 0u64)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::A => 0u16,
            Self::B(value) => {
                writer.write_u64(1u32, *value, 0u64);
                1u16
            }
            _ => {
                return;
            }
        };
        writer.write_u16(0u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestNewUnionVersion {
    A(TestNewUnionVersion__A),
    B(u64),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestNewUnionVersion {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(0u32, 0) {
            0u16 => Self::A(TestNewUnionVersion__A::from_node(reader)),
            1u16 => Self::B(reader.read_u64(1u32, 0u64)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::A(value) => {
                value.update_node(writer);
                0u16
            }
            Self::B(value) => {
                writer.write_u64(1u32, *value, 0u64);
                1u16
            }
            _ => {
                return;
            }
        };
        writer.write_u16(0u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestNewUnionVersion__A {
    A0,
    A1(u64),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestNewUnionVersion__A {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(4u32, 0) {
            0u16 => Self::A0,
            1u16 => Self::A1(reader.read_u64(2u32, 0u64)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::A0 => 0u16,
            Self::A1(value) => {
                writer.write_u64(2u32, *value, 0u64);
                1u16
            }
            _ => {
                return;
            }
        };
        writer.write_u16(4u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestStructUnion__SomeStruct {
    pub some_text: String,
    pub more_text: String,
}
impl CapnpPlainStruct for TestStructUnion__SomeStruct {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestStructUnion__SomeStruct {
            some_text: reader.read_text(0u32),
            more_text: reader.read_text(1u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_text(0u32, &self.some_text);
        writer.write_text(1u32, &self.more_text);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestStructUnion {
    pub un: TestStructUnion__Un,
}
impl CapnpPlainStruct for TestStructUnion {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestStructUnion {
            un: TestStructUnion__Un::from_node(reader),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestStructUnion__Un {
    Struct(TestStructUnion__SomeStruct),
    Object(TestAnyPointer),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestStructUnion__Un {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(0u32, 0) {
            0u16 => {
                Self::Struct(
                    TestStructUnion__SomeStruct::from_node(
                        reader.read_struct(0u32).unwrap(),
                    ),
                )
            }
            1u16 => {
                Self::Object(
                    TestAnyPointer::from_node(reader.read_struct(0u32).unwrap()),
                )
            }
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::Struct(value) => 0u16,
            Self::Object(value) => 1u16,
            _ => {
                return;
            }
        };
        writer.write_u16(0u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestPrintInlineStructs__InlineStruct {
    pub int_32_field: i32,
    pub text_field: String,
}
impl CapnpPlainStruct for TestPrintInlineStructs__InlineStruct {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestPrintInlineStructs__InlineStruct {
            int_32_field: reader.read_i32(0u32, 0i32),
            text_field: reader.read_text(0u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_i32(0u32, self.int_32_field, 0i32);
        writer.write_text(0u32, &self.text_field);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestPrintInlineStructs {
    pub some_text: String,
    pub struct_list: Vec<TestPrintInlineStructs__InlineStruct>,
}
impl CapnpPlainStruct for TestPrintInlineStructs {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestPrintInlineStructs {
            some_text: reader.read_text(0u32),
            struct_list: reader.read_list(1u32, |r| r.read_struct_children()),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_text(0u32, &self.some_text);
        writer
            .write_child(
                1u32,
                CapnpNode::List(CapnpListNode::write_struct_children(&self.struct_list)),
            );
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestWholeFloatDefault {
    pub field: f32,
    pub big_field: f32,
}
impl CapnpPlainStruct for TestWholeFloatDefault {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestWholeFloatDefault {
            field: reader.read_f32(0u32, 0.0),
            big_field: reader.read_f32(1u32, 0.0),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestGenerics_0 {
    pub rev: Option<Box<TestGenerics>>,
    pub list: Vec<TestGenerics__Inner>,
}
impl CapnpPlainStruct for TestGenerics_0 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestGenerics_0 {
            rev: reader.read_struct(1u32).map(|x| Box::new(TestGenerics::from_node(x))),
            list: reader.read_list(2u32, |r| r.read_struct_children()),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        if let Some(child) = &self.rev {
            writer.write_child(1u32, CapnpNode::Struct(child.to_node()));
        }
        writer
            .write_child(
                2u32,
                CapnpNode::List(CapnpListNode::write_struct_children(&self.list)),
            );
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestGenerics_1 {
    Uv,
    Ug(TestGenerics__Ug),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestGenerics_1 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(0u32, 0) {
            0u16 => Self::Uv,
            1u16 => Self::Ug(TestGenerics__Ug::from_node(reader)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::Uv => 0u16,
            Self::Ug(value) => {
                value.update_node(writer);
                1u16
            }
            _ => {
                return;
            }
        };
        writer.write_u16(0u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestGenerics(pub TestGenerics_0, pub TestGenerics_1);
impl CapnpPlainStruct for TestGenerics {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestGenerics(
            TestGenerics_0::from_node(reader),
            TestGenerics_1::from_node(reader),
        )
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        self.0.update_node(writer);
        self.1.update_node(writer);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestGenerics__Ug {
    pub ugfoo: i32,
}
impl CapnpPlainStruct for TestGenerics__Ug {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestGenerics__Ug {
            ugfoo: reader.read_i32(1u32, 0i32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_i32(1u32, self.ugfoo, 0i32);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestGenerics__Inner {}
impl CapnpPlainStruct for TestGenerics__Inner {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestGenerics__Inner {}
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestGenerics__Inner2 {
    pub inner_bound: Option<Box<TestGenerics__Inner>>,
    pub inner_unbound: Option<Box<TestGenerics__Inner>>,
}
impl CapnpPlainStruct for TestGenerics__Inner2 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestGenerics__Inner2 {
            inner_bound: reader
                .read_struct(2u32)
                .map(|x| Box::new(TestGenerics__Inner::from_node(x))),
            inner_unbound: reader
                .read_struct(3u32)
                .map(|x| Box::new(TestGenerics__Inner::from_node(x))),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        if let Some(child) = &self.inner_bound {
            writer.write_child(2u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.inner_unbound {
            writer.write_child(3u32, CapnpNode::Struct(child.to_node()));
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestGenerics__Inner2__DeepNest {}
impl CapnpPlainStruct for TestGenerics__Inner2__DeepNest {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestGenerics__Inner2__DeepNest {}
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestGenerics__UseAliases {
    pub inner: Option<Box<TestGenerics__Inner>>,
    pub inner_2: Option<Box<TestGenerics__Inner2>>,
    pub inner_2_bind: Option<Box<TestGenerics__Inner2>>,
    pub inner_2_text: Option<Box<TestGenerics__Inner2>>,
}
impl CapnpPlainStruct for TestGenerics__UseAliases {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestGenerics__UseAliases {
            inner: reader
                .read_struct(1u32)
                .map(|x| Box::new(TestGenerics__Inner::from_node(x))),
            inner_2: reader
                .read_struct(2u32)
                .map(|x| Box::new(TestGenerics__Inner2::from_node(x))),
            inner_2_bind: reader
                .read_struct(3u32)
                .map(|x| Box::new(TestGenerics__Inner2::from_node(x))),
            inner_2_text: reader
                .read_struct(4u32)
                .map(|x| Box::new(TestGenerics__Inner2::from_node(x))),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        if let Some(child) = &self.inner {
            writer.write_child(1u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.inner_2 {
            writer.write_child(2u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.inner_2_bind {
            writer.write_child(3u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.inner_2_text {
            writer.write_child(4u32, CapnpNode::Struct(child.to_node()));
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoxedText {
    pub text: String,
}
impl CapnpPlainStruct for BoxedText {
    fn from_node(reader: &CapnpStructNode) -> Self {
        BoxedText {
            text: reader.read_text(0u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_text(0u32, &self.text);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestGenericsWrapper {
    pub value: Option<Box<TestGenerics>>,
}
impl CapnpPlainStruct for TestGenericsWrapper {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestGenericsWrapper {
            value: reader.read_struct(0u32).map(|x| Box::new(TestGenerics::from_node(x))),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        if let Some(child) = &self.value {
            writer.write_child(0u32, CapnpNode::Struct(child.to_node()));
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestGenericsWrapper2 {
    pub value: Option<Box<TestGenericsWrapper>>,
}
impl CapnpPlainStruct for TestGenericsWrapper2 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestGenericsWrapper2 {
            value: reader
                .read_struct(0u32)
                .map(|x| Box::new(TestGenericsWrapper::from_node(x))),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        if let Some(child) = &self.value {
            writer.write_child(0u32, CapnpNode::Struct(child.to_node()));
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestGenericsUnion {
    Foo(),
    Bar(),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestGenericsUnion {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(0u32, 0) {
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::Foo(..) => 0u16,
            Self::Bar(..) => 1u16,
            _ => {
                return;
            }
        };
        writer.write_u16(0u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestUseGenerics {
    pub basic: Option<Box<TestGenerics>>,
    pub inner: Option<Box<TestGenerics__Inner>>,
    pub inner_2: Option<Box<TestGenerics__Inner2>>,
    pub unspecified: Option<Box<TestGenerics>>,
    pub unspecified_inner: Option<Box<TestGenerics__Inner2>>,
    pub default: Option<Box<TestGenerics>>,
    pub default_inner: Option<Box<TestGenerics__Inner>>,
    pub default_user: Option<Box<TestUseGenerics>>,
    pub wrapper: Option<Box<TestGenericsWrapper>>,
    pub default_wrapper: Option<Box<TestGenericsWrapper>>,
    pub default_wrapper_2: Option<Box<TestGenericsWrapper2>>,
    pub alias_foo: Option<Box<TestAllTypes>>,
    pub alias_inner: Option<Box<TestGenerics__Inner>>,
    pub alias_inner_2: Option<Box<TestGenerics__Inner2>>,
    pub alias_inner_2_bind: Option<Box<TestGenerics__Inner2>>,
    pub alias_inner_2_text: Option<Box<TestGenerics__Inner2>>,
    pub alias_rev: String,
    pub use_aliases: Option<Box<TestGenerics__UseAliases>>,
    pub cap: Option<Box<TestGenerics>>,
    pub bind_enum_list: Option<Box<TestGenerics>>,
}
impl CapnpPlainStruct for TestUseGenerics {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestUseGenerics {
            basic: reader
                .read_struct(0u32)
                .map(|x| Box::new(TestGenerics::from_node(x))),
            inner: reader
                .read_struct(1u32)
                .map(|x| Box::new(TestGenerics__Inner::from_node(x))),
            inner_2: reader
                .read_struct(2u32)
                .map(|x| Box::new(TestGenerics__Inner2::from_node(x))),
            unspecified: reader
                .read_struct(3u32)
                .map(|x| Box::new(TestGenerics::from_node(x))),
            unspecified_inner: reader
                .read_struct(4u32)
                .map(|x| Box::new(TestGenerics__Inner2::from_node(x))),
            default: reader
                .read_struct(5u32)
                .map(|x| Box::new(TestGenerics::from_node(x))),
            default_inner: reader
                .read_struct(6u32)
                .map(|x| Box::new(TestGenerics__Inner::from_node(x))),
            default_user: reader
                .read_struct(7u32)
                .map(|x| Box::new(TestUseGenerics::from_node(x))),
            wrapper: reader
                .read_struct(8u32)
                .map(|x| Box::new(TestGenericsWrapper::from_node(x))),
            default_wrapper: reader
                .read_struct(9u32)
                .map(|x| Box::new(TestGenericsWrapper::from_node(x))),
            default_wrapper_2: reader
                .read_struct(10u32)
                .map(|x| Box::new(TestGenericsWrapper2::from_node(x))),
            alias_foo: reader
                .read_struct(11u32)
                .map(|x| Box::new(TestAllTypes::from_node(x))),
            alias_inner: reader
                .read_struct(12u32)
                .map(|x| Box::new(TestGenerics__Inner::from_node(x))),
            alias_inner_2: reader
                .read_struct(13u32)
                .map(|x| Box::new(TestGenerics__Inner2::from_node(x))),
            alias_inner_2_bind: reader
                .read_struct(14u32)
                .map(|x| Box::new(TestGenerics__Inner2::from_node(x))),
            alias_inner_2_text: reader
                .read_struct(15u32)
                .map(|x| Box::new(TestGenerics__Inner2::from_node(x))),
            alias_rev: reader.read_text(16u32),
            use_aliases: reader
                .read_struct(17u32)
                .map(|x| Box::new(TestGenerics__UseAliases::from_node(x))),
            cap: reader.read_struct(18u32).map(|x| Box::new(TestGenerics::from_node(x))),
            bind_enum_list: reader
                .read_struct(20u32)
                .map(|x| Box::new(TestGenerics::from_node(x))),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        if let Some(child) = &self.basic {
            writer.write_child(0u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.inner {
            writer.write_child(1u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.inner_2 {
            writer.write_child(2u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.unspecified {
            writer.write_child(3u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.unspecified_inner {
            writer.write_child(4u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.default {
            writer.write_child(5u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.default_inner {
            writer.write_child(6u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.default_user {
            writer.write_child(7u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.wrapper {
            writer.write_child(8u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.default_wrapper {
            writer.write_child(9u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.default_wrapper_2 {
            writer.write_child(10u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.alias_foo {
            writer.write_child(11u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.alias_inner {
            writer.write_child(12u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.alias_inner_2 {
            writer.write_child(13u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.alias_inner_2_bind {
            writer.write_child(14u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.alias_inner_2_text {
            writer.write_child(15u32, CapnpNode::Struct(child.to_node()));
        }
        writer.write_text(16u32, &self.alias_rev);
        if let Some(child) = &self.use_aliases {
            writer.write_child(17u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.cap {
            writer.write_child(18u32, CapnpNode::Struct(child.to_node()));
        }
        if let Some(child) = &self.bind_enum_list {
            writer.write_child(20u32, CapnpNode::Struct(child.to_node()));
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestEmptyStruct {}
impl CapnpPlainStruct for TestEmptyStruct {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestEmptyStruct {}
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestConstants {}
impl CapnpPlainStruct for TestConstants {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestConstants {}
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestAnyPointerConstants {}
impl CapnpPlainStruct for TestAnyPointerConstants {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestAnyPointerConstants {}
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestListOfAny {}
impl CapnpPlainStruct for TestListOfAny {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestListOfAny {}
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestPipeline__Box {}
impl CapnpPlainStruct for TestPipeline__Box {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestPipeline__Box {}
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestPipeline__AnyBox {}
impl CapnpPlainStruct for TestPipeline__AnyBox {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestPipeline__AnyBox {}
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestTailCallee__TailResult {
    pub i: u32,
    pub t: String,
}
impl CapnpPlainStruct for TestTailCallee__TailResult {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestTailCallee__TailResult {
            i: reader.read_u32(0u32, 0u32),
            t: reader.read_text(0u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u32(0u32, self.i, 0u32);
        writer.write_text(0u32, &self.t);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamResult {}
impl CapnpPlainStruct for StreamResult {
    fn from_node(reader: &CapnpStructNode) -> Self {
        StreamResult {}
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestMembrane__Result {
    pub text: String,
}
impl CapnpPlainStruct for TestMembrane__Result {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestMembrane__Result {
            text: reader.read_text(0u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_text(0u32, &self.text);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestContainMembrane {}
impl CapnpPlainStruct for TestContainMembrane {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestContainMembrane {}
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestTransferCap__Element {
    pub text: String,
}
impl CapnpPlainStruct for TestTransferCap__Element {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestTransferCap__Element {
            text: reader.read_text(0u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_text(0u32, &self.text);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestTransferCap {
    pub list: Vec<TestTransferCap__Element>,
}
impl CapnpPlainStruct for TestTransferCap {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestTransferCap {
            list: reader.read_list(0u32, |r| r.read_struct_children()),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer
            .write_child(
                0u32,
                CapnpNode::List(CapnpListNode::write_struct_children(&self.list)),
            );
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestSturdyRefHostId {
    pub host: String,
}
impl CapnpPlainStruct for TestSturdyRefHostId {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestSturdyRefHostId {
            host: reader.read_text(0u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_text(0u32, &self.host);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestSturdyRef {
    pub host_id: Option<Box<TestSturdyRefHostId>>,
}
impl CapnpPlainStruct for TestSturdyRef {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestSturdyRef {
            host_id: reader
                .read_struct(0u32)
                .map(|x| Box::new(TestSturdyRefHostId::from_node(x))),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        if let Some(child) = &self.host_id {
            writer.write_child(0u32, CapnpNode::Struct(child.to_node()));
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, Serialize, Deserialize)]
pub enum TestSturdyRefObjectId__Tag {
    TestInterface = 0isize,
    TestExtends = 1isize,
    TestPipeline = 2isize,
    TestTailCallee = 3isize,
    TestTailCaller = 4isize,
    TestMoreStuff = 5isize,
    UnknownEnumerant,
}
impl TestSturdyRefObjectId__Tag {
    pub fn decode(x: u16) -> Self {
        match x {
            0..=5u16 => Self::from_u16(x).unwrap(),
            _ => Self::UnknownEnumerant,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestSturdyRefObjectId {
    pub tag: TestSturdyRefObjectId__Tag,
}
impl CapnpPlainStruct for TestSturdyRefObjectId {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestSturdyRefObjectId {
            tag: TestSturdyRefObjectId__Tag::decode(reader.read_u16(0u32, 0u16)),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u16(0u32, self.tag as u16, 0u16);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestProvisionId {}
impl CapnpPlainStruct for TestProvisionId {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestProvisionId {}
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestRecipientId {}
impl CapnpPlainStruct for TestRecipientId {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestRecipientId {}
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestThirdPartyCapId {}
impl CapnpPlainStruct for TestThirdPartyCapId {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestThirdPartyCapId {}
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestJoinResult {}
impl CapnpPlainStruct for TestJoinResult {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestJoinResult {}
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestNameAnnotation__BadlyNamedUnion__BadlyNamedGroup {
    pub foo: (),
    pub bar: (),
}
impl CapnpPlainStruct for TestNameAnnotation__BadlyNamedUnion__BadlyNamedGroup {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestNameAnnotation__BadlyNamedUnion__BadlyNamedGroup {
            foo: (),
            bar: (),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestNameAnnotation__NestedStruct {
    pub bad_nested_field_name: bool,
    pub another_bad_nested_field_name: Option<Box<TestNameAnnotation__NestedStruct>>,
}
impl CapnpPlainStruct for TestNameAnnotation__NestedStruct {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestNameAnnotation__NestedStruct {
            bad_nested_field_name: reader.read_bool(0u32, false),
            another_bad_nested_field_name: reader
                .read_struct(0u32)
                .map(|x| Box::new(TestNameAnnotation__NestedStruct::from_node(x))),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_bool(0u32, self.bad_nested_field_name, false);
        if let Some(child) = &self.another_bad_nested_field_name {
            writer.write_child(0u32, CapnpNode::Struct(child.to_node()));
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestNameAnnotation_0 {
    pub another_bad_field_name: TestNameAnnotation__BadlyNamedEnum,
    pub badly_named_union: TestNameAnnotation__BadlyNamedUnion,
}
impl CapnpPlainStruct for TestNameAnnotation_0 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestNameAnnotation_0 {
            another_bad_field_name: TestNameAnnotation__BadlyNamedEnum::decode(
                reader.read_u16(2u32, 0u16),
            ),
            badly_named_union: TestNameAnnotation__BadlyNamedUnion::from_node(reader),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u16(2u32, self.another_bad_field_name as u16, 0u16);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestNameAnnotation_1 {
    BadFieldName(bool),
    Bar(i8),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestNameAnnotation_1 {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(1u32, 0) {
            0u16 => Self::BadFieldName(reader.read_bool(0u32, false)),
            1u16 => Self::Bar(reader.read_i8(0u32, 0i8)),
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::BadFieldName(value) => {
                writer.write_bool(0u32, *value, false);
                0u16
            }
            Self::Bar(value) => {
                writer.write_i8(0u32, *value, 0i8);
                1u16
            }
            _ => {
                return;
            }
        };
        writer.write_u16(1u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestNameAnnotation(pub TestNameAnnotation_0, pub TestNameAnnotation_1);
impl CapnpPlainStruct for TestNameAnnotation {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestNameAnnotation(
            TestNameAnnotation_0::from_node(reader),
            TestNameAnnotation_1::from_node(reader),
        )
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        self.0.update_node(writer);
        self.1.update_node(writer);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum TestNameAnnotation__BadlyNamedUnion {
    BadlyNamedGroup(TestNameAnnotation__BadlyNamedUnion__BadlyNamedGroup),
    Baz(TestNameAnnotation__NestedStruct),
    UnknownDiscriminant,
}
impl CapnpPlainStruct for TestNameAnnotation__BadlyNamedUnion {
    fn from_node(reader: &CapnpStructNode) -> Self {
        match reader.read_u16(3u32, 0) {
            0u16 => {
                Self::BadlyNamedGroup(
                    TestNameAnnotation__BadlyNamedUnion__BadlyNamedGroup::from_node(
                        reader,
                    ),
                )
            }
            1u16 => {
                Self::Baz(
                    TestNameAnnotation__NestedStruct::from_node(
                        reader.read_struct(0u32).unwrap(),
                    ),
                )
            }
            _ => Self::UnknownDiscriminant,
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        let discriminant_value = match self {
            Self::BadlyNamedGroup(value) => {
                value.update_node(writer);
                0u16
            }
            Self::Baz(value) => 1u16,
            _ => {
                return;
            }
        };
        writer.write_u16(3u32, discriminant_value, 0);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, Serialize, Deserialize)]
pub enum TestNameAnnotation__BadlyNamedEnum {
    Foo = 0isize,
    Bar = 1isize,
    Baz = 2isize,
    UnknownEnumerant,
}
impl TestNameAnnotation__BadlyNamedEnum {
    pub fn decode(x: u16) -> Self {
        match x {
            0..=2u16 => Self::from_u16(x).unwrap(),
            _ => Self::UnknownEnumerant,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, Serialize, Deserialize)]
pub enum TestNameAnnotation__NestedStruct__DeeplyNestedEnum {
    Quux = 0isize,
    Corge = 1isize,
    Grault = 2isize,
    UnknownEnumerant,
}
impl TestNameAnnotation__NestedStruct__DeeplyNestedEnum {
    pub fn decode(x: u16) -> Self {
        match x {
            0..=2u16 => Self::from_u16(x).unwrap(),
            _ => Self::UnknownEnumerant,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestImpliedFirstField {
    pub text_struct: Option<Box<TestImpliedFirstField__TextStruct>>,
    pub text_struct_list: Vec<TestImpliedFirstField__TextStruct>,
    pub int_group: TestImpliedFirstField__IntGroup,
}
impl CapnpPlainStruct for TestImpliedFirstField {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestImpliedFirstField {
            text_struct: reader
                .read_struct(0u32)
                .map(|x| Box::new(TestImpliedFirstField__TextStruct::from_node(x))),
            text_struct_list: reader.read_list(1u32, |r| r.read_struct_children()),
            int_group: TestImpliedFirstField__IntGroup::from_node(reader),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        if let Some(child) = &self.text_struct {
            writer.write_child(0u32, CapnpNode::Struct(child.to_node()));
        }
        writer
            .write_child(
                1u32,
                CapnpNode::List(
                    CapnpListNode::write_struct_children(&self.text_struct_list),
                ),
            );
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestImpliedFirstField__IntGroup {
    pub i: u32,
    pub str: String,
}
impl CapnpPlainStruct for TestImpliedFirstField__IntGroup {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestImpliedFirstField__IntGroup {
            i: reader.read_u32(0u32, 0u32),
            str: reader.read_text(2u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_u32(0u32, self.i, 0u32);
        writer.write_text(2u32, &self.str);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestImpliedFirstField__TextStruct {
    pub text: String,
    pub i: u32,
}
impl CapnpPlainStruct for TestImpliedFirstField__TextStruct {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestImpliedFirstField__TextStruct {
            text: reader.read_text(0u32),
            i: reader.read_u32(0u32, 321u32),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer.write_text(0u32, &self.text);
        writer.write_u32(0u32, self.i, 321u32);
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestCycleBNoCaps {
    pub foo: Vec<TestCycleANoCaps>,
    pub bar: Option<Box<TestAllTypes>>,
}
impl CapnpPlainStruct for TestCycleBNoCaps {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestCycleBNoCaps {
            foo: reader.read_list(0u32, |r| r.read_struct_children()),
            bar: reader.read_struct(1u32).map(|x| Box::new(TestAllTypes::from_node(x))),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer
            .write_child(
                0u32,
                CapnpNode::List(CapnpListNode::write_struct_children(&self.foo)),
            );
        if let Some(child) = &self.bar {
            writer.write_child(1u32, CapnpNode::Struct(child.to_node()));
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestCycleANoCaps {
    pub foo: Option<Box<TestCycleBNoCaps>>,
}
impl CapnpPlainStruct for TestCycleANoCaps {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestCycleANoCaps {
            foo: reader
                .read_struct(0u32)
                .map(|x| Box::new(TestCycleBNoCaps::from_node(x))),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        if let Some(child) = &self.foo {
            writer.write_child(0u32, CapnpNode::Struct(child.to_node()));
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestCycleBWithCaps {
    pub foo: Vec<TestCycleAWithCaps>,
}
impl CapnpPlainStruct for TestCycleBWithCaps {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestCycleBWithCaps {
            foo: reader.read_list(0u32, |r| r.read_struct_children()),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        writer
            .write_child(
                0u32,
                CapnpNode::List(CapnpListNode::write_struct_children(&self.foo)),
            );
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestCycleAWithCaps {
    pub foo: Option<Box<TestCycleBWithCaps>>,
}
impl CapnpPlainStruct for TestCycleAWithCaps {
    fn from_node(reader: &CapnpStructNode) -> Self {
        TestCycleAWithCaps {
            foo: reader
                .read_struct(0u32)
                .map(|x| Box::new(TestCycleBWithCaps::from_node(x))),
        }
    }
    fn update_node(&self, writer: &mut CapnpStructNode) {
        if let Some(child) = &self.foo {
            writer.write_child(0u32, CapnpNode::Struct(child.to_node()));
        }
    }
}
