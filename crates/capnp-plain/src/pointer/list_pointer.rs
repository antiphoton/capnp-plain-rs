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

use anyhow::{ensure, Error, Result};

use crate::message::word::{word_ref::WordRef, word_slice::WordSlice, Word};

use super::{
    get_offset_bits,
    struct_pointer::{StructPointer, StructReader},
};

#[derive(Debug)]
pub enum ScalarSize {
    Void,
    OneBit,
    OneByte,
    TwoBytes,
    FourBytes,
    EightBytes,
}

#[derive(Debug)]
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

impl TryFrom<Word> for ListPointer {
    type Error = Error;
    fn try_from(Word(a): Word) -> Result<Self, Self::Error> {
        let offset = get_offset_bits(Word(a), 1)?;
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

pub enum ListReader<'a> {
    Scalar {
        element_size: ScalarSize,
        list_len: usize,
        data: WordSlice<'a>,
    },
    Composite {
        count: usize,
        tag: StructPointer,
        data: WordSlice<'a>,
    },
}

impl<'a> ListReader<'a> {
    pub fn new(pointer: ListPointer, content_base: WordRef<'a>) -> Result<Self> {
        let ListPointer {
            offset,
            element_size,
            list_len,
        } = pointer;
        let reader = match element_size {
            ElementSize::Composite => {
                let mut tag =
                    StructPointer::try_from(*content_base.get_sibling(offset, 1).get(0).unwrap())?;
                let count = tag.offset as usize;
                ensure!((tag.data_size + tag.pointer_size) * count <= list_len);
                tag.offset = 0;
                Self::Composite {
                    count,
                    tag,
                    data: content_base.get_sibling(offset + 1, list_len),
                }
            }
            _ => todo!(),
        };
        Ok(reader)
    }
    pub fn len(&self) -> usize {
        match self {
            Self::Composite { count, .. } => *count,
            Self::Scalar { list_len, .. } => *list_len,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() > 0
    }
    pub fn read_struct_child(&self, index: usize) -> Result<StructReader<'_>> {
        match self {
            Self::Composite { count, tag, data } => {
                ensure!(index < count);
                let element_size = tag.data_size + tag.pointer_size;
                StructReader::new(tag.clone(), data.get(element_size * index).unwrap())
            }
            _ => todo!(),
        }
    }
}
