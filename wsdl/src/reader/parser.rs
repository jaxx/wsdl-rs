use super::super::errors::*;
use super::events::WsdlEvent;
use std::io::Read;
use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

pub type XmlReader<R> = EventReader<R>;

pub struct Parser<R: Read> {
    reader: XmlReader<R>,
    depth: u32
}

impl<R: Read> Parser<R> {
    #[inline]
    pub fn new(reader: XmlReader<R>) -> Parser<R> {
        Parser { reader, depth: 0 }
    }

    pub fn next(&mut self) -> Result<WsdlEvent> {
        let ns_wsdl = Some("http://schemas.xmlsoap.org/wsdl/".to_string());
        loop {
            match (self.depth, self.reader.next()?) {
                (0, XmlEvent::StartElement { ref name, ref attributes, .. })
                    if name.namespace == ns_wsdl && name.local_name == "definitions" => {
                        self.depth += 1;
                        return Ok(WsdlEvent::StartDefinition {
                            name: find_attribute("name", attributes),
                            target_namespace: find_attribute("targetNamespace", attributes)
                        })
                    },
                (1, XmlEvent::EndElement { ref name, .. })
                    if name.namespace == ns_wsdl && name.local_name == "definitions" => {
                        self.depth -= 1;
                        return Ok(WsdlEvent::EndDefinition)
                    },
                (_, XmlEvent::StartElement { .. }) => self.depth += 1,
                (_, XmlEvent::EndElement { .. }) => self.depth -= 1,
                _ => {}
            }
        }
    }
}

fn find_attribute(name: &str, attributes: &[OwnedAttribute]) -> Option<String> {
    attributes
        .iter()
        .find(|a| a.name.namespace.is_none() && a.name.local_name == name)
        .map(|a| a.value.clone())
}
