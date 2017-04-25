use std::io::Read;

use hyper::Client;
use hyper::error::Error;

pub fn get(url: &str) -> Result<Vec<u8>, Error> {
    let client = Client::new();
    let mut bytes = Vec::new();

    client.get(url)
          .send()?
          .read_to_end(&mut bytes)?;

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test() {
        let result = get("http://httpbin.org/get");

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.len() > 0)
    }

    #[test]
    #[should_panic]
    fn get_fail_test() {
        get("http://www.sde.dd/").unwrap();
    }
}