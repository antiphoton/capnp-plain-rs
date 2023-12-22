mod primitive_reader;
mod primitive_writer;

use crate::{
    message::{
        word::{word_ref::WordRef, Word},
        Message,
    },
    pointer::struct_pointer::StructPointer,
    util::array::extend_and_get,
};

use super::{list_node::ListNode, Node};

pub struct StructNode {
    pub data: Vec<Word>,
    pub children: Vec<Option<Node>>,
}

#[derive(Debug, PartialEq)]
pub struct StructNodeSerializer {
    pub head: Word,
    pub stack: Vec<Word>,
    pub heap: Vec<Word>,
}

impl StructNode {
    pub fn new() -> Self {
        StructNode {
            data: Vec::with_capacity(0),
            children: Vec::with_capacity(0),
        }
    }
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
                let child = Node::from_pointer(word_ref).ok();
                match child {
                    Some(Node::List(x)) => {
                        if x.is_empty() {
                            None
                        } else {
                            Some(Node::List(x))
                        }
                    }
                    _ => child,
                }
            })
            .collect();
        StructNode { data, children }
    }
    pub fn to_builder(
        &self,
        stack_size: &StructStackSize,
        stack_offset: usize,
        heap_offset: usize,
    ) -> StructNodeSerializer {
        let mut stack = [Word([0; 8])].repeat(stack_size.total());
        let data_size = std::cmp::min(stack_size.data, self.data.len());
        stack[0..data_size].copy_from_slice(&self.data[0..data_size]);
        let mut heap = vec![];
        for (i, child) in self.children.iter().enumerate() {
            if i >= stack_size.pointer {
                continue;
            }
            let child_stack_offset = stack_size.pointer - 1 - i + heap_offset + heap.len();
            let child_head = match child {
                Some(Node::Struct(struct_child)) => {
                    let child_stack_size =
                        get_struct_stack_size(std::slice::from_ref(struct_child));
                    let child_builder =
                        struct_child.to_builder(&child_stack_size, child_stack_offset, 0);
                    heap.extend_from_slice(&child_builder.stack);
                    heap.extend_from_slice(&child_builder.heap);
                    child_builder.head
                }
                Some(Node::List(list_child)) => {
                    let child_builder = list_child.to_builder(child_stack_offset);
                    heap.extend_from_slice(&child_builder.content);
                    child_builder.head
                }
                _ => continue,
            };
            stack[stack_size.data + i] = child_head;
        }
        let head = StructPointer {
            offset: stack_offset as isize,
            data_size: stack_size.data,
            pointer_size: stack_size.pointer,
        };
        StructNodeSerializer {
            head: head.into(),
            stack,
            heap,
        }
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
    pub fn write_struct_pointer(&mut self, offset: u32, child: StructNode) {
        let _ = extend_and_get(&mut self.children, offset as usize).insert(Node::Struct(child));
    }
    pub fn write_list_pointer(&mut self, offset: u32, child: ListNode) {
        let target = extend_and_get(&mut self.children, offset as usize);
        if child.is_empty() {
            target.take();
        } else {
            let _ = target.insert(Node::List(child));
        }
    }
    pub fn remove_pointer(&mut self, offset: u32) {
        extend_and_get(&mut self.children, offset as usize).take();
    }
}

impl Default for StructNode {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
pub struct StructStackSize {
    pub data: usize,
    pub pointer: usize,
}

impl StructStackSize {
    pub fn total(&self) -> usize {
        self.data + self.pointer
    }
}

fn trim_right_with<T>(a: &[T], f: impl Fn(&T) -> bool) -> usize {
    let r = a.iter().rev().take_while(|x| f(x)).count();
    a.len() - r
}

pub fn get_struct_stack_size(a: &[StructNode]) -> StructStackSize {
    let data_size = a
        .iter()
        .map(|x| trim_right_with(&x.data, Word::is_zero))
        .max()
        .unwrap_or(0);
    let pointer_size = a
        .iter()
        .map(|x| trim_right_with(&x.children, Option::is_none))
        .max()
        .unwrap_or(0);

    StructStackSize {
        data: data_size,
        pointer: pointer_size,
    }
}

impl From<&StructNodeSerializer> for Message {
    fn from(input: &StructNodeSerializer) -> Self {
        let mut words = vec![input.head];
        words.extend_from_slice(&input.stack);
        words.extend_from_slice(&input.heap);
        Message::new_flat(words)
    }
}

struct ChildrenDebugHelper<'a>(&'a Vec<Option<Node>>);

impl<'a> std::fmt::Debug for ChildrenDebugHelper<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = f.debug_map();
        for (i, x) in self.0.iter().enumerate() {
            if let Some(x) = x {
                map.entry(&i, x);
            } else {
                map.entry(&i, &Option::<()>::None);
            }
        }
        map.finish()
    }
}

