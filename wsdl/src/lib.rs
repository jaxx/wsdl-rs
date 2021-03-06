#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate hyper;
extern crate xml;
extern crate encoding;
extern crate xsd;

#[macro_use]
extern crate error_chain;

mod errors;
mod file;
mod http;
mod schema;

pub use schema::{
    Documented,
    NamedItem,
    Wsdl,
    WsdlBinding,
    WsdlOperationBinding,
    WsdlInputBinding,
    WsdlOutputBinding,
    WsdlFaultBinding,
    WsdlPort,
    WsdlService
};
