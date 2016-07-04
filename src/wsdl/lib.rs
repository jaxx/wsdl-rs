extern crate hyper;
extern crate xml;
extern crate encoding;

mod http;
mod file;

use std::io::Result as IoResult;
use http::Result as HttpResult;
use xml::reader::{EventReader, XmlEvent};
use encoding::all::UTF_8;
use encoding::DecoderTrap;
use encoding::types::decode;

pub struct Wsdl {
    
}

impl Wsdl {
    pub fn load_from_url(url: &str) -> HttpResult<Wsdl> {
        let contents = try!(http::get(url));
        let decoded_contents = &decode_contents(contents)[..];
        let parser = EventReader::new(decoded_contents);

        for element in parser {
            match element {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    println!("{:?}", name);
                },
                Ok(XmlEvent::EndElement { name }) => {
                    println!("{:?}", name);
                },
                Err(e) => {
                    panic!("Error: {}", e);
                }
                _ => {}
            }
        }

        Ok(Wsdl {

        })
    }

    pub fn load_from_file(location: &str) -> IoResult<Wsdl> {
        let contents = try!(file::load(location));
        let decoded_contents = &decode_contents(contents)[..];
        let parser = EventReader::new(decoded_contents);

        for element in parser {
            match element {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    println!("{:?}", name);
                },
                Ok(XmlEvent::EndElement { name }) => {
                    println!("{:?}", name);
                },
                Err(e) => {
                    panic!("Error: {}", e);
                }
                _ => {}
            }
        }

        Ok(Wsdl {
            
        })
    }
}

fn decode_contents(bytes: Vec<u8>) -> Vec<u8> {
    let (decoded_contents, _) = decode(&bytes, DecoderTrap::Replace, UTF_8);
    

    match decoded_contents {
        Ok(contents) => contents.as_bytes().to_vec(),
        Err(e) => panic!("Failed to decode contents: {}", e)
    }
}