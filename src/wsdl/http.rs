use std::result;
use std::io::Read;

use hyper::Client;
use hyper::error::Error as HttpError;

pub type Result<T> = result::Result<T, HttpError>;

pub fn get(url: &str) -> Result<Vec<u8>> {
    let client = Client::new();
    let mut response = try!(client.get(url).send());

    let mut bytes = Vec::new();
    try!(response.read_to_end(&mut bytes));

    Ok(bytes)
}