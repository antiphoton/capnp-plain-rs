mod context;
mod language;

use std::collections::BTreeMap;

use anyhow::{ensure, Result};

use crate::schema::schema_capnp::{CodeGeneratorRequest, Field};

fn get_output_file_name(
    code_generator_request: &CodeGeneratorRequest,
    extension: &str,
) -> Result<String> {
    let request_files = &code_generator_request.requested_files;
    ensure!(request_files.len() == 1);
    let s = request_files[0].filename.strip_suffix(".capnp").unwrap();
    Ok(format!("{}_capnp.{}", s, extension))
}

fn split_fields(fields: &[Field]) -> (Vec<&Field>, BTreeMap<u16, &Field>) {
    let (common_fields, variant_fields) = fields
        .iter()
        .partition::<Vec<_>, _>(|f| f.0.discriminant_value == 0xffff);
    let variant_fields: BTreeMap<_, _> = variant_fields
        .into_iter()
        .map(|f| (f.0.discriminant_value, f))
        .collect();
    (common_fields, variant_fields)
}

pub fn compile(code_generator_request: &CodeGeneratorRequest) -> Result<()> {
    language::rust::compile(code_generator_request)?;
    Ok(())
}
