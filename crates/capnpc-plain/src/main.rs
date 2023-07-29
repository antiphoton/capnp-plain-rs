use std::io::Read;

use capnp_plain::message::Message;

fn main() {
    let input = {
        let mut buffer = vec![];
        std::io::stdin().read_to_end(&mut buffer).unwrap();
        buffer
    };
    let message = Message::from_bytes(&input);
    message.dump(0);
}
