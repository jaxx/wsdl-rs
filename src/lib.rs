#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate hyper;
extern crate xml;
extern crate encoding;

mod http;
mod file;

use std::error::Error;

use xml::reader::{EventReader, Events, XmlEvent};
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

struct WsdlPort {

}

struct WsdlService {
    name: String,
    ports: Vec<WsdlPort>
}

impl Wsdl {
    pub fn load_from_url(url: &str) -> Result<Wsdl, Box<Error>> {
        let contents = http::get(url)?;
        let decoded_contents = decode_contents(&contents);
        parse_wsdl(&decoded_contents[..])
    }

    pub fn load_from_file(location: &str) -> Result<Wsdl, Box<Error>> {
        let contents = file::load(location)?;
        let decoded_contents = decode_contents(&contents);
        parse_wsdl(&decoded_contents[..])
    }
}

fn decode_contents(bytes: &[u8]) -> Vec<u8> {
    let (decoded_contents, _) = decode(bytes, DecoderTrap::Replace, UTF_8);

    match decoded_contents {
        Ok(contents) => contents.as_bytes().to_vec(),
        Err(e) => panic!("Failed to decode contents: {}", e)
    }
}

fn parse_wsdl(decoded_contents: &[u8]) -> Result<Wsdl, Box<Error>> {
    let parser = EventReader::new(decoded_contents);
    let mut iter = parser.into_iter();

    let wsdl_ns = Some(NAMESPACE_WSDL.to_string());

    while let Some(v) = iter.next() {
        match v? {
            XmlEvent::EndDocument => break,
            XmlEvent::StartElement { ref name, .. } if name.namespace == wsdl_ns && name.local_name == "definitions" => {
                parse_definitions(&mut iter)?;
            },
            e => println!("Unexpected element in WSDL document: {:?}", e)
        }
    }

    Ok(Wsdl {

    })
}

fn parse_definitions(iter: &mut Events<&[u8]>) -> Result<(), Box<Error>> {
    let mut depth = 0;

    while let Some(v) = iter.next() {
        match v? {
            XmlEvent::StartElement { ref name, .. } => {
                depth += 1;
                println!("[def] start element: {}", name.local_name);
            },
            XmlEvent::EndElement { .. } => {
                depth -= 1;
                if depth < 0 {
                    break;
                }
            },
            event => println!("[def] event: {:?}", event)
        }
    }

    Ok(())
}
