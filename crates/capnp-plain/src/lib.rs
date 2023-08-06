use message::tree::struct_node::StructNode;

pub mod message;
pub mod pointer;
mod util;

pub trait CapnpPlainStruct: Sized {
    fn from_node(_node: &StructNode) -> Self;
}
