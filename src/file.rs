use std::fs::File;
use std::io::{Read, Result};

pub fn load(location: &str) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    File::open(location)?
        .read_to_end(&mut bytes)?;

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_file_test() {
        let result = load("examples/files/etoimik.wsdl");

        assert!(result.is_ok());
        let file_contents = result.unwrap();
        assert!(file_contents.len() > 0);
    }

    #[test]
    fn load_file_fail_test() {
        let result = load("examples/files/etoimik2.wsdl");
        assert!(result.is_err());
    }
}