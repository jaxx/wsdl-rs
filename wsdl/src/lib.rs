#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate hyper;
extern crate xml;
extern crate encoding;
extern crate xsd;

mod http;
mod file;
mod error;

use error::Error;

use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::namespace::Namespace;

use xml::reader::{
    EventReader,
    Events,
    XmlEvent
};

use encoding::all::UTF_8;
use encoding::DecoderTrap;
use encoding::types::decode;

const NAMESPACE_WSDL: &'static str = "http://schemas.xmlsoap.org/wsdl/";

#[derive(Debug)]
pub struct Wsdl {
    services: Vec<WsdlService>
}

#[derive(Debug)]
struct WsdlPort {
    name: String,
    binding: OwnedName
}

#[derive(Debug)]
struct WsdlService {
    name: String,
    ports: Vec<WsdlPort>
}

trait Documented {

}

impl Wsdl {
    pub fn load_from_url(url: &str) -> Result<Wsdl, Error> {
        let contents = http::get(url)?;
        let decoded_contents = decode_contents(&contents)?;
        parse_wsdl(&decoded_contents[..])
    }

    pub fn load_from_file(location: &str) -> Result<Wsdl, Error> {
        let contents = file::load(location)?;
        let decoded_contents = decode_contents(&contents)?;
        parse_wsdl(&decoded_contents[..])
    }
}

impl WsdlService {
    fn read(attributes: &[OwnedAttribute], iter: &mut Events<&[u8]>) -> Result<WsdlService, Error> {
        let mut name: Option<String> = None;
        for attr in attributes {
            if attr.name.namespace == None && attr.name.local_name == "name" {
                name = Some(attr.value.clone());
            }
        }
        let mut ports = vec![];
        for event in iter {
            match event? {
                XmlEvent::StartElement { ref name, ref attributes, ref namespace } if name.local_name == "port" => {
                    ports.push(WsdlPort::read(attributes, namespace)?);
                }
                XmlEvent::EndElement { .. } => {
                    return Ok(WsdlService {
                        name: name.ok_or_else(|| Error::WsdlError(String::from("Attribute `name` is mandatory for `wsdl:service` element.")))?,
                        ports: ports
                    });
                },
                _ => continue
            }
        }
        Err(Error::WsdlError(String::from("Invalid `wsdl:service` element.")))
    }
}

impl WsdlPort {
    fn read(attributes: &[OwnedAttribute], namespace: &Namespace) -> Result<WsdlPort, Error> {
        let mut name: Option<String> = None;
        let mut binding: Option<String> = None;
        for attr in attributes {
            if attr.name.namespace.is_none() {
                if attr.name.local_name == "name" {
                    name = Some(attr.value.clone());
                } else if attr.name.local_name == "binding" {
                    binding = Some(attr.value.clone());
                }
            }
        }
        let mut binding: OwnedName = binding.ok_or_else(|| Error::WsdlError(String::from("Attribute `binding` is mandatory for `wsdl:port` element.")))?.parse().unwrap();
        if let Some(ref pfx) = binding.prefix {
            binding.namespace = namespace.get(pfx).map(|x| x.to_string());
        }
        Ok(WsdlPort {
            name: name.ok_or_else(|| Error::WsdlError(String::from("Attribute `name` is mandatory for `wsdl:port` element.")))?,
            binding: binding
        })
    }
}

impl Documented for WsdlService {

}

impl Documented for WsdlPort {

}

fn decode_contents(bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let (decoded_contents, _) = decode(bytes, DecoderTrap::Replace, UTF_8);
    Ok(decoded_contents?.as_bytes().to_vec())
}

fn parse_wsdl(decoded_contents: &[u8]) -> Result<Wsdl, Error> {
    let parser = EventReader::new(decoded_contents);
    let mut iter = parser.into_iter();

    let wsdl_ns = Some(NAMESPACE_WSDL.to_string());

    while let Some(v) = iter.next() {
        match v? {
            XmlEvent::StartDocument { .. } => continue,
            XmlEvent::EndDocument => break,
            XmlEvent::StartElement { ref name, .. } if name.namespace == wsdl_ns && name.local_name == "definitions" => {
                return parse_definitions(&mut iter);
            },
            e => println!("Unexpected element in WSDL document: {:?}", e)
        }
    }

    Err(Error::WsdlError(String::from("Required `definitions` element is missing from WSDL document.")))
}

fn parse_definitions(mut iter: &mut Events<&[u8]>) -> Result<Wsdl, Error> {
    let mut depth = 0;

    let ns = Some(String::from(NAMESPACE_WSDL));
    let mut services: Vec<WsdlService> = vec![];

    while let Some(v) = iter.next() {
        match v? {
            XmlEvent::StartElement { ref name, ref attributes, .. } if depth == 0 && name.namespace == ns && name.local_name == "service" => {
                services.push(WsdlService::read(attributes, &mut iter)?);
            },
            XmlEvent::StartElement { .. } => {
                depth += 1;
            },
            XmlEvent::EndElement { .. } => {
                depth -= 1;
                if depth < 0 {
                    break;
                }
            },
            _ => {}
        }
    }

    Ok(Wsdl {
        services: services
    })
}
