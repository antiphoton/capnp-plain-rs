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

use anyhow::{ensure, Result};

use crate::message::word::{word_ref::WordRef, Word};

use super::{get_offset_bits, LocalPointer};

pub struct FarPointer<'a> {
    double_landing: bool,
    offset: usize,
    segment_id: usize,
    word_ref: WordRef<'a>,
}

impl<'a> FarPointer<'a> {
    pub fn new(word_ref: WordRef<'a>) -> Result<Self> {
        let Word(a) = *word_ref;
        let offset = get_offset_bits(Word(a), 2)? as usize;
        let double_landing = offset % 2 == 1;
        let offset = offset >> 1;
        let segment_id = u32::from_le_bytes([a[4], a[5], a[6], a[7]]);
        let pointer = FarPointer {
            double_landing,
            offset,
            segment_id: segment_id as usize,
            word_ref,
        };
        Ok(pointer)
    }
    pub fn read(&self) -> Result<(LocalPointer, WordRef<'a>)> {
        let landing = self.word_ref.get_cousin(self.segment_id, self.offset);
        ensure!(landing.0[0] % 4 != 2);
        if self.double_landing {
            let tag_word = landing.get_next();
            let nested = FarPointer::new(landing)?;
            ensure!(nested.double_landing == false);
            let base = self
                .word_ref
                .get_cousin(nested.segment_id, nested.offset + 1);
            ensure!(tag_word.0[0] % 4 != 2);
            let local = LocalPointer::read(tag_word)?.0;
            Ok((local, base))
        } else {
            LocalPointer::read(landing)
        }
    }
}
