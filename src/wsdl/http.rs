use std::result;
use std::io::Read;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::error::Error as HttpError;

pub type Result<T> = result::Result<T, HttpError>;

#[derive(Debug)]
pub struct Response {
    pub status: StatusCode,
    pub body: String
}

pub fn get(url: &str) -> Result<Response> {
    let client = Client::new();
    let mut response = try!(client.get(url).send());

    let mut body = String::new();
    try!(response.read_to_string(&mut body));

    Ok(Response {
        status: response.status,
        body: body
    })
}