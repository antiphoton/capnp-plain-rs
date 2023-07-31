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
    let nodes = code_generator_request
        .read_pointer(0)
        .unwrap()
        .into_list_reader()
        .unwrap();
    for i in 0..nodes.len() {
        let node = nodes.read_struct_child(i).unwrap();
        let node_id = node.read_u64(0, 0);
        println!("{:x}", node_id);
    }
}
