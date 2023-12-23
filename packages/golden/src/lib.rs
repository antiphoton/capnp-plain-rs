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
    let test_all_types: TestAllTypes = message.to_root().unwrap();
    serde_json::to_string_pretty(&test_all_types).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use capnp_plain::{
        message::{tree::struct_node::StructNode, word::Word},
        CapnpPlainStruct,
    };

    use super::*;

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
            Message::from_root(&data).as_flat().unwrap(),
            vec![Word([0, 0, 0, 0, 0, 0, 0, 0]),]
        );
    }
}
