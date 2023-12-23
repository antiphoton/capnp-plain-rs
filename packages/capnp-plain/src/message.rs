pub mod segment;
pub mod tree;
pub mod word;

use anyhow::{Error, Result};

use crate::{message::tree::struct_node::get_struct_stack_size, CapnpPlainStruct};

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
    pub fn as_flat(&self) -> Option<&[Word]> {
        if self.segments.len() == 1 {
            let segment = &self.segments[0];
            Some(&segment.words)
        } else {
            None
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
    pub fn to_bytes(&self, options: EncodingOptions) -> Vec<u8> {
        let mut output = vec![];
        let segment_count = self.segments.len();
        if options.segment_table {
            assert!(segment_count >= 1);
            push_u32(&mut output, segment_count - 1);
            for segment in &self.segments {
                push_u32(&mut output, segment.words.len());
            }
            if segment_count % 2 == 0 {
                push_u32(&mut output, 0);
            }
        } else {
            assert!(segment_count == 1);
        }
        for segment in &self.segments {
            for word in &segment.words {
                output.extend_from_slice(&word.0);
            }
        }
        if options.pack {
            Word::to_packed_bytes(&output)
        } else {
            output
        }
    }
    pub fn to_root<T: CapnpPlainStruct>(&self) -> Result<T> {
        let word_ref = WordRef::new(self, 0, 0);
        let root = Node::from_pointer(word_ref)?;
        match root {
            Node::Struct(x) => Ok(T::from_node(&x)),
            _ => Err(Error::msg("Message root is not a struct.")),
        }
    }
    pub fn from_root(data: &impl CapnpPlainStruct) -> Self {
        let node = data.to_node();
        let stack_size = get_struct_stack_size(std::slice::from_ref(&node));
        let serializer = node.to_builder(&stack_size, 0, 0);
        Self::from(&serializer)
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

fn push_u32(output: &mut Vec<u8>, item: usize) {
    output.extend_from_slice(&(item as u32).to_le_bytes());
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.segments.iter().map(|x| &x.words))
            .finish()
    }
}
