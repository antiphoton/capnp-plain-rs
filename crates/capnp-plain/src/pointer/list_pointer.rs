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

use anyhow::{Error, Result};

use crate::message::word::Word;

use super::{read_offset_bits, write_offset_bits};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarSize {
    Void,
    OneBit,
    OneByte,
    TwoBytes,
    FourBytes,
    EightBytes,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ElementSize {
    Scalar(ScalarSize),
    Pointer,
    Composite,
}

#[derive(Debug)]
pub struct ListPointer {
    pub offset: isize,
    pub element_size: ElementSize,
    pub list_len: usize,
}

impl ListPointer {
    pub const TAG: u8 = 1;
}

impl TryFrom<Word> for ListPointer {
    type Error = Error;
    fn try_from(Word(a): Word) -> Result<Self, Self::Error> {
        let offset = read_offset_bits(Word(a), Self::TAG)?;
        let element_size = match a[4] % 8 {
            0 => ElementSize::Scalar(ScalarSize::Void),
            1 => ElementSize::Scalar(ScalarSize::OneBit),
            2 => ElementSize::Scalar(ScalarSize::OneByte),
            3 => ElementSize::Scalar(ScalarSize::TwoBytes),
            4 => ElementSize::Scalar(ScalarSize::FourBytes),
            5 => ElementSize::Scalar(ScalarSize::EightBytes),
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

impl From<ListPointer> for Word {
    fn from(input: ListPointer) -> Self {
        let ListPointer {
            offset,
            element_size,
            list_len,
        } = input;
        assert_eq!(offset << 2 >> 2, offset);
        let mut a = write_offset_bits(offset, ListPointer::TAG);
        let element_size = match element_size {
            ElementSize::Scalar(ScalarSize::Void) => 0,
            ElementSize::Scalar(ScalarSize::OneBit) => 1,
            ElementSize::Scalar(ScalarSize::OneByte) => 2,
            ElementSize::Scalar(ScalarSize::TwoBytes) => 3,
            ElementSize::Scalar(ScalarSize::FourBytes) => 4,
            ElementSize::Scalar(ScalarSize::EightBytes) => 5,
            ElementSize::Pointer => 6,
            ElementSize::Composite => 7,
        };
        let data = ((list_len as u32) << 3) + element_size;
        [a[4], a[5], a[6], a[7]] = data.to_le_bytes();
        Word(a)
    }
}
