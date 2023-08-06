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

use anyhow::{Error, Result};

use crate::{message::word::Word, pointer::get_offset_bits};

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
