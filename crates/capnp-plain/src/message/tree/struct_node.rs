use crate::{
    message::word::{word_ref::WordRef, Word},
    pointer::struct_pointer::StructPointer,
};

use super::Node;

pub struct StructNode {
    pub data: Vec<Word>,
    pub children: Vec<Option<Node>>,
}

impl StructNode {
    pub fn read(struct_pointer: StructPointer, content_base: WordRef) -> Self {
        let StructPointer {
            offset,
            data_size,
            pointer_size,
        } = struct_pointer;
        let data = content_base.get_sibling(offset, data_size).copy_to_owned();
        let pointers = content_base.get_sibling(offset + data_size as isize, pointer_size);
        let children: Vec<_> = (0..pointer_size)
            .map(|i| {
                let word_ref = pointers.get(i)?;
                if word_ref.0 == [0; 8] {
                    return None;
                }
                Node::read(word_ref).ok()
            })
            .collect();
        StructNode { data, children }
    }
}
