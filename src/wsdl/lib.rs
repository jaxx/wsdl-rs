extern crate hyper;
extern crate xml;
extern crate encoding;

mod http;
mod file;

use std::error::Error;

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

impl Wsdl {
    pub fn load_from_url(url: &str) -> Result<Wsdl, Box<Error>> {
        let contents = http::get(url)?;
        let decoded_contents = decode_contents(contents);
        parse_wsdl(&decoded_contents[..])
    }

    pub fn load_from_file(location: &str) -> Result<Wsdl, Box<Error>> {
        let contents = file::load(location)?;
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

fn parse_wsdl(decoded_contents: &[u8]) -> Result<Wsdl, Box<Error>> {
    let parser = EventReader::new(decoded_contents);
    let mut iter = parser.into_iter();

    loop {
        match iter.next() {
            Some(e) => {
                let event = e.unwrap();

                match event {
                    XmlEvent::StartElement { ref name, .. } => {
                        println!("start element: {}", name.local_name);
                    },
                    _ => continue
                }
            },
            None => break
        }
    }

    Ok(Wsdl {

    })
}