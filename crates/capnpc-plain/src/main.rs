use std::io::Read;

use capnp_plain::message::Message;

fn main() {
    let input = {
        let mut buffer = vec![];
        std::io::stdin().read_to_end(&mut buffer).unwrap();
        buffer
    };
    let message = Message::from_bytes(&input);
    let code_generator_request = message.read_root().unwrap();
    let capnp_version = code_generator_request
        .read_pointer(2)
        .unwrap()
        .into_struct_reader()
        .unwrap();
    println!(
        "Capnp version: {}.{}.{}",
        capnp_version.read_u16(0, 0),
        capnp_version.read_u8(2, 0),
        capnp_version.read_u8(3, 0)
    );
}
