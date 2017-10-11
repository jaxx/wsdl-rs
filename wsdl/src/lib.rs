#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate hyper;
extern crate xml;
extern crate encoding;
extern crate xsd;
extern crate tokio_core;
extern crate futures;

#[macro_use]
extern crate error_chain;

mod errors;
pub mod file;
pub mod http;
mod schema;
pub mod reader;

pub use self::reader::WsdlEvent;
