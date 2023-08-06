pub mod list_node;
pub mod struct_node;

use anyhow::Result;

use crate::pointer::LocalPointer;

use self::{list_node::ListNode, struct_node::StructNode};

use super::word::word_ref::WordRef;

pub enum Node {
    Struct(StructNode),
    List(ListNode),
}

impl Node {
    pub fn from_pointer(word_ref: WordRef) -> Result<Self> {
        let (pointer, content_base) = LocalPointer::read(word_ref)?;
        let node = match pointer {
            LocalPointer::Struct(struct_pointer) => {
                Self::Struct(StructNode::from_pointer(struct_pointer, content_base))
            }
            LocalPointer::List(list_pointer) => {
                Self::List(ListNode::from_pointer(list_pointer, content_base))
            }
        };
        Ok(node)
    }
}
