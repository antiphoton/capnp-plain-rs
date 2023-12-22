use crate::{message::word::Word, CapnpPlainEnum};

use super::{ListNode, ScalarSize};

impl ListNode {
    fn as_scalar(&self, expected_scalar_size: ScalarSize) -> Option<(usize, &[Word])> {
        match self {
            Self::Scalar {
                scalar_size,
                list_len,
                data,
            } => {
                if *scalar_size == expected_scalar_size {
                    Some((*list_len, data))
                } else {
                    None
                }
            }
            Self::Composite(_) => None,
        }
    }
}

macro_rules! define_byte_reader {
    ($name:ident, $t:ty) => {
        impl ListNode {
            pub fn $name(&self) -> Vec<$t> {
                let Some((list_len, data)) = self.as_scalar(ScalarSize::OneByte) else {
                    return Vec::with_capacity(0);
                };
                let mut result = Vec::with_capacity(list_len);
                for index in 0..list_len {
                    let i = index / 8;
                    let j = index % 8;
                    let value = if let Some(word) = data.get(i) {
                        word.0[j] as $t
                    } else {
                        Default::default()
                    };
                    result.push(value);
                }
                result
            }
        }
    };
}

macro_rules! define_small_reader {
    ($name:ident, $s:expr, $t:ty, $($i:expr),+) => {
        impl ListNode {
            pub fn $name(&self) -> Vec<$t> {
                let Some((list_len, data)) = self.as_scalar($s) else {
                    return Vec::with_capacity(0);
                };
                let mut result = Vec::with_capacity(list_len);
                for index in 0..list_len {
                    let byte_offset = std::mem::size_of::<$t>() * index as usize;
                    let i = byte_offset / 8;
                    let j = byte_offset % 8;
                    let value = if let Some(word) = data.get(i) {
                        <$t>::from_le_bytes([$(word.0[j + $i],)+])
                    } else {
                        Default::default()
                    };
                    result.push(value)
                }
                result
            }
        }
    };
}

macro_rules! define_big_reader {
    ($name:ident, $t:ty) => {
        impl ListNode {
            pub fn $name(&self) -> Vec<$t> {
                let Some((list_len, data)) = self.as_scalar(ScalarSize::EightBytes) else {
                    return Vec::with_capacity(0);
                };
                let mut result = Vec::with_capacity(list_len);
                for index in 0..list_len {
                    let Some(word) = data.get(index) else {
                        result.push(Default::default());
                        continue;
                    };
                    result.push(<$t>::from_le_bytes(word.0))
                }
                result
            }
        }
    };
}

define_byte_reader!(read_i8_children, i8);
define_byte_reader!(read_u8_children, u8);

define_small_reader!(read_i16_children, ScalarSize::TwoBytes, i16, 0, 1);
define_small_reader!(read_u16_children, ScalarSize::TwoBytes, u16, 0, 1);
define_small_reader!(read_i32_children, ScalarSize::FourBytes, i32, 0, 1, 2, 3);
define_small_reader!(read_u32_children, ScalarSize::FourBytes, u32, 0, 1, 2, 3);
define_small_reader!(read_f32_children, ScalarSize::FourBytes, f32, 0, 1, 2, 3);

define_big_reader!(read_i64_children, i64);
define_big_reader!(read_u64_children, u64);
define_big_reader!(read_f64_children, f64);

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
    pub fn read_enum_children<T: CapnpPlainEnum>(&self) -> Vec<T> {
        self.read_u16_children()
            .into_iter()
            .map(T::decode)
            .collect()
    }
}
