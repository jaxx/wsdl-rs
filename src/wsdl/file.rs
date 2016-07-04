use std::fs::File;
use std::io::{Read, Result};

pub fn load(location: &str) -> Result<Vec<u8>> {
    let mut file = try!(File::open(location));

    let mut bytes = Vec::new();
    try!(file.read_to_end(&mut bytes));

    Ok(bytes)
}