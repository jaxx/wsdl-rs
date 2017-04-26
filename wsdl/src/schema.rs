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

macro_rules! impl_documented {
    ($type:ty) => {
        impl Documented for $type {
        }
    }
}

pub trait NamedItem {
    fn get_name(&self) -> &str;
}

macro_rules! impl_named_item {
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
    pub services: Vec<WsdlService>,
    pub bindings: Vec<WsdlBinding>
}

impl_documented!(Wsdl);

#[derive(Debug)]
pub struct WsdlInputBinding {

}

impl_documented!(WsdlInputBinding);

#[derive(Debug)]
pub struct WsdlOutputBinding {

}

impl_documented!(WsdlOutputBinding);

#[derive(Debug)]
pub struct WsdlFaultBinding {
    pub name: String
}

impl_documented!(WsdlFaultBinding);
impl_named_item!(WsdlFaultBinding);

#[derive(Debug)]
pub struct WsdlOperationBinding {
    pub name: String,
    pub input: Option<WsdlInputBinding>,
    pub output: Option<WsdlOutputBinding>,
    pub fault: Option<WsdlFaultBinding>
}

impl_documented!(WsdlOperationBinding);
impl_named_item!(WsdlOperationBinding);

#[derive(Debug)]
pub struct WsdlBinding {
    pub name: String,
    pub port_type: OwnedName,
    pub operations: Vec<WsdlOperationBinding>
}

impl_documented!(WsdlBinding);
impl_named_item!(WsdlBinding);

#[derive(Debug)]
pub struct WsdlPort {
    pub name: String,
    pub binding: OwnedName
}

impl_documented!(WsdlPort);
impl_named_item!(WsdlPort);

#[derive(Debug)]
pub struct WsdlService {
    pub name: String,
    pub ports: Vec<WsdlPort>
}

impl_documented!(WsdlService);
impl_named_item!(WsdlService);

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
        let target_namespace = attributes
            .iter()
            .find(|a| a.name.namespace.is_none() && a.name.local_name == "targetNamespace")
            .map(|a| a.value.clone());

        let wsdl_ns = Some(NAMESPACE_WSDL.to_string());
   
        let mut services: Vec<WsdlService> = vec![];
        let mut bindings: Vec<WsdlBinding> = vec![];

        while let Some(v) = iter.next() {
            match v? {
                XmlEvent::StartElement { ref name, ref attributes, .. }
                    if name.namespace == wsdl_ns && name.local_name == "service" => {
                        services.push(WsdlService::read(attributes, &mut iter)?);
                },
                XmlEvent::StartElement { ref name, ref attributes, ref namespace }
                    if name.namespace == wsdl_ns && name.local_name == "binding" => {
                        bindings.push(WsdlBinding::read(attributes, namespace, &mut iter)?)
                },
                _ => continue
            }
        }

        Ok(Wsdl {
            services,
            target_namespace,
            bindings
        })
    }
}

impl WsdlService {
    fn read(attributes: &[OwnedAttribute], iter: &mut Events<&[u8]>) -> Result<WsdlService, Error> {
        let name = attributes
            .iter()
            .find(|a| a.name.namespace.is_none() && a.name.local_name == "name")
            .map(|a| a.value.clone());

        let mut ports = vec![];

        for event in iter {
            match event? {
                XmlEvent::StartElement { ref name, ref attributes, ref namespace }
                    if name.local_name == "port" => {
                        ports.push(WsdlPort::read(attributes, namespace)?);
                },
                XmlEvent::EndElement { .. } => {
                    return Ok(WsdlService {
                        name: name.ok_or_else(|| Error::WsdlError("Attribute `name` is mandatory for `wsdl:service` element.".to_string()))?,
                        ports
                    });
                },
                _ => continue
            }
        }
  
        Err(Error::WsdlError("Invalid `wsdl:service` element.".to_string()))
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
        let mut binding: OwnedName = binding.ok_or_else(|| Error::WsdlError("Attribute `binding` is mandatory for `wsdl:port` element.".to_string()))?.parse().unwrap();
        if let Some(ref pfx) = binding.prefix {
            binding.namespace = namespace.get(pfx).map(|x| x.to_string());
        }

        Ok(WsdlPort {
            name: name.ok_or_else(|| Error::WsdlError("Attribute `name` is mandatory for `wsdl:port` element.".to_string()))?,
            binding
        })
    }
}

impl WsdlBinding {
    fn read(attributes: &[OwnedAttribute], namespace: &Namespace, iter: &mut Events<&[u8]>) -> Result<WsdlBinding, Error> {
        let mut binding_name: Option<String> = None;
        let mut port_type: Option<String> = None;

        let wsdl_ns = Some(NAMESPACE_WSDL.to_string());

        for attr in attributes {
            if attr.name.namespace.is_none() {
                if attr.name.local_name == "name" {
                    binding_name = Some(attr.value.clone());
                } else if attr.name.local_name == "type" {
                    port_type = Some(attr.value.clone());
                }
            }
        }

        let mut port_type: OwnedName = port_type.ok_or_else(|| Error::WsdlError("Attribute `type` is mandatory for `wsdl:binding` element.".to_string()))?.parse().unwrap();

        if let Some(ref pfx) = port_type.prefix {
            port_type.namespace = namespace.get(pfx).map(|x| x.to_string());
        }

        let mut operations: Vec<WsdlOperationBinding> = vec![];

        for event in iter {
            match event? {
                XmlEvent::StartElement { ref name, ref attributes, ref namespace }
                    if name.local_name == "operation" => {
                        operations.push(WsdlOperationBinding::read(attributes, namespace)?);
                },
                XmlEvent::EndElement { ref name, .. }
                    if name.local_name == "binding" && name.namespace == wsdl_ns => {; 
                        return Ok(WsdlBinding {
                            name: binding_name.ok_or_else(|| Error::WsdlError("Attribute `name` is mandatory for `wsdl:binding` element.".to_string()))?,
                            port_type,
                            operations
                        });
                },
                _ => continue
            }
        }
   
        Err(Error::WsdlError("Invalid `wsdl:binding` element.".to_string()))
    }
}

impl WsdlOperationBinding {
    fn read(attributes: &[OwnedAttribute], namespace: &Namespace) -> Result<WsdlOperationBinding, Error> {
        let name = attributes
            .iter()
            .find(|a| a.name.namespace.is_none() && a.name.local_name == "name")
            .map(|a| a.value.clone());
 
        Ok(WsdlOperationBinding {
            name: name.ok_or_else(|| Error::WsdlError("Attribute `name` is mandatory for `wsdl:operation` element.".to_string()))?,
            input: None,
            output: None,
            fault: None
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
            XmlEvent::StartElement { ref name, ref attributes, .. }
                if name.namespace == wsdl_ns && name.local_name == "definitions" => {
                    return Wsdl::read(attributes, &mut iter);
            },
            e => println!("Unexpected element in WSDL document: {:?}", e)
        }
    }

    Err(Error::WsdlError("Required `definitions` element is missing from WSDL document.".to_string()))
}
