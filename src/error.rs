use std::io::Error as IoError;

use hyper::Error as HyperError;
use xml::reader::Error as XmlError;

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    HttpError(HyperError),
    XmlError(XmlError)
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