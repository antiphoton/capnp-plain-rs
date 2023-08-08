mod primitive_reader;
mod primitive_writer;

use crate::{
    message::word::{word_ref::WordRef, Word},
    pointer::{
        list_pointer::{ElementSize, ListPointer, ScalarSize},
        struct_pointer::StructPointer,
    },
    CapnpPlainStruct,
};

use super::struct_node::{get_struct_stack_size, StructNode};

pub enum ListNode {
    Scalar {
        scalar_size: ScalarSize,
        list_len: usize,
        data: Vec<Word>,
    },
    Composite(Vec<StructNode>),
}

#[derive(Debug, PartialEq)]
pub struct ListNodeSerializer {
    pub head: Word,
    pub content: Vec<Word>,
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
                        if let Some(base) = data.get(item_size * index) {
                            StructNode::from_pointer(tag.clone(), base)
                        } else {
                            StructNode::new()
                        }
                    })
                    .collect();
                Self::Composite(children)
            }
            _ => todo!(),
        }
    }
    pub fn to_builder(&self, offset: usize) -> ListNodeSerializer {
        match self {
            Self::Scalar {
                scalar_size,
                list_len,
                data,
            } => {
                let head = ListPointer {
                    offset: offset as isize,
                    element_size: ElementSize::Scalar(*scalar_size),
                    list_len: *list_len,
                };
                ListNodeSerializer {
                    head: head.into(),
                    content: data.to_owned(),
                }
            }
            Self::Composite(children) => {
                if children.is_empty() {
                    return ListNodeSerializer {
                        head: Word([0; 8]),
                        content: Vec::with_capacity(0),
                    };
                }
                let mut head = ListPointer {
                    offset: offset as isize,
                    element_size: ElementSize::Composite,
                    list_len: 0,
                };
                let mut stack = vec![];
                let mut heap = vec![];
                let child_stack_size = get_struct_stack_size(children);
                let tag = StructPointer {
                    offset: children.len() as isize,
                    data_size: child_stack_size.data,
                    pointer_size: child_stack_size.pointer,
                };
                stack.push(tag.into());
                for (i, child) in children.iter().enumerate() {
                    let child_stack_offset = 0; // This value does not matter.
                    let child_heap_offset =
                        (children.len() - 1 - i) * child_stack_size.total() + heap.len();
                    let child_builder =
                        child.to_builder(&child_stack_size, child_stack_offset, child_heap_offset);
                    stack.extend_from_slice(&child_builder.stack);
                    heap.extend_from_slice(&child_builder.heap);
                }
                let mut content = vec![];
                content.append(&mut stack);
                content.append(&mut heap);
                head.list_len = content.len() - 1;
                ListNodeSerializer {
                    head: head.into(),
                    content,
                }
            }
        }
    }
    pub fn read_struct_children<T: CapnpPlainStruct>(&self) -> Vec<T> {
        match self {
            Self::Composite(a) => a.iter().map(T::from_node).collect(),
            _ => vec![],
        }
    }
    pub fn write_struct_children<T: CapnpPlainStruct>(children: &[T]) -> Self {
        let children: Vec<_> = children.iter().map(|x| x.to_node()).collect();
        Self::Composite(children)
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Scalar { list_len, .. } => *list_len == 0,
            Self::Composite(x) => x.is_empty(),
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

#[cfg(test)]
mod tests {
    use crate::message::tree::Node;

    use super::*;

    #[test]
    fn builds_composite_list() {
        assert_eq!(
            ListNode::Composite(vec![
                StructNode {
                    data: vec![Word([1; 8])],
                    children: vec![],
                },
                StructNode {
                    data: vec![Word([2; 8]), Word([3; 8])],
                    children: vec![],
                }
            ])
            .to_builder(10),
            ListNodeSerializer {
                head: Word([41, 0, 0, 0, 39, 0, 0, 0]),
                content: vec![
                    Word([8, 0, 0, 0, 2, 0, 0, 0]),
                    Word([1; 8]),
                    Word([0; 8]),
                    Word([2; 8]),
                    Word([3; 8]),
                ]
            }
        );
    }
    #[test]
    fn builds_composite_list_with_nested_pointer() {
        assert_eq!(
            ListNode::Composite(vec![
                StructNode {
                    data: vec![Word([1; 8])],
                    children: vec![
                        None,
                        Some(Node::Struct(StructNode {
                            data: vec![Word([2; 8])],
                            children: vec![Some(Node::Struct(StructNode {
                                data: vec![Word([3; 8])],
                                children: vec![]
                            }))],
                        })),
                    ],
                },
                StructNode {
                    data: vec![Word([4; 8]), Word([5; 8])],
                    children: vec![
                        Some(Node::Struct(StructNode {
                            data: vec![Word([6; 8])],
                            children: vec![],
                        })),
                        None,
                    ],
                }
            ])
            .to_builder(20),
            ListNodeSerializer {
                head: Word([81, 0, 0, 0, 103, 0, 0, 0]),
                content: vec![
                    Word([8, 0, 0, 0, 2, 0, 2, 0]),
                    Word([1; 8]),
                    Word([0; 8]),
                    Word([0; 8]),
                    Word([16, 0, 0, 0, 1, 0, 1, 0]),
                    Word([4; 8]),
                    Word([5; 8]),
                    Word([16, 0, 0, 0, 1, 0, 0, 0]),
                    Word([0; 8]),
                    Word([2; 8]),
                    Word([0, 0, 0, 0, 1, 0, 0, 0]),
                    Word([3; 8]),
                    Word([6; 8]),
                ]
            }
        );
    }
}
