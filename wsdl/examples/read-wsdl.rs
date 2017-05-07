extern crate encoding;
extern crate xml;
extern crate wsdl;

use encoding::all::UTF_8;
use encoding::DecoderTrap;
use encoding::types::decode;

use std::fs::File;
use std::path::PathBuf;
use std::io::Write;
use std::env;

use wsdl::file;
use wsdl::http;

type XmlEventReader<R> = xml::reader::EventReader<R>;
type WsdlEventReader<R> = wsdl::reader::EventReader<R>;

fn decode_contents(bytes: &[u8]) -> Vec<u8> {
    let (decoded_contents, _) = decode(bytes, DecoderTrap::Replace, UTF_8);
    decoded_contents.unwrap().as_bytes().to_vec()
}

fn main() {
    let http_1 = http::get("http://www.webservicex.com/globalweather.asmx?WSDL").unwrap();
    let http_contents = decode_contents(&http_1);
    let http_xml = XmlEventReader::new(&http_contents[..]);
    let http_wsdl = WsdlEventReader::new(http_xml);

    for event in http_wsdl.into_iter() {
        println!("{:?}", event.unwrap());
    }

    let file_1 = file::load("wsdl/examples/files/etoimik.wsdl").unwrap();
    let file_contents = decode_contents(&file_1);
    let file_xml = XmlEventReader::new(&file_contents[..]);
    let file_wsdl = WsdlEventReader::new(file_xml);

    for event in file_wsdl.into_iter() {
        println!("{:?}", event.unwrap());
    }

    /*
    let wsdl = match Wsdl::load_from_url("http://www.webservicex.com/globalweather.asmx?WSDL") {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };

    let tmp_dir = env::temp_dir();
    print_wsdl(&wsdl, Some(tmp_dir.join("wsdl_globalweather.txt"))).expect("Error while printing WSDL.");

    let wsdl = match Wsdl::load_from_file("wsdl/examples/files/etoimik.wsdl") {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };

    print_wsdl(&wsdl, Some(tmp_dir.join("wsdl_etoimik.txt"))).expect("Error while printing WSDL.");
    */
}

/*
fn print_wsdl(wsdl: &Wsdl, file: Option<PathBuf>) -> Result<(), std::io::Error> {
    match file {
        None => println!("WSDL: {:#?}", wsdl),
        Some(f) => {
            let wsdl_str = format!("{:#?}", wsdl);
            File::create(f)?.write_all(wsdl_str.as_bytes())?;
        }
    }

    Ok(())
}
*/
