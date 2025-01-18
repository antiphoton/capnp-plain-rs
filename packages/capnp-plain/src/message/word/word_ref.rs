use std::ops::Deref;

use crate::message::Message;

use super::{word_slice::WordSlice, Word};

pub struct WordRef<'a> {
    message: &'a Message,
    segment_id: usize,
    offset: usize,
}

impl Deref for WordRef<'_> {
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
    pub fn get_next(&self) -> Self {
        Self::new(self.message, self.segment_id, self.offset + 1)
    }
    pub fn get_sibling(&self, offset: isize, length: usize) -> WordSlice<'a> {
        WordSlice::new(
            self.message,
            self.segment_id,
            (self.offset as isize + offset) as usize,
            length,
        )
    }
    pub fn get_cousin(&self, segment_id: usize, offset: usize) -> Self {
        Self::new(self.message, segment_id, offset)
    }
}

impl std::fmt::Debug for WordRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let location = format!("[{}][{}]", self.segment_id, self.offset,);
        let data = self.message.segments[self.segment_id].words[self.offset];
        f.debug_struct("WordSlice")
            .field("location", &location)
            .field("data", &data)
            .finish()
    }
}
