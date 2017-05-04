extern crate wsdl;

use std::fs::File;
use std::path::PathBuf;
use std::io::Write;
use std::env;

use wsdl::Wsdl;

fn main() {
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
}

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
