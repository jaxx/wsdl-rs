use std::fs::File;
use std::io::{Read, Result};

pub fn load(location: &str) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    File::open(location)?
        .read_to_end(&mut bytes)?;

    Ok(bytes)
}