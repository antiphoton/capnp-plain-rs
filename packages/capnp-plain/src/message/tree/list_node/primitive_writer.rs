use crate::message::word::Word;

use super::{ListNode, ScalarSize};

macro_rules! define_byte_writer {
    ($name:ident, $s:expr, $t:ty) => {
        impl ListNode {
            pub fn $name(input: &[$t]) -> Self {
                let list_len = input.len();
                let mut output = [Word::default()].repeat((list_len + 7) / 8);
                for index in 0..list_len {
                    let i = index / 8;
                    let j = index % 8;
                    output[i].0[j] = input[index] as u8;
                }
                Self::Scalar {
                    scalar_size: $s,
                    list_len,
                    data: output,
                }
            }
        }
    };
}

define_byte_writer!(write_i8_children, ScalarSize::OneByte, i8);
define_byte_writer!(write_u8_children, ScalarSize::OneByte, u8);

impl ListNode {
    pub fn write_bool_children(input: Vec<bool>) -> Self {
        let list_len = input.len();
        let mut output = [Word::default()].repeat((list_len + 63) / 64);
        for (index, input_item) in input.iter().enumerate() {
            let i = index / 64;
            let j = (index % 64) / 8;
            let k = index % 8;
            if *input_item {
                output[i].0[j] |= 1 << k;
            }
        }
        Self::Scalar {
            scalar_size: ScalarSize::OneBit,
            list_len,
            data: output,
        }
    }
}
