mod schema;

use std::{fs::File, io::Write};

use capnp_plain::message::Message;

use crate::schema::test_capnp::TestAllTypes;

fn get_pretty_json() -> String {
    let message = Message::from_bytes(include_bytes!("./testdata/binary"));
    let test_all_types: TestAllTypes = message.read_root().unwrap();
    serde_json::to_string_pretty(&test_all_types).unwrap()
}

fn main() {
    let mut file = File::create("src/testdata/pretty.json").unwrap();
    file.write_all(get_pretty_json().as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(include_str!("testdata/pretty.json"), get_pretty_json());
    }
}
