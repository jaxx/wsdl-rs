extern crate wsdl;

use wsdl::Wsdl;

fn main() {
    let wsdl = Wsdl::fetch("http://10.1.210.116/etoimik.asmx?WSDL");
}