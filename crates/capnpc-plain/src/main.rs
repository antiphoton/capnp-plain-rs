mod compiler;
mod schema;

use std::io::Read;

use anyhow::Result;
use capnp_plain::message::Message;
use compiler::write_rust_code;

use schema::CodeGeneratorRequest;

fn main() -> Result<()> {
    let input = {
        let mut buffer = vec![];
        std::io::stdin().read_to_end(&mut buffer)?;
        buffer
    };
    let message = Message::from_bytes(&input);
    let code_generator_request = message.read_root::<CodeGeneratorRequest>()?;
    write_rust_code(&code_generator_request)?;
    Ok(())
}
