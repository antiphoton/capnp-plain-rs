mod primitive_reader;

use crate::{
    message::word::{word_ref::WordRef, Word},
    pointer::struct_pointer::StructPointer,
};

use super::{list_node::ListNode, Node};

pub struct StructNode {
    pub data: Vec<Word>,
    pub children: Vec<Option<Node>>,
}

impl StructNode {
    pub fn from_pointer(struct_pointer: StructPointer, content_base: WordRef) -> Self {
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
                Node::from_pointer(word_ref).ok()
            })
            .collect();
        StructNode { data, children }
    }
    pub fn read_list<T>(&self, offset: u32, f: impl FnOnce(&ListNode) -> Vec<T>) -> Vec<T> {
        let Some(Some(Node::List(list_node))) = self.children.get(offset as usize) else {
            return Vec::with_capacity(0);
        };
        f(list_node)
    }
    pub fn read_struct(&self, offset: u32) -> Option<&StructNode> {
        match self.children.get(offset as usize) {
            Some(Some(Node::Struct(p))) => Some(p),
            _ => None,
        }
    }
}
