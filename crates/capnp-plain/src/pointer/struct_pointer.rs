//! ```text
//! lsb                      struct pointer                       msb
//! +-+-----------------------------+---------------+---------------+
//! |A|             B               |       C       |       D       |
//! +-+-----------------------------+---------------+---------------+
//!
//! A (2 bits) = 0, to indicate that this is a struct pointer.
//! B (30 bits) = Offset, in words, from the end of the pointer to the
//!     start of the struct's data section.  Signed.
//! C (16 bits) = Size of the struct's data section, in words.
//! D (16 bits) = Size of the struct's pointer section, in words.
//! ```

mod primitive_reader;

use anyhow::{Error, Result};

use crate::{
    message::word::{word_ref::WordRef, word_slice::WordSlice, Word},
    pointer::get_offset_bits,
};

use super::{list_pointer::ListReader, Reader};

#[derive(Clone)]
pub struct StructPointer {
    pub offset: isize,
    pub data_size: usize,
    pub pointer_size: usize,
}

impl TryFrom<Word> for StructPointer {
    type Error = Error;
    fn try_from(Word(a): Word) -> Result<Self, Self::Error> {
        let offset = get_offset_bits(Word(a), 0)?;
        let data_size = u16::from_le_bytes([a[4], a[5]]);
        let pointer_size = u16::from_le_bytes([a[6], a[7]]);
        let pointer = StructPointer {
            offset,
            data_size: data_size as usize,
            pointer_size: pointer_size as usize,
        };
        Ok(pointer)
    }
}

pub struct StructReader<'a> {
    data: WordSlice<'a>,
    pointers: WordSlice<'a>,
}

impl<'a> StructReader<'a> {
    pub fn new(pointer: StructPointer, content_base: WordRef<'a>) -> Result<Self> {
        let StructPointer {
            offset,
            data_size,
            pointer_size,
        } = pointer;
        let data = content_base.get_sibling(offset, data_size);
        let pointers = content_base.get_sibling(offset + data_size as isize, pointer_size);
        let reader = StructReader { data, pointers };
        Ok(reader)
    }
    pub fn clone_ref(&self) -> Self {
        Self {
            data: self.data.clone_ref(),
            pointers: self.pointers.clone_ref(),
        }
    }
    pub fn read_pointer(&self, offset: u32) -> Result<Reader<'_>> {
        let x = self
            .pointers
            .get(offset as usize)
            .ok_or_else(|| Error::msg("out of bound"))?;
        Reader::new(x)
    }
    pub fn read_struct_child<T: CapnpPlainStruct>(&self, offset: u32) -> Result<T> {
        let reader = self.read_pointer(offset)?.into_struct_reader()?;
        T::try_from_reader(reader)
    }
    pub fn read_text_field(&self, offset: u32) -> String {
        let mut bytes = self.read_list_field(offset, |r| r.read_u8_children());
        let terminator = bytes.pop();
        if terminator != Some(0) {
            return "".to_string();
        }
        String::from_utf8(bytes).unwrap_or_default()
    }
    pub fn read_list_field<T>(
        &self,
        offset: u32,
        f: impl FnOnce(ListReader) -> Result<Vec<T>>,
    ) -> Vec<T> {
        let Ok(p) = self.read_pointer(offset) else {
            return Vec::with_capacity(0);
        };
        let Ok(p) = p.into_list_reader() else {
            return Vec::with_capacity(0);
        };
        f(p).unwrap_or_else(|_| Vec::with_capacity(0))
    }
}

pub trait CapnpPlainStruct: Sized {
    fn try_from_reader(reader: StructReader) -> Result<Self>;
}
