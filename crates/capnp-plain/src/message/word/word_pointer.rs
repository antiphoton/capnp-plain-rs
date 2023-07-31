use std::ops::Deref;

use crate::message::Message;

use super::{word_slice::WordSlice, Word};

pub struct WordPointer<'a> {
    message: &'a Message,
    segment_id: usize,
    offset: usize,
}

impl<'a> Deref for WordPointer<'a> {
    type Target = Word;
    fn deref(&self) -> &Self::Target {
        let segment = &self.message.segments[self.segment_id];
        &segment.words[self.offset]
    }
}

impl<'a> WordPointer<'a> {
    pub fn new(message: &'a Message, segment_id: usize, offset: usize) -> Self {
        WordPointer {
            message,
            segment_id,
            offset,
        }
    }
    pub fn dump(&self, indent: usize) {
        let slice = WordSlice::new(self.message, self.segment_id, self.offset, 1);
        slice.dump(indent);
    }
}
