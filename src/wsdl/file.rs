use std::fs::File;
use std::io::{Read, Result};

pub fn load(location: &str) -> Result<String> {
    let mut file = try!(File::open(location));

    let mut content = String::new();
    try!(file.read_to_string(&mut content));

    Ok(content)
}