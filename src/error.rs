use std::io::Error as IoError;
use std::fmt::{Display, Formatter, Result};

use hyper::Error as HyperError;
use xml::reader::Error as XmlError;

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    HttpError(HyperError),
    XmlError(XmlError)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Error::IoError(ref e) => write!(f, "IO error: {}", e),
            Error::HttpError(ref e) => write!(f, "HTTP error: {}", e),
            Error::XmlError(ref e) => write!(f, "XML error: {}", e)
        }
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Error::IoError(error)
    }
}

impl From<HyperError> for Error {
    fn from(error: HyperError) -> Error {
        Error::HttpError(error)
    }
}

impl From<XmlError> for Error {
    fn from(error: XmlError) -> Error {
        Error::XmlError(error)
    }
}