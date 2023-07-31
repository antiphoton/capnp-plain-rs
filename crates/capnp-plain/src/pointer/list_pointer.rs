//! ```text
//! +-+-----------------------------+--+----------------------------+
//! |A|             B               |C |             D              |
//! +-+-----------------------------+--+----------------------------+
//!
//! A (2 bits) = 1, to indicate that this is a list pointer.
//! B (30 bits) = Offset, in words, from the end of the pointer to the
//!     start of the first element of the list.  Signed.
//! C (3 bits) = Size of each element:
//!     0 = 0 (e.g. List(Void))
//!     1 = 1 bit
//!     2 = 1 byte
//!     3 = 2 bytes
//!     4 = 4 bytes
//!     5 = 8 bytes (non-pointer)
//!     6 = 8 bytes (pointer)
//!     7 = composite (see below)
//! D (29 bits) = Size of the list:
//!     when C <> 7: Number of elements in the list.
//!     when C = 7: Number of words in the list, not counting the tag word
//!     (see below).

use anyhow::Error;

use crate::message::word::Word;

use super::get_offset_bits;

pub enum ElementSize {
    Void,
    OneBit,
    OneByte,
    TwoBytes,
    FourBytes,
    EightBytes,
    Pointer,
    Composite,
}

pub struct ListPointer {
    pub offset: isize,
    pub element_size: ElementSize,
    pub list_len: usize,
}

impl TryFrom<Word> for ListPointer {
    type Error = Error;
    fn try_from(Word(a): Word) -> Result<Self, Self::Error> {
        let offset = get_offset_bits(Word(a), 1)?;
        let element_size = match a[4] % 8 {
            0 => ElementSize::Void,
            1 => ElementSize::OneBit,
            2 => ElementSize::OneByte,
            3 => ElementSize::TwoBytes,
            4 => ElementSize::FourBytes,
            5 => ElementSize::EightBytes,
            6 => ElementSize::Pointer,
            7 => ElementSize::Composite,
            _ => unreachable!(),
        };
        let list_len = u32::from_le_bytes([a[4], a[5], a[6], a[7]]) / 8;
        let pointer = ListPointer {
            offset,
            element_size,
            list_len: list_len as usize,
        };
        Ok(pointer)
    }
}
