pub mod far_pointer;
pub mod list_pointer;
pub mod struct_pointer;

use anyhow::{ensure, Error, Result};

use crate::message::word::word_ref::WordRef;
use crate::message::word::Word;

use self::far_pointer::{read_far_pointer, FarPointer};
use self::list_pointer::ListPointer;
use self::struct_pointer::{StructPointer, StructReader};

pub enum Pointer {
    Struct(StructPointer),
    List(ListPointer),
    Far(FarPointer),
}

impl TryFrom<Word> for Pointer {
    type Error = Error;
    fn try_from(Word(a): Word) -> Result<Self, Self::Error> {
        let pointer = match a[0] % 4 {
            0 => Self::Struct(StructPointer::try_from(Word(a))?),
            1 => Self::List(ListPointer::try_from(Word(a))?),
            2 => Self::Far(FarPointer::try_from(Word(a))?),
            _ => todo!(),
        };
        Ok(pointer)
    }
}

pub enum Reader<'a> {
    Struct(StructReader<'a>),
}

impl<'a> Reader<'a> {
    pub fn new_local(pointer: Pointer, content_base: WordRef<'a>) -> Result<Self> {
        let reader = match pointer {
            Pointer::Struct(p) => Self::Struct(StructReader::new(p, content_base)?),
            _ => todo!(),
        };
        Ok(reader)
    }
    pub fn new(word_ref: WordRef<'a>) -> Result<Self> {
        let pointer = Pointer::try_from(*word_ref)?;
        match pointer {
            Pointer::Far(_) => read_far_pointer(word_ref),
            _ => Self::new_local(pointer, word_ref.get_next()),
        }
    }
    pub fn into_struct_reader(self) -> Result<StructReader<'a>> {
        match self {
            Self::Struct(x) => Ok(x),
        }
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
