//! ```text
//! lsb                        far pointer                        msb
//! +-+-+---------------------------+-------------------------------+
//! |A|B|            C              |               D               |
//! +-+-+---------------------------+-------------------------------+
//!
//! A (2 bits) = 2, to indicate that this is a far pointer.
//! B (1 bit) = 0 if the landing pad is one word, 1 if it is two words.
//!     See explanation below.
//! C (29 bits) = Offset, in words, from the start of the target segment
//!     to the location of the far-pointer landing-pad within that
//!     segment.  Unsigned.
//! D (32 bits) = ID of the target segment.  (Segments are numbered
//!     sequentially starting from zero.)
//! ```

use anyhow::{ensure, Error, Result};

use crate::{
    message::word::{word_ref::WordRef, Word},
    pointer::Pointer,
};

use super::{get_offset_bits, Reader};

#[derive(Debug)]
pub struct FarPointer {
    double_landing: bool,
    offset: usize,
    segment_id: usize,
}

impl TryFrom<Word> for FarPointer {
    type Error = Error;
    fn try_from(Word(a): Word) -> Result<Self, Self::Error> {
        let offset = get_offset_bits(Word(a), 2)? as usize;
        let double_landing = offset % 2 == 1;
        let offset = offset / 2;
        let segment_id = u32::from_le_bytes([a[4], a[5], a[6], a[7]]);
        let pointer = FarPointer {
            double_landing,
            offset,
            segment_id: segment_id as usize,
        };
        Ok(pointer)
    }
}

pub fn read_far_pointer(word_ref: WordRef) -> Result<Reader> {
    let FarPointer {
        double_landing,
        offset,
        segment_id,
    } = FarPointer::try_from(*word_ref)?;
    word_ref.dump(0);
    let landing = word_ref.get_cousin(segment_id, offset);
    landing.dump(0);
    ensure!(landing.0[0] % 4 != 2);
    if double_landing {
        let nested = FarPointer::try_from(*landing)?;
        ensure!(nested.double_landing == false);
        let base = word_ref.get_cousin(nested.segment_id, nested.offset + 1);
        let tag_word = landing.get_sibling(1, 1);
        let pointer = Pointer::try_from(*tag_word.get(0).unwrap())?;
        Reader::new_local(pointer, base)
    } else {
        let pointer = Pointer::try_from(*landing)?;
        Reader::new_local(pointer, landing.get_next())
    }
}
