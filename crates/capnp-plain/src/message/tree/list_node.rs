mod primitive_reader;

use crate::{
    message::word::{word_ref::WordRef, Word},
    pointer::{
        list_pointer::{ElementSize, ListPointer, ScalarSize},
        struct_pointer::StructPointer,
    },
    CapnpPlainStruct,
};

use super::struct_node::StructNode;

pub enum ListNode {
    Scalar {
        scalar_size: ScalarSize,
        list_len: usize,
        data: Vec<Word>,
    },
    Composite(Vec<StructNode>),
}

impl ListNode {
    pub fn from_pointer(list_pointer: ListPointer, content_base: WordRef) -> Self {
        let ListPointer {
            offset,
            element_size,
            list_len,
        } = list_pointer;
        match element_size {
            ElementSize::Scalar(scalar_size) => {
                let data_word_length = match scalar_size {
                    ScalarSize::Void => 0,
                    ScalarSize::OneBit => (list_len + 63) / 64,
                    ScalarSize::OneByte => (list_len + 7) / 8,
                    ScalarSize::TwoBytes => (list_len + 3) / 4,
                    ScalarSize::FourBytes => (list_len + 1) / 2,
                    ScalarSize::EightBytes => list_len,
                };
                let data = content_base
                    .get_sibling(offset, data_word_length)
                    .copy_to_owned();
                Self::Scalar {
                    scalar_size,
                    list_len,
                    data,
                }
            }
            ElementSize::Composite => {
                let mut tag =
                    StructPointer::try_from(*content_base.get_sibling(offset, 1).get(0).unwrap())
                        .unwrap();
                let count = tag.offset as usize;
                assert!((tag.data_size + tag.pointer_size) * count <= list_len);
                tag.offset = 0;
                let item_size = tag.data_size + tag.pointer_size;
                let data = content_base.get_sibling(offset + 1, list_len);
                let children: Vec<_> = (0..count)
                    .map(|index| {
                        StructNode::from_pointer(tag.clone(), data.get(item_size * index).unwrap())
                    })
                    .collect();
                Self::Composite(children)
            }
            _ => todo!(),
        }
    }
    pub fn read_struct_children<T: CapnpPlainStruct>(&self) -> Vec<T> {
        match self {
            Self::Composite(a) => a.iter().map(T::from_node).collect(),
            _ => vec![],
        }
    }
}

impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scalar {
                scalar_size,
                list_len,
                data,
            } => f
                .debug_struct("ScalarList")
                .field("len", list_len)
                .field("scalar", scalar_size)
                .field("data", data)
                .finish(),
            Self::Composite(a) => f.debug_list().entries(a).finish(),
        }
    }
}
