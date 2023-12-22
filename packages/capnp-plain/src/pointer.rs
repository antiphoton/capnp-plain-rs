pub mod far_pointer;
pub mod list_pointer;
pub mod struct_pointer;

use anyhow::{ensure, Result};

use crate::message::word::{word_ref::WordRef, Word};

use self::far_pointer::FarPointer;
use self::list_pointer::ListPointer;
use self::struct_pointer::StructPointer;

#[derive(Debug)]
pub enum LocalPointer {
    Struct(StructPointer),
    List(ListPointer),
}

impl LocalPointer {
    pub fn read(word_ref: WordRef) -> Result<(LocalPointer, WordRef)> {
        let y = match word_ref.0[0] % 4 {
            StructPointer::TAG => {
                let p = StructPointer::try_from(*word_ref)?;
                (LocalPointer::Struct(p), word_ref.get_next())
            }
            ListPointer::TAG => {
                let p = ListPointer::try_from(*word_ref)?;
                (LocalPointer::List(p), word_ref.get_next())
            }
            FarPointer::TAG => {
                let far_pointer = FarPointer::new(word_ref)?;
                far_pointer.read()?
            }
            3 => unimplemented!(),
            _ => unreachable!(),
        };
        Ok(y)
    }
}

pub fn read_offset_bits(Word(a): Word, asserted_low_bits: u8) -> Result<isize> {
    ensure!(a[0] % 4 == asserted_low_bits);
    let offset = i32::from_le_bytes([a[0], a[1], a[2], a[3]]) >> 2;
    Ok(offset as isize)
}

pub fn write_offset_bits(offset: isize, low_bit_tag: u8) -> [u8; 8] {
    assert!(low_bit_tag < 4);
    let offset = offset as i32;
    let mut a = [0; 8];
    [a[0], a[1], a[2], a[3]] = (offset << 2).to_le_bytes();
    a[0] += low_bit_tag;
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            read_offset_bits(Word([10, 31, 0, 0, 0, 0, 0, 0]), 2).unwrap(),
            1986
        );
    }
    #[test]
    fn negative_offset() {
        assert_eq!(
            read_offset_bits(Word([254, 255, 255, 255, 0, 0, 0, 0]), 2).unwrap(),
            -1
        );
        assert_eq!(
            read_offset_bits(Word([250, 255, 255, 255, 0, 0, 0, 0]), 2).unwrap(),
            -2
        );
        assert_eq!(
            read_offset_bits(Word([246, 255, 255, 255, 0, 0, 0, 0]), 2).unwrap(),
            -3
        );
    }
}
