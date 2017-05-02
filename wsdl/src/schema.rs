use errors::*;

use http;
use file;

use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::{EventReader, Events, XmlEvent };

use encoding::all::UTF_8;
use encoding::DecoderTrap;
use encoding::types::decode;

const NS_WSDL: &'static str = "http://schemas.xmlsoap.org/wsdl/";

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
    pub bindings: Vec<WsdlBinding>,
    pub messages: Vec<WsdlMessage>
}

impl_documented!(Wsdl);

#[derive(Debug)]
pub struct WsdlService {
    pub name: String,
    pub ports: Vec<WsdlPort>
}

impl_documented!(WsdlService);
impl_named_item!(WsdlService);

#[derive(Debug)]
pub struct WsdlBinding {
    pub name: String,
    pub port_type: OwnedName,
    pub operations: Vec<WsdlOperationBinding>
}

impl_documented!(WsdlBinding);
impl_named_item!(WsdlBinding);

#[derive(Debug)]
pub struct WsdlMessage {
    pub name: String,
    pub parts: Vec<WsdlMessagePart>
}

impl_documented!(WsdlMessage);
impl_named_item!(WsdlMessage);

#[derive(Debug)]
pub struct WsdlPort {
    pub name: String,
    pub binding: OwnedName
}

impl_documented!(WsdlPort);
impl_named_item!(WsdlPort);

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
pub struct WsdlMessagePart {
    pub name: String,
    pub element: Option<OwnedName>,
    pub part_type: Option<OwnedName>
}

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

impl Wsdl {
    pub fn load_from_url(url: &str) -> Result<Wsdl> {
        let contents = http::get(url)?;
        let decoded_contents = decode_contents(&contents)?;
        parse_wsdl(&decoded_contents[..])
    }

    pub fn load_from_file(location: &str) -> Result<Wsdl> {
        let contents = file::load(location)?;
        let decoded_contents = decode_contents(&contents)?;
        parse_wsdl(&decoded_contents[..])
    }

    fn read(attributes: &[OwnedAttribute], mut iter: &mut Events<&[u8]>) -> Result<Wsdl> {
        let ns_wsdl = Some(NS_WSDL.to_string());
        let target_namespace = find_attribute("targetNamespace", attributes);

        let mut services = Vec::new();
        let mut bindings = Vec::new();
        let mut messages = Vec::new();

        while let Some(v) = iter.next() {
            match v? {
                XmlEvent::StartElement { ref name, ref attributes, .. }
                    if name.namespace == ns_wsdl && name.local_name == "service" => {
                        services.push(WsdlService::read(attributes, &mut iter)?);
                },
                XmlEvent::StartElement { ref name, ref attributes, ref namespace }
                    if name.namespace == ns_wsdl && name.local_name == "binding" => {
                        bindings.push(WsdlBinding::read(attributes, namespace, &mut iter)?)
                },
                XmlEvent::StartElement { ref name, ref attributes, .. }
                    if name.namespace == ns_wsdl && name.local_name == "message" => {
                        messages.push(WsdlMessage::read(attributes, &mut iter)?);
                },
                _ => continue
            }
        }

        Ok(Wsdl {
            target_namespace,
            services,
            bindings,
            messages
        })
    }
}

impl WsdlService {
    fn read(attributes: &[OwnedAttribute], iter: &mut Events<&[u8]>) -> Result<WsdlService> {
        let ns_wsdl = Some(NS_WSDL.to_string());
        let service_name = find_attribute("name", attributes);

        let mut ports = Vec::new();

        for event in iter {
            match event? {
                XmlEvent::StartElement { ref name, ref attributes, ref namespace }
                    if name.local_name == "port" => {
                        ports.push(WsdlPort::read(attributes, namespace)?);
                },
                XmlEvent::EndElement { ref name, .. }
                    if name.namespace == ns_wsdl && name.local_name == "service" => {
                        return Ok(WsdlService {
                            name: service_name.ok_or_else(|| ErrorKind::MandatoryAttribute("name".to_string(), "wsdl:service".to_string()))?,
                            ports
                        });
                },
                _ => continue
            }
        }
  
        Err(ErrorKind::InvalidElement("wsdl:service".to_string()).into())
    }
}

impl WsdlBinding {
    fn read(attributes: &[OwnedAttribute], namespace: &Namespace, iter: &mut Events<&[u8]>) -> Result<WsdlBinding> {
        let ns_wsdl = Some(NS_WSDL.to_string());
        let mut binding_name = None;
        let mut port_type = None;

        for attr in attributes {
            if attr.name.namespace.is_none() {
                if attr.name.local_name == "name" {
                    binding_name = Some(attr.value.clone());
                } else if attr.name.local_name == "type" {
                    port_type = Some(attr.value.clone());
                }
            }
        }

        let mut port_type: OwnedName = port_type.ok_or_else(|| ErrorKind::MandatoryAttribute("type".to_string(), "wsdl:binding".to_string()))?.parse().unwrap();

        if let Some(ref pfx) = port_type.prefix {
            port_type.namespace = namespace.get(pfx).map(|x| x.to_string());
        }

        let mut operations = Vec::new();

        for event in iter {
            match event? {
                XmlEvent::StartElement { ref name, ref attributes, ref namespace }
                    if name.namespace == ns_wsdl && name.local_name == "operation" => {
                        operations.push(WsdlOperationBinding::read(attributes, namespace)?);
                },
                XmlEvent::EndElement { ref name, .. }
                    if name.namespace == ns_wsdl && name.local_name == "binding" => {; 
                        return Ok(WsdlBinding {
                            name: binding_name.ok_or_else(|| ErrorKind::MandatoryAttribute("name".to_string(), "wsdl:binding".to_string()))?,
                            port_type,
                            operations
                        });
                },
                _ => continue
            }
        }
   
        Err(ErrorKind::InvalidElement("wsdl:binding".to_string()).into())
    }
}

impl WsdlMessage {
    fn read(attributes: &[OwnedAttribute], iter: &mut Events<&[u8]>) -> Result<WsdlMessage> {
        let ns_wsdl = Some(NS_WSDL.to_string());
        let message_name = find_attribute("name", attributes);

        let mut parts = Vec::new();
        
        for event in iter {
            match event? {
                XmlEvent::StartElement { ref name, ref attributes, .. }
                    if name.namespace == ns_wsdl && name.local_name == "part" => {
                        parts.push(WsdlMessagePart::read(attributes)?);
                },
                XmlEvent::EndElement { ref name, .. }
                    if name.namespace == ns_wsdl && name.local_name == "message" => {
                        return Ok(WsdlMessage {
                            name: message_name.ok_or_else(|| ErrorKind::MandatoryAttribute("name".to_string(), "wsdl:message".to_string()))?,
                            parts
                        });
                },
                _ => continue
            }
        }

        Err(ErrorKind::InvalidElement("wsdl:message".to_string()).into())
    }
}

impl WsdlPort {
    fn read(attributes: &[OwnedAttribute], namespace: &Namespace) -> Result<WsdlPort> {
        let mut name = None;
        let mut binding = None;
   
        for attr in attributes {
            if attr.name.namespace.is_none() {
                if attr.name.local_name == "name" {
                    name = Some(attr.value.clone());
                } else if attr.name.local_name == "binding" {
                    binding = Some(attr.value.clone());
                }
            }
        }
        let mut binding: OwnedName = binding.ok_or_else(|| ErrorKind::MandatoryAttribute("binding".to_string(), "wsdl:port".to_string()))?.parse().unwrap();
        if let Some(ref pfx) = binding.prefix {
            binding.namespace = namespace.get(pfx).map(|x| x.to_string());
        }

        Ok(WsdlPort {
            name: name.ok_or_else(|| ErrorKind::MandatoryAttribute("name".to_string(), "wsdl:port".to_string()))?,
            binding
        })
    }
}

impl WsdlOperationBinding {
    fn read(attributes: &[OwnedAttribute], namespace: &Namespace) -> Result<WsdlOperationBinding> {
        let name = find_attribute("name", attributes);
 
        Ok(WsdlOperationBinding {
            name: name.ok_or_else(|| ErrorKind::MandatoryAttribute("name".to_string(), "wsdl:operation".to_string()))?,
            input: None,
            output: None,
            fault: None
        })
    }
}

impl WsdlMessagePart {
    fn read(attributes: &[OwnedAttribute]) -> Result<WsdlMessagePart> {
        let part_name = find_attribute("name", attributes);

        Ok(WsdlMessagePart {
            name: part_name.ok_or_else(|| ErrorKind::MandatoryAttribute("name".to_string(), "wsdl:part".to_string()))?,
            element: None,
            part_type: None
        })
    }
}

fn decode_contents(bytes: &[u8]) -> Result<Vec<u8>> {
    let (decoded_contents, _) = decode(bytes, DecoderTrap::Replace, UTF_8);
    Ok(decoded_contents?.as_bytes().to_vec())
}

fn parse_wsdl(decoded_contents: &[u8]) -> Result<Wsdl> {
    let ns_wsdl = Some(NS_WSDL.to_string());
    let parser = EventReader::new(decoded_contents);
    let mut iter = parser.into_iter();

    while let Some(v) = iter.next() {
        match v? {
            XmlEvent::StartDocument { .. } => continue,
            XmlEvent::EndDocument => break,
            XmlEvent::StartElement { ref name, ref attributes, .. }
                if name.namespace == ns_wsdl && name.local_name == "definitions" => {
                    return Wsdl::read(attributes, &mut iter);
            },
            e => println!("Unexpected element in WSDL document: {:?}", e)
        }
    }

    Err(ErrorKind::MissingElement("definitions".to_string()).into())
}

fn find_attribute(name: &str, attributes: &[OwnedAttribute]) -> Option<String> {
    attributes
        .iter()
        .find(|a| a.name.namespace.is_none() && a.name.local_name == name)
        .map(|a| a.value.clone())
}