pub mod segment;
pub mod tree;
pub mod word;

use anyhow::{Error, Result};

use crate::{util::split_array::split_array_ref, CapnpPlainStruct};

use self::{
    segment::Segment,
    tree::Node,
    word::{word_ref::WordRef, Word},
};

pub struct Message {
    segments: Vec<Segment>,
}

impl Message {
    pub fn new_flat(words: Vec<Word>) -> Self {
        Message {
            segments: vec![Segment { words }],
        }
    }
    pub fn from_bytes(mut input: &[u8]) -> Self {
        let segment_count = take_u32(&mut input) as usize + 1;
        let lens: Vec<_> = (0..segment_count)
            .map(|_| take_u32(&mut input) as usize)
            .collect();
        if segment_count % 2 == 0 {
            take_u32(&mut input);
        }
        let segments = lens
            .into_iter()
            .map(|len| {
                let data;
                (data, input) = input.split_at(len * 8);
                Segment::from_bytes(data)
            })
            .collect();
        Message { segments }
    }
    pub fn read_root<T: CapnpPlainStruct>(&self) -> Result<T> {
        let word_ref = WordRef::new(self, 0, 0);
        let root = Node::from_pointer(word_ref)?;
        match root {
            Node::Struct(x) => Ok(T::from_node(&x)),
            _ => Err(Error::msg("Message root is not a struct.")),
        }
    }
}

fn take_u32(bytes: &mut &[u8]) -> u32 {
    let value;
    (value, *bytes) = split_array_ref(bytes);
    u32::from_le_bytes(*value)
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.segments.iter().map(|x| &x.words))
            .finish()
    }
}
