use crate::message::Message;

use super::{word_ref::WordRef, Word};

pub struct WordSlice<'a> {
    message: &'a Message,
    segment_id: usize,
    offset: usize,
    length: usize,
}

impl<'a> WordSlice<'a> {
    pub fn new(message: &'a Message, segment_id: usize, offset: usize, length: usize) -> Self {
        WordSlice {
            message,
            segment_id,
            offset,
            length,
        }
    }
    fn get_raw(&self) -> &[Word] {
        let segment = &self.message.segments[self.segment_id];
        if self.offset >= segment.words.len() {
            return &[];
        }
        let end = std::cmp::min(segment.words.len(), self.offset + self.length);
        &segment.words[self.offset..end]
    }
    pub fn copy_to_owned(&self) -> Vec<Word> {
        self.get_raw().to_owned()
    }
    pub fn get(&self, offset: usize) -> Option<WordRef> {
        if offset < self.length {
            Some(WordRef::new(
                self.message,
                self.segment_id,
                self.offset + offset,
            ))
        } else {
            None
        }
    }
}

impl std::fmt::Debug for WordSlice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let location = format!(
            "[{}][{}:{}]",
            self.segment_id,
            self.offset,
            self.offset + self.length
        );
        f.debug_struct("WordSlice")
            .field("location", &location)
            .field("data", &self.get_raw())
            .finish()
    }
}
