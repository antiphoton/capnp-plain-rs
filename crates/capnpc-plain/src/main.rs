mod schema;

use std::io::Read;

use capnp_plain::message::Message;
use schema::CodeGeneratorRequest;

fn main() {
    let input = {
        let mut buffer = vec![];
        std::io::stdin().read_to_end(&mut buffer).unwrap();
        buffer
    };
    let message = Message::from_bytes(&input);
    let code_generator_request = message.read_root::<CodeGeneratorRequest>().unwrap();
    dbg!(code_generator_request);
}
