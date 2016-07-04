extern crate wsdl;

use wsdl::Wsdl;

fn main() {
    let wsdl = match Wsdl::load_from_url("http://www.webservicex.com/globalweather.asmx?WSDL") {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };

    let wsdl = match Wsdl::load_from_file("/home/jaxx/Downloads/etoimik.asmx") {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };
}