use message::tree::struct_node::StructNode;

pub mod message;
pub mod pointer;
mod util;

pub trait CapnpPlainStruct: Sized {
    fn from_node(node: &StructNode) -> Self;
    fn update_node(&self, node: &mut StructNode);
    fn to_node(&self) -> StructNode {
        let mut output = StructNode {
            data: vec![],
            children: vec![],
        };
        self.update_node(&mut output);
        output
    }
}

pub trait CapnpPlainEnum: Sized {
    fn encode(self) -> u16;
    fn decode(x: u16) -> Self;
}
