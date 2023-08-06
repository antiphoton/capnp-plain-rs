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
                            result.push(data.get(i).unwrap().0[j] as $t);
                        }
                        result
                    }
                    _ => todo!(),
                }
            }
        }
    };
}

define_byte_reader!(read_i8_children, ScalarSize::OneByte, i8);
define_byte_reader!(read_u8_children, ScalarSize::OneByte, u8);

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
                    result.push((data.get(i).unwrap().0[j] >> k) % 2 == 1);
                }
                result
            }
            _ => todo!(),
        }
    }
}
