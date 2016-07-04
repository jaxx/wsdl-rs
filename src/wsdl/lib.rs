extern crate hyper;
extern crate xml;
extern crate encoding;

mod http;
mod file;

use std::io::Result as IoResult;
use http::Result as HttpResult;
use xml::reader::{EventReader, XmlEvent};

pub struct Wsdl {
    
}

impl Wsdl {
    pub fn load_from_url(url: &str) -> HttpResult<Wsdl> {
        let response = try!(http::get(url));
        let parser = EventReader::new(response.as_bytes());

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
        let parser = EventReader::new(content.as_bytes());

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