use error::Error;

use http;
use file;

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

pub trait Documented {

}

macro_rules! documented {
    ($type:ty) => {
        impl Documented for $type {
        }
    }
}

pub trait NamedItem {
    fn get_name(&self) -> &str;
}

macro_rules! named_item {
    ($type:ty) => {
        impl NamedItem for $type {
            fn get_name(&self) -> &str {
                self.name.as_str()
            }
        }
    }
}

#[derive(Debug)]
pub struct Wsdl {
    pub target_namespace: Option<String>,
    pub services: Vec<WsdlService>
}

documented!(Wsdl);

#[derive(Debug)]
pub struct WsdlInputBinding {

}

documented!(WsdlInputBinding);

#[derive(Debug)]
pub struct WsdlOutputBinding {

}

documented!(WsdlOutputBinding);

#[derive(Debug)]
pub struct WsdlFaultBinding {
    pub name: String
}

documented!(WsdlFaultBinding);
named_item!(WsdlFaultBinding);

#[derive(Debug)]
pub struct WsdlOperationBinding {
    pub name: String,
    pub input: Option<WsdlInputBinding>,
    pub output: Option<WsdlOutputBinding>,
    pub fault: Option<WsdlFaultBinding>
}

documented!(WsdlOperationBinding);
named_item!(WsdlOperationBinding);

#[derive(Debug)]
pub struct WsdlBinding {
    pub name: String,
    pub port_type: OwnedName,
    pub operations: Vec<WsdlOperationBinding>
}

documented!(WsdlBinding);
named_item!(WsdlBinding);

#[derive(Debug)]
pub struct WsdlPort {
    pub name: String,
    pub binding: OwnedName
}

documented!(WsdlPort);
named_item!(WsdlPort);

#[derive(Debug)]
pub struct WsdlService {
    pub name: String,
    pub ports: Vec<WsdlPort>
}

documented!(WsdlService);
named_item!(WsdlService);

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

    fn read(attributes: &[OwnedAttribute], mut iter: &mut Events<&[u8]>) -> Result<Wsdl, Error> {
        let mut target_namespace: Option<String> = None;

        for attr in attributes {
            if attr.name.namespace == None && attr.name.local_name == "targetNamespace" {
                target_namespace = Some(attr.value.clone());
            }
        }

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
            services,
            target_namespace
        })
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
                        ports
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
            binding
        })
    }
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
            XmlEvent::StartElement { ref name, ref attributes, .. } if name.namespace == wsdl_ns && name.local_name == "definitions" => {
                return Wsdl::read(attributes, &mut iter);
            },
            e => println!("Unexpected element in WSDL document: {:?}", e)
        }
    }

    Err(Error::WsdlError(String::from("Required `definitions` element is missing from WSDL document.")))
}
