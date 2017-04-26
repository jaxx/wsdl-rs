extern crate wsdl;

use wsdl::{
    NamedItem,
    Wsdl
};

fn print_name(named_item: &NamedItem) {
    println!("Name is: {}", named_item.get_name());
}

fn main() {
    let wsdl = match Wsdl::load_from_url("http://www.webservicex.com/globalweather.asmx?WSDL") {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };

    println!("WSDL: {:?}", wsdl);
    print_name(&wsdl.services[0]);

    let wsdl = match Wsdl::load_from_file("wsdl/examples/files/etoimik.wsdl") {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };

    println!("WSDL: {:?}", wsdl);

    print_name(&wsdl.services[0]);
}