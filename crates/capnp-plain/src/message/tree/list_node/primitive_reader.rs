use super::{ListNode, ScalarSize};

macro_rules! define_byte_reader {
    ($name:ident, $s:expr, $t:ty) => {
        impl ListNode {
            pub fn $name(&self) -> Vec<$t> {
                match self {
                    Self::Scalar {
                        scalar_size,
                        list_len,
                        data,
                    } => {
                        if *scalar_size != $s {
                            return Vec::with_capacity(0);
                        }
                        let mut result = Vec::with_capacity(*list_len);
                        for index in 0..*list_len {
                            let i = index / 8;
                            let j = index % 8;
                            if let Some(word) = data.get(i) {
                                result.push(word.0[j] as $t);
                            } else {
                                result.push(Default::default());
                            }
                        }
                        result
                    }
                    Self::Composite(_) => vec![],
                }
            }
        }
    };
}

define_byte_reader!(read_i8_children, ScalarSize::OneByte, i8);
define_byte_reader!(read_u8_children, ScalarSize::OneByte, u8);
define_byte_reader!(read_i16_children, ScalarSize::TwoBytes, i16);
define_byte_reader!(read_u16_children, ScalarSize::TwoBytes, u16);
define_byte_reader!(read_i32_children, ScalarSize::FourBytes, i32);
define_byte_reader!(read_u32_children, ScalarSize::FourBytes, u32);
define_byte_reader!(read_i64_children, ScalarSize::EightBytes, i64);
define_byte_reader!(read_u64_children, ScalarSize::EightBytes, u64);
define_byte_reader!(read_f32_children, ScalarSize::EightBytes, f32);
define_byte_reader!(read_f64_children, ScalarSize::EightBytes, f64);

impl ListNode {
    pub fn read_bool_children(&self) -> Vec<bool> {
        match self {
            Self::Scalar {
                scalar_size,
                list_len,
                data,
            } => {
                if *scalar_size != ScalarSize::OneBit {
                    return Vec::with_capacity(0);
                }
                let mut result = Vec::with_capacity(*list_len);
                for index in 0..*list_len {
                    let i = index / 64;
                    let j = (index % 64) / 8;
                    let k = index % 8;
                    if let Some(word) = data.get(i) {
                        result.push((word.0[j] >> k) % 2 == 1);
                    } else {
                        result.push(false);
                    }
                }
                result
            }
            _ => vec![],
        }
    }
}
