use anyhow::{ensure, Result};

use super::{ListReader, ScalarSize};

macro_rules! define_byte_reader {
    ($name:ident, $s:expr, $t:ty) => {
        impl<'a> ListReader<'a> {
            pub fn $name(&self) -> Result<Vec<$t>> {
                match self {
                    Self::Scalar {
                        scalar_size,
                        list_len,
                        data,
                    } => {
                        ensure!(*scalar_size == $s);
                        let mut result = Vec::with_capacity(*list_len);
                        for index in 0..*list_len {
                            let i = index / 8;
                            let j = index % 8;
                            result.push(data.get(i).unwrap().0[j] as $t);
                        }
                        Ok(result)
                    }
                    _ => todo!(),
                }
            }
        }
    };
}

define_byte_reader!(read_i8_children, ScalarSize::OneByte, i8);
define_byte_reader!(read_u8_children, ScalarSize::OneByte, u8);

impl<'a> ListReader<'a> {
    pub fn read_text(&self) -> Result<String> {
        let mut bytes = self.read_u8_children()?;
        let terminator = bytes.pop();
        ensure!(terminator == Some(0));
        let s = String::from_utf8(bytes)?;
        Ok(s)
    }
}
