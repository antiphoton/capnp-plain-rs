mod context;
mod language;

use std::{fs::File, io::Write};

use anyhow::{ensure, Result};
use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::CodeGeneratorRequest;
fn get_output_file_name(
    code_generator_request: &CodeGeneratorRequest,
    extension: &str,
) -> Result<String> {
    let request_files = &code_generator_request.requested_files;
    ensure!(request_files.len() == 1);
    let s = request_files[0].filename.strip_suffix(".capnp").unwrap();
    Ok(format!("{}_capnp.{}", s, extension))
}

pub fn write_rust_code(code_generator_request: &CodeGeneratorRequest) -> Result<()> {
    let mut file = File::create(get_output_file_name(code_generator_request, "rs")?)?;
    let tokens = language::rust::generate_code(code_generator_request);
    let tokens: TokenStream = tokens.into_iter().collect();
    let output = quote! {
        //! @generated
        #![allow(clippy::all)]
        #![allow(dead_code)]
        #![allow(non_camel_case_types)]
        use anyhow::Result;
        use capnp_plain::pointer::struct_pointer::{CapnpPlainStruct, StructReader};
        #tokens
    };
    let output = syn::parse2(output)?;
    let output = prettyplease::unparse(&output);
    file.write_all(output.as_bytes())?;
    Ok(())
}
