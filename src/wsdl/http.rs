use std::result;
use std::io::Read;

use hyper::Client;
use hyper::error::Error as HttpError;

pub type Result<T> = result::Result<T, HttpError>;

pub fn get(url: &str) -> Result<Vec<u8>> {
    let client = Client::new();
    let mut bytes = Vec::new();

    client
        .get(url)
        .send()?
        .read_to_end(&mut bytes);

    Ok(bytes)
}