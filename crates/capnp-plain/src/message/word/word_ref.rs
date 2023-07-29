use std::ops::Deref;

use crate::message::Message;

use super::{word_slice::WordSlice, Word};

pub struct WordRef<'a> {
    message: &'a Message,
    segment_id: usize,
    offset: usize,
}

impl<'a> Deref for WordRef<'a> {
    type Target = Word;
    fn deref(&self) -> &Self::Target {
        let segment = &self.message.segments[self.segment_id];
        &segment.words[self.offset]
    }
}

impl<'a> WordRef<'a> {
    pub fn new(message: &'a Message, segment_id: usize, offset: usize) -> Self {
        WordRef {
            message,
            segment_id,
            offset,
        }
    }
    pub fn get_sibling(&self, offset: isize, length: usize) -> WordSlice<'a> {
        WordSlice::new(
            self.message,
            self.segment_id,
            (self.offset as isize + offset) as usize,
            length,
        )
    }
    pub fn dump(&self, indent: usize) {
        self.get_sibling(0, 1).dump(indent);
    }
}
