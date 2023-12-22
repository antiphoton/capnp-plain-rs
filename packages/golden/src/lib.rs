pub mod schema;

use capnp_plain::message::{EncodingOptions, Message};
use schema::test_capnp::TestAllTypes;

pub fn get_pretty_json(capnp_bytes: &[u8]) -> String {
    let message = Message::from_bytes(
        capnp_bytes,
        EncodingOptions {
            pack: false,
            segment_table: true,
        },
    );
    let test_all_types: TestAllTypes = message.read_root().unwrap();
    serde_json::to_string_pretty(&test_all_types).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use capnp_plain::{
        message::{
            tree::struct_node::{get_struct_stack_size, StructNode},
            word::Word,
        },
        CapnpPlainStruct,
    };

    use super::*;

    fn serialize_data(data: &impl CapnpPlainStruct) -> Vec<Word> {
        let node = data.to_node();
        let stack_size = get_struct_stack_size(std::slice::from_ref(&node));
        let serializer = node.to_builder(&stack_size, 0, 0);
        Message::from(&serializer)
            .as_flat()
            .unwrap()
            .into_iter()
            .cloned()
            .collect()
    }

    #[test]
    fn capnp_golden() {
        assert_eq!(
            include_str!("testdata/pretty.json"),
            get_pretty_json(&read("src/testdata/binary").unwrap())
        );
    }

    #[test]
    fn empty_struct() {
        let data = TestAllTypes::from_node(&StructNode::default());
        assert_eq!(
            serialize_data(&data),
            vec![
                Word([0, 0, 0, 0, 0, 0, 0x12, 0]),
                Word([0x45, 0, 0, 0, 0x0a, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
                Word([0, 0, 0, 0, 0, 0, 0, 0]),
            ]
        );
    }
}
