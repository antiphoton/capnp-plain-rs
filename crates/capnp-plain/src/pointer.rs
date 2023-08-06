pub mod far_pointer;
pub mod list_pointer;
pub mod struct_pointer;

use anyhow::{ensure, Error, Result};

use crate::message::word::{word_ref::WordRef, Word};

use self::far_pointer::{read_far_pointer, FarPointer, FarPointerOld};
use self::list_pointer::{ListPointer, ListReader};
use self::struct_pointer::{StructPointer, StructReader};

pub enum LocalPointer {
    Struct(StructPointer),
    List(ListPointer),
}

pub enum PointerOld {
    Local(LocalPointer),
    Far(FarPointerOld),
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

impl TryFrom<Word> for PointerOld {
    type Error = Error;
    fn try_from(Word(a): Word) -> Result<Self, Self::Error> {
        let pointer = match a[0] % 4 {
            0 => Self::Local(LocalPointer::Struct(StructPointer::try_from(Word(a))?)),
            1 => Self::Local(LocalPointer::List(ListPointer::try_from(Word(a))?)),
            2 => Self::Far(FarPointerOld::try_from(Word(a))?),
            _ => todo!(),
        };
        Ok(pointer)
    }
}

pub enum Reader<'a> {
    Struct(StructReader<'a>),
    List(ListReader<'a>),
}

impl<'a> Reader<'a> {
    pub fn new_local(pointer: LocalPointer, content_base: WordRef<'a>) -> Result<Self> {
        let reader = match pointer {
            LocalPointer::Struct(p) => Self::Struct(StructReader::new(p, content_base)?),
            LocalPointer::List(p) => Self::List(ListReader::new(p, content_base)?),
        };
        Ok(reader)
    }
    pub fn new(word_ref: WordRef<'a>) -> Result<Self> {
        let pointer = PointerOld::try_from(*word_ref)?;
        match pointer {
            PointerOld::Far(_) => read_far_pointer(word_ref),
            PointerOld::Local(local) => Self::new_local(local, word_ref.get_next()),
        }
    }
    pub fn into_struct_reader(self) -> Result<StructReader<'a>> {
        match self {
            Self::Struct(x) => Ok(x),
            _ => Err(Error::msg("not a struct pointer")),
        }
    }
    pub fn into_list_reader(self) -> Result<ListReader<'a>> {
        match self {
            Self::List(x) => Ok(x),
            _ => Err(Error::msg("not a list pointer")),
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
