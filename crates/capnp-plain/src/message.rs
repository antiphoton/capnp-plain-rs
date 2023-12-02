pub mod segment;
pub mod tree;
pub mod word;

use anyhow::{Error, Result};

use crate::CapnpPlainStruct;

use self::{
    segment::Segment,
    tree::Node,
    word::{word_ref::WordRef, Word},
};

pub struct Message {
    segments: Vec<Segment>,
}

pub struct EncodingOptions {
    pub segment_table: bool,
    pub pack: bool,
}

impl Message {
    pub fn new_flat(words: Vec<Word>) -> Self {
        Message {
            segments: vec![Segment { words }],
        }
    }
    pub fn from_bytes(input: &[u8], options: EncodingOptions) -> Self {
        let words = if options.pack {
            Word::from_packed_bytes(input)
        } else {
            Word::from_bytes(input)
        };
        let segments = if options.segment_table {
            let segment_count = get_u32(&words, 0) as usize + 1;
            let lens: Vec<_> = (0..segment_count).map(|i| get_u32(&words, 1 + i)).collect();
            let mut words = words.iter().cloned().skip(segment_count / 2 + 1);
            lens.into_iter()
                .map(|x| Segment {
                    words: words.by_ref().take(x as usize).collect(),
                })
                .collect()
        } else {
            vec![Segment { words }]
        };
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

fn get_u32(words: &[Word], i: usize) -> u32 {
    let word = words[i / 2].0;
    let a = if i % 2 == 0 {
        [word[0], word[1], word[2], word[3]]
    } else {
        [word[4], word[5], word[6], word[7]]
    };
    u32::from_le_bytes(a)
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.segments.iter().map(|x| &x.words))
            .finish()
    }
}
