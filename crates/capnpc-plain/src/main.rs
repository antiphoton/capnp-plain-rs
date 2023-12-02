mod compiler;
mod schema;

use std::io::Read;

use anyhow::Result;
use capnp_plain::message::{EncodingOptions, Message};

use compiler::compile;
use schema::schema_capnp::CodeGeneratorRequest;

fn main() -> Result<()> {
    let input = {
        let mut buffer = vec![];
        std::io::stdin().read_to_end(&mut buffer)?;
        buffer
    };
    let message = Message::from_bytes(
        &input,
        EncodingOptions {
            segment_table: true,
            pack: false,
        },
    );
    let code_generator_request = message.read_root::<CodeGeneratorRequest>()?;
    compile(&code_generator_request)?;
    Ok(())
}