impl std::fmt::Debug for StructNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructNode")
            .field("data", &self.data)
            .field("children", &ChildrenDebugHelper(&self.children))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::message::Message;

    use super::*;

    fn build_node(node: StructNode) -> StructNodeSerializer {
        let stack_size = get_struct_stack_size(std::slice::from_ref(&node));
        node.to_builder(&stack_size, 0, 0)
    }
    #[test]
    fn trims_null_pointer_children() {
        assert_eq!(
            get_struct_stack_size(&[StructNode {
                data: vec![],
                children: vec![None, None],
            }]),
            StructStackSize {
                data: 0,
                pointer: 0
            }
        );
        assert_eq!(
            get_struct_stack_size(&[StructNode {
                data: vec![],
                children: vec![Some(Node::List(ListNode::Composite(vec![]))), None, None],
            }]),
            StructStackSize {
                data: 0,
                pointer: 1
            }
        );
    }
    #[test]
    fn builds_empty_node() {
        assert_eq!(
            build_node(StructNode {
                data: vec![],
                children: vec![],
            }),
            StructNodeSerializer {
                head: Word([0; 8]),
                stack: vec![],
                heap: vec![],
            }
        );
    }
    #[test]
    fn builds_and_reads_text_child() {
        let mut x = StructNode::new();
        x.write_text(1, "01234567");
        x.write_text(2, "012345678");
        x.write_text(0, "0123456");
        x.write_text(3, "ABC");
        let x = build_node(x);
        assert_eq!(
            x,
            StructNodeSerializer {
                head: Word([0, 0, 0, 0, 0, 0, 4, 0]),
                stack: vec![
                    Word([13, 0, 0, 0, 66, 0, 0, 0]),
                    Word([13, 0, 0, 0, 74, 0, 0, 0]),
                    Word([17, 0, 0, 0, 82, 0, 0, 0]),
                    Word([21, 0, 0, 0, 34, 0, 0, 0]),
                ],
                heap: vec![
                    Word([48, 49, 50, 51, 52, 53, 54, 0]),
                    Word([48, 49, 50, 51, 52, 53, 54, 55]),
                    Word([0; 8]),
                    Word([48, 49, 50, 51, 52, 53, 54, 55]),
                    Word([56, 0, 0, 0, 0, 0, 0, 0]),
                    Word([65, 66, 67, 0, 0, 0, 0, 0]),
                ],
            }
        );
        let x = Message::from(&x);
        let x = Node::from_pointer(WordRef::new(&x, 0, 0)).unwrap();
        let Node::Struct(x) = x else {
            panic!();
        };
        assert_eq!(x.read_text(0), "0123456");
        assert_eq!(x.read_text(1), "01234567");
        assert_eq!(x.read_text(2), "012345678");
        assert_eq!(x.read_text(3), "ABC");
    }
    #[test]
    fn empty_text_child() {
        let mut x = StructNode::new();
        x.write_text(0, "ABC");
        x.write_text(0, "");
        let x = build_node(x);
        assert_eq!(
            x,
            StructNodeSerializer {
                head: Word([0, 0, 0, 0, 0, 0, 0, 0]),
                stack: vec![],
                heap: vec![],
            }
        );
    }
    #[test]
    fn builds_struct_child() {
        assert_eq!(
            build_node(StructNode {
                data: vec![Word([1; 8])],
                children: vec![
                    Some(Node::Struct(StructNode {
                        data: vec![Word([2; 8]), Word([3; 8])],
                        children: vec![],
                    })),
                    None,
                    Some(Node::Struct(StructNode {
                        data: vec![Word([4; 8]), Word([5; 8])],
                        children: vec![],
                    })),
                ],
            }),
            StructNodeSerializer {
                head: Word([0, 0, 0, 0, 1, 0, 3, 0]),
                stack: vec![
                    Word([1; 8]),
                    Word([8, 0, 0, 0, 2, 0, 0, 0]),
                    Word([0; 8]),
                    Word([8, 0, 0, 0, 2, 0, 0, 0]),
                ],
                heap: vec![Word([2; 8]), Word([3; 8]), Word([4; 8]), Word([5; 8])]
            }
        );
    }
    #[test]
    fn builds_struct_child_with_nested_pointer() {
        assert_eq!(
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
            }
            .to_builder(
                &StructStackSize {
                    data: 1,
                    pointer: 2
                },
                10,
                20
            ),
            StructNodeSerializer {
                head: Word([40, 0, 0, 0, 1, 0, 2, 0]),
                stack: vec![Word([1; 8]), Word([0; 8]), Word([80, 0, 0, 0, 1, 0, 1, 0]),],
                heap: vec![Word([2; 8]), Word([0, 0, 0, 0, 1, 0, 0, 0]), Word([3; 8]),]
            }
        );
    }
    #[test]
    fn writes_and_reads_scalar_field() {
        let mut x = StructNode::new();
        x.write_u64(2, 0x1111111111111111, 0);
        assert_eq!(x.read_u64(2, 0), 0x1111111111111111);
        x.write_u16(7, 123, 0);
        assert_eq!(x.read_u64(2, 0), 0x1111111111111111);
    }
}
