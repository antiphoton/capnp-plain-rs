pub mod far_pointer;
pub mod list_pointer;
pub mod struct_pointer;

use anyhow::{ensure, Result};

use crate::message::word::{word_ref::WordRef, Word};

use self::far_pointer::FarPointer;
use self::list_pointer::ListPointer;
use self::struct_pointer::StructPointer;

pub enum LocalPointer {
    Struct(StructPointer),
    List(ListPointer),
}

impl LocalPointer {
    pub fn read(word_ref: WordRef) -> Result<(LocalPointer, WordRef)> {
        let y = match word_ref.0[0] % 4 {
            0 => {
                let p = StructPointer::try_from(*word_ref)?;
                (LocalPointer::Struct(p), word_ref.get_next())
            }
            1 => {
                let p = ListPointer::try_from(*word_ref)?;
                (LocalPointer::List(p), word_ref.get_next())
            }
            2 => {
                let far_pointer = FarPointer::new(word_ref)?;
                far_pointer.read()?
            }
            _ => todo!(),
        };
        Ok(y)
    }
}

pub fn get_offset_bits(Word(a): Word, asserted_low_bits: u8) -> Result<isize> {
    ensure!(a[0] % 4 == asserted_low_bits);
    let offset = i32::from_le_bytes([a[0], a[1], a[2], a[3]]) / 4;
    Ok(offset as isize)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            get_offset_bits(Word([10, 31, 0, 0, 0, 0, 0, 0]), 2).unwrap(),
            1986
        );
    }
}
