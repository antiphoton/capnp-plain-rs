use std::{
    fs::{read, File},
    io::Write,
};

use capnp_plain_golden::get_pretty_json;

fn main() {
    let mut file = File::create("src/testdata/pretty.json").unwrap();
    file.write_all(get_pretty_json(&read("src/testdata/binary").unwrap()).as_bytes())
        .unwrap();
}
