use futures::{Future, Stream};
use futures::future;
use hyper::{Client, Error};
use errors::*;
use tokio_core::reactor::Core;

pub fn get(url: &str) -> Result<Vec<u8>> {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    let uri = url.parse()?;

    let work = client.get(uri)
          .and_then(|res| {
              res.body().fold(Vec::new(), |mut v: Vec<u8>, chunk| {
                  v.extend(&chunk[..]);
                  future::ok::<_, Error>(v)
              })
          });

    Ok(core.run(work)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_url_test() {
        let result = get("http://httpbin.org/get");

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.len() > 0)
    }

    #[test]
    fn get_url_fail_test() {
        let result = get("http://www.sde.dd/");

        assert!(result.is_err());
    }
}
