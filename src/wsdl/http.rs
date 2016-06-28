use std::result;
use std::io::Read;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::ContentType;
use hyper::mime::Mime;
use hyper::mime::Attr::Charset;
use hyper::error::Error as HttpError;

use encoding::label::encoding_from_whatwg_label;
use encoding::types::EncodingRef;

pub type Result<T> = result::Result<T, HttpError>;

pub struct Response {
    pub status: StatusCode,
    pub encoding: Option<EncodingRef>,
    pub body: String,
}

pub fn get(url: &str) -> Result<Response> {
    let client = Client::new();
    let mut response = try!(client.get(url).send());

    let mut body = String::new();
    try!(response.read_to_string(&mut body));

    Ok(Response {
        status: response.status,
        encoding: response.headers
            .get::<ContentType>()
            .and_then(|ct| parse_charset_from_content_type(ct))
            .and_then(|charset| encoding_from_whatwg_label(&charset)),
        body: body,
    })
}

fn parse_charset_from_content_type(header: &ContentType) -> Option<String> {
    match **header {
        Mime(_, _, ref params) => {
            match params.iter().find(|&&(ref ptype, _)| *ptype == Charset) {
                Some(&(_, ref charset)) => Some(charset.to_string()),
                None => None,
            }
        }
    }
}