extern crate hyper;
extern crate xml;

mod http;

use xml::reader::{EventReader, XmlEvent};

pub struct Wsdl {
    
}

impl Wsdl {
    pub fn fetch(url: &str) -> http::Result<Wsdl> {
        let response = try!(http::get(url));

        println!("{:?}", response.body);

        let bytes = response.body.as_bytes();

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