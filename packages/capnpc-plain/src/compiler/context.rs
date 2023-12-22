use std::collections::HashMap;

use convert_case::{Case, Casing};

use crate::schema::schema_capnp::{CodeGeneratorRequest, Node};

pub struct CompilerContext<'a> {
    pub node_map: HashMap<u64, &'a Node>,
}

impl<'a> CompilerContext<'a> {
    pub fn new(code_generator_request: &'a CodeGeneratorRequest) -> Self {
        let CodeGeneratorRequest { nodes, .. } = code_generator_request;
        let node_map = nodes.iter().map(|node| (node.0.id, node)).collect();
        CompilerContext { node_map }
    }
    pub fn get_node<'b>(&'b self, node_id: u64) -> Option<&'a Node> {
        self.node_map.get(&node_id).copied()
    }
    fn get_name_segments<'b>(&'b self, node: &'b Node) -> Vec<&'b str> {
        let s = &node.0.display_name[(node.0.display_name_prefix_length as usize)..];
        if let Some(p) = self.node_map.get(&node.0.scope_id) {
            let mut a = self.get_name_segments(p);
            a.push(s);
            a
        } else {
            vec![s]
        }
    }
    pub fn get_full_name(&self, node: &Node) -> String {
        let a: Vec<_> = self
            .get_name_segments(node)
            .iter()
            .map(|s| s.to_case(Case::UpperCamel))
            .collect();
        a[1..].join("__")
    }
}
