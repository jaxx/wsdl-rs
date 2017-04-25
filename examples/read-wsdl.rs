extern crate wsdl;

use wsdl::Wsdl;

fn main() {
    let wsdl = match Wsdl::load_from_url("http://www.webservicex.com/globalweather.asmx?WSDL") {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };

    println!("WSDL: {:?}", wsdl);

    let wsdl = match Wsdl::load_from_file("examples/files/etoimik.wsdl") {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };

    println!("WSDL: {:?}", wsdl);
}