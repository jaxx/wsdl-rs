use std::fs::File;
use std::io::{Read, Result};

use encoding::all::UTF_8;
use encoding::DecoderTrap;
use encoding::types::decode;

pub fn load(location: &str) -> Result<String> {
    let mut file = try!(File::open(location));

    let mut bytes = Vec::new();
    try!(file.read_to_end(&mut bytes));

    let (decoded_contents, _) = decode(&bytes, DecoderTrap::Replace, UTF_8);

    Ok(match decoded_contents {
        Ok(contents) => contents,
        Err(e) => panic!("Failed to decode file contents: {}", e)
    })
}