extern crate hyper;
extern crate xml;
extern crate encoding;

mod http;
mod file;

use std::io::Result as IoResult;
use std::fmt;
use http::Result as HttpResult;
use xml::reader::{EventReader, XmlEvent};
use encoding::all::UTF_8;
use encoding::DecoderTrap;
use encoding::types::decode;

const NAMESPACE_WSDL: &'static str = "http://schemas.xmlsoap.org/wsdl/";

pub struct Wsdl {
    //operations: Vec<WsdlOperation>
    //types: Vec<WsdlType> 
}

struct WsdlType {
    
}

struct WsdlOperation {

}

#[derive(Debug)]
pub enum ParseError {
    Http(hyper::Error),
    Io(std::io::Error)
}

pub type ParseResult<T> = Result<T, ParseError>;

impl Wsdl {
    pub fn load_from_url(url: &str) -> ParseResult<Wsdl> {
        let contents = http::get(url).map_err(ParseError::Http)?;
        let decoded_contents = decode_contents(contents);
        parse_wsdl(&decoded_contents[..])
    }

    pub fn load_from_file(location: &str) -> ParseResult<Wsdl> {
        let contents = file::load(location).map_err(ParseError::Io)?;
        let decoded_contents = decode_contents(contents);
        parse_wsdl(&decoded_contents[..])
    }
}

fn decode_contents(bytes: Vec<u8>) -> Vec<u8> {
    let (decoded_contents, _) = decode(&bytes, DecoderTrap::Replace, UTF_8);

    match decoded_contents {
        Ok(contents) => contents.as_bytes().to_vec(),
        Err(e) => panic!("Failed to decode contents: {}", e)
    }
}

fn parse_wsdl(decoded_contents: &[u8]) -> ParseResult<Wsdl> {
    let parser = EventReader::new(decoded_contents);

    for element in parser {
        match element {
            Ok(XmlEvent::StartElement { ref name, .. }) => {
                if name.local_name == "definitions" {
                    println!("Found definitions element: {:?}", name);
                }
            },
            /*
            Ok(XmlEvent::EndElement { name }) => {
                println!("{:?}", name);
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
            */
            _ => {}
            
        }
    }

    Ok(Wsdl {

    })
}

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ParseError::Http(ref err) => write!(fmt, "{}", err),
            &ParseError::Io(ref err) => write!(fmt, "{}", err)
        }
    }
}
