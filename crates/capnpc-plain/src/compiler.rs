mod context;
mod language;

use anyhow::{ensure, Result};

use crate::schema::schema_capnp::CodeGeneratorRequest;
fn get_output_file_name(
    code_generator_request: &CodeGeneratorRequest,
    extension: &str,
) -> Result<String> {
    let request_files = &code_generator_request.requested_files;
    ensure!(request_files.len() == 1);
    let s = request_files[0].filename.strip_suffix(".capnp").unwrap();
    Ok(format!("{}_capnp.{}", s, extension))
}

pub fn compile_rust_code(code_generator_request: &CodeGeneratorRequest) -> Result<()> {
    language::rust::compile(code_generator_request)
}
