use errors::*;

use std::io::Read;

mod events;
mod parser;

pub use self::events::WsdlEvent;
pub use self::parser::{Parser, XmlReader};

pub struct EventReader<R: Read> {
    parser: Parser<R>
}

pub struct Events<R: Read> {
    reader: EventReader<R>,
    finished: bool
}

impl<R: Read> EventReader<R> {
    #[inline]
    pub fn new(reader: XmlReader<R>) -> EventReader<R> {
        EventReader { parser: Parser::new(reader) }
    }

    #[inline]
    pub fn next(&mut self) -> Result<WsdlEvent> {
        self.parser.next()
    }
}

impl<R: Read> IntoIterator for EventReader<R> {
    type Item = Result<WsdlEvent>;
    type IntoIter = Events<R>;

    fn into_iter(self) -> Events<R> {
        Events { reader: self, finished: false }
    }
}

impl<R: Read> Iterator for Events<R> {
    type Item = Result<WsdlEvent>;

    #[inline]
    fn next(&mut self) -> Option<Result<WsdlEvent>> {
        if self.finished { None }
        else {
            let event = self.reader.next();
            match event {
                Ok(WsdlEvent::EndDefinition) | Err(_) => self.finished = true,
                _ => {}
            }
            Some(event)
        }
    }
}
