extern crate hyper;

pub mod http;

pub struct Wsdl {
    
}


impl Wsdl {
    pub fn fetch(url: &str) -> http::Result<Wsdl> {
        let response = try!(http::get(url));

        println!("{:?}", response);

        Ok(Wsdl {

        })
    }
}