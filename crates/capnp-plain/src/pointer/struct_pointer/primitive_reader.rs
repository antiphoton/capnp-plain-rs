use super::StructReader;

macro_rules! define_byte_reader {
    ($name:ident, $t:ty) => {
        impl<'a> StructReader<'a> {
            pub fn $name(&self, offset: u32, default_value: $t) -> $t {
                let byte_offset = std::mem::size_of::<$t>() * offset as usize;
                let i = byte_offset / 8;
                let j = byte_offset % 8;
                if let Some(word) = self.data.get(i) {
                    word.0[j] as $t ^ default_value
                } else {
                    default_value
                }
            }
        }
    };
}

macro_rules! define_small_reader {
    ($name:ident, $t:ty, $($i:expr),+) => {
      impl<'a> StructReader<'a> {
        pub fn $name(&self, offset: u32, default_value: $t) -> $t {
            let byte_offset = std::mem::size_of::<$t>() * offset as usize;
            let i = byte_offset / 8;
            let j = byte_offset % 8;
            let Some(word) = self.data.get(i) else {
                return default_value;
            };
            let default_value = default_value.to_le_bytes();
            let value = [$(word.0[j + $i] ^ default_value[$i],)+];
            <$t>::from_le_bytes(value)
        }
      }
    };
}

macro_rules! define_big_reader {
    ($name:ident, $t:ty) => {
        impl<'a> StructReader<'a> {
            pub fn $name(&self, offset: u32, default_value: $t) -> $t {
                if let Some(word) = self.data.get(offset as usize) {
                    let value = <$t>::from_le_bytes(word.0);
                    <$t>::from_ne_bytes(
                        (<u64>::from_ne_bytes(value.to_ne_bytes())
                            ^ <u64>::from_ne_bytes(default_value.to_ne_bytes()))
                        .to_ne_bytes(),
                    )
                } else {
                    default_value
                }
            }
        }
    };
}

impl<'a> StructReader<'a> {
    pub fn read_bool(&self, offset: u32, default_value: bool) -> bool {
        let offset = offset as usize;
        let i = offset / 64;
        let Some(word) = self.data.get(i)  else {
            return default_value;
        };
        let j = (offset % 64) / 8;
        let k = offset % 8;
        let byte = word.0[j];
        let bit = (byte >> k) % 2 == 1;
        bit ^ default_value
    }
}

define_byte_reader!(read_i8, i8);
define_byte_reader!(read_u8, u8);

define_small_reader!(read_i16, i16, 0, 1);
define_small_reader!(read_u16, u16, 0, 1);
define_small_reader!(read_i32, i32, 0, 1, 2, 3);
define_small_reader!(read_u32, u32, 0, 1, 2, 3);
define_small_reader!(read_f32, f32, 0, 1, 2, 3);

define_big_reader!(read_i64, i64);
define_big_reader!(read_u64, u64);
define_big_reader!(read_f64, f64);
