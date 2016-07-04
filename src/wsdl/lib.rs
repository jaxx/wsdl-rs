extern crate hyper;
extern crate xml;
extern crate encoding;

mod http;
mod file;

use std::io::Result as IoResult;
use http::Result as HttpResult;
use xml::reader::{EventReader, XmlEvent};

use encoding::EncoderTrap;

pub struct Wsdl {
    
}

impl Wsdl {
    pub fn load_from_url(url: &str) -> HttpResult<Wsdl> {
        let response = try!(http::get(url));

        let encoded_body = match response.encoding.encode(&response.body, EncoderTrap::Replace) {
            Ok(body) => body,
            Err(e) => panic!("Can't encode response body: {}", e)
        };

        let parser = EventReader::new(&encoded_body[..]);

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
        let content = try!(file::load(location));

        //let bytes = content.as_bytes();
        let bytes = if content.starts_with("\u{feff}") {
            content[3..].as_bytes()
        } else {
            content[..].as_bytes()
        };

        let parser = EventReader::new(bytes);

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