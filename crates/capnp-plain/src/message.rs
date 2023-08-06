pub mod segment;
pub mod tree;
pub mod word;

use anyhow::Result;

use crate::{
    message::word::word_slice::WordSlice, util::split_array::split_array_ref, CapnpPlainStruct,
};

use self::{segment::Segment, tree::Node, word::word_ref::WordRef};

pub struct Message {
    segments: Vec<Segment>,
}

impl Message {
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
            _ => todo!(),
        }
    }
    pub fn dump(&self, indent: usize) {
        let tab = " ".repeat(indent);
        for (i, segment) in self.segments.iter().enumerate() {
            println!("{} Segment {}", tab, i);
            let slice = WordSlice::new(self, i, 0, segment.words.len());
            slice.dump(indent + 2);
        }
    }
}

fn take_u32(bytes: &mut &[u8]) -> u32 {
    let value;
    (value, *bytes) = split_array_ref(bytes);
    u32::from_le_bytes(*value)
}
