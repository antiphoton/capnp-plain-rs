pub mod far_pointer;
pub mod list_pointer;
pub mod struct_pointer;

use anyhow::{ensure, Result, Error};

use crate::message::word::word_ref::WordRef;
use crate::message::word::Word;

use self::far_pointer::read_far_pointer;
use self::list_pointer::ListPointer;
use self::struct_pointer::{StructPointer, StructReader};

pub enum Pointer {
    Struct(StructPointer),
    List(ListPointer),
}

impl TryFrom<Word> for Pointer {
    type Error = Error;
    fn try_from(Word(a): Word) -> Result<Self, Self::Error> {
        let pointer = match a[0] % 4 {
            0 => Self::Struct(StructPointer::try_from(Word(a))?),
            1 => Self::List(ListPointer::try_from(Word(a))?),
            _ => todo!(),
        };
        Ok(pointer)
    }
}

pub enum Reader<'a> {
    Struct(StructReader<'a>),
}

impl<'a> Reader<'a> {
    pub fn new_local(base: WordRef<'a>, pointer: Pointer) -> Result<Self> {
        let reader = match pointer {

            Pointer::Struct(p) => Self::Struct(StructReader::new(base, p)?),
            _ => todo!(),
        };
        Ok(reader)
    }
    pub fn new(word_ref: WordRef<'a>) -> Result<Self> {
        if word_ref.0[0] % 4 == 2 {
            todo!()
        } else {
            Self::new_local(word_ref, pointer)
        }
    }
    pub fn into_struct_reader(self) -> Result<StructReader<'a>> {
        match self {
            Self::Struct(x) => Ok(x),
        }
    }
}

pub fn get_offset_bits(Word(a): Word, asserted_low_bits: u8) -> Result<isize> {
    ensure!(a[0] == asserted_low_bits);
    let offset = i32::from_le_bytes([a[0], a[1], a[2], a[3]]) / 4;
    Ok(offset as isize)
}
