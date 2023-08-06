use crate::message::Message;

use super::word_ref::WordRef;

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
    pub fn clone_ref(&self) -> Self {
        Self::new(self.message, self.segment_id, self.offset, self.length)
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
    pub fn dump(&self, indent: usize) {
        const WORDS_PER_LINE: usize = 2;
        let tab = " ".repeat(indent);
        let lines = {
            let segment = &self.message.segments[self.segment_id];
            let slice = &segment.words[self.offset..(self.offset + self.length)];
            slice.chunks(WORDS_PER_LINE)
        };
        for line in lines {
            print!("{}", tab);
            for word in line {
                for x in word.0 {
                    print!("{:02x} ", x);
                }
                print!(" ");
            }
            for _ in line.len()..WORDS_PER_LINE {
                for _ in 0..25 {
                    print!(" ");
                }
            }
            for word in line {
                for x in word.0 {
                    if (0x21..=0x7e).contains(&x) {
                        print!("{}", x as char);
                    } else {
                        print!(".");
                    }
                }
                print!(" ");
            }
            println!();
        }
    }
}
