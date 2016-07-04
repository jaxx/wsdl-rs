use std::result;
use std::io::Read;

use hyper::Client;
use hyper::error::Error as HttpError;

use encoding::all::UTF_8;
use encoding::DecoderTrap;
use encoding::types::decode;

pub type Result<T> = result::Result<T, HttpError>;

pub fn get(url: &str) -> Result<String> {
    let client = Client::new();
    let mut response = try!(client.get(url).send());

    let mut bytes = Vec::new();
    try!(response.read_to_end(&mut bytes));

    let (decoded_contents, _) = decode(&bytes, DecoderTrap::Replace, UTF_8);



    Ok(match decoded_contents {
        Ok(contents) => contents,
        Err(e) => panic!("Failed to decode response: {}", e)
    })
}