use crate::message::{
    tree::{list_node::ListNode, Node},
    word::Word,
};

use super::StructNode;

macro_rules! define_byte_writer {
    ($name:ident, $t:ty) => {
        impl StructNode {
            pub fn $name(&mut self, offset: u32, value: $t, default_value: $t) {
                let byte_offset = std::mem::size_of::<$t>() * offset as usize;
                let i = byte_offset / 8;
                let j = byte_offset % 8;
                let word = self.extend_and_get(i);
                word.0[j] = (value ^ default_value) as u8;
            }
        }
    };
}

macro_rules! define_small_writer {
    ($name:ident, $t:ty, $($i:expr),+) => {
      impl StructNode {
        pub fn $name(&mut self, offset: u32, value: $t, default_value: $t) {
            let byte_offset = std::mem::size_of::<$t>() * offset as usize;
            let i = byte_offset / 8;
            let j = byte_offset % 8;
            let word = self.extend_and_get(i);
            let value = value.to_le_bytes();
            let default_value = default_value.to_le_bytes();
            $(word.0[j + $i] = value[$i] ^ default_value[$i];)+
        }
      }
    };
}

macro_rules! define_big_writer {
    ($name:ident, $t:ty) => {
        impl StructNode {
            pub fn $name(&mut self, offset: u32, value: $t, default_value: $t) {
                let word = self.extend_and_get(offset as usize);
                let value = <u64>::from_ne_bytes(value.to_ne_bytes());
                let default_value = <u64>::from_ne_bytes(default_value.to_ne_bytes());
                word.0 = (value ^ default_value).to_le_bytes();
            }
        }
    };
}

impl StructNode {
    fn extend_and_get(&mut self, i: usize) -> &mut Word {
        self.data.resize_with(i + 1, Default::default);
        self.data.last_mut().unwrap()
    }
    pub fn write_bool(&mut self, offset: u32, value: bool, default_value: bool) {
        if value == default_value {
            return;
        }
        let offset = offset as usize;
        let i = offset / 64;
        let word = self.extend_and_get(i);
        let j = (offset % 64) / 8;
        let k = offset % 8;
        word.0[j] ^= 1 << k;
    }
}

define_byte_writer!(write_i8, i8);
define_byte_writer!(write_u8, u8);

define_small_writer!(write_i16, i16, 0, 1);
define_small_writer!(write_u16, u16, 0, 1);
define_small_writer!(write_i32, i32, 0, 1, 2, 3);
define_small_writer!(write_u32, u32, 0, 1, 2, 3);
define_small_writer!(write_f32, f32, 0, 1, 2, 3);

define_big_writer!(write_i64, i64);
define_big_writer!(write_u64, u64);
define_big_writer!(write_f64, f64);

impl StructNode {
    pub fn write_text(&mut self, offset: u32, data: &str) {
        let data = data.as_bytes();
        let mut child = ListNode::write_u8_children(data);
        match &mut child {
            ListNode::Scalar { list_len, .. } => {
                *list_len += 1;
            }
            _ => unreachable!(),
        }
        self.write_child(offset, Node::List(child));
    }
}
