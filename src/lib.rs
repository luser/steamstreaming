extern crate openssl;
extern crate rand;
extern crate rustc_serialize;

use openssl::ssl::{SslMethod, SslConnectorBuilder};
use rustc_serialize::hex::FromHex;
use std::net::{TcpStream, ToSocketAddrs};

const PSK_IDENTITY: &'static [u8] = b"steam";

pub fn connect<A>(hostname: A, psk: &str, client_id: u64)
    where A: ToSocketAddrs,
{
    let addrs = hostname.to_socket_addrs();
    let psk = psk.from_hex().unwrap();
    assert_eq!(psk.len(), 32);
    let connector = SslConnectorBuilder::new(SslMethod::tls()).unwrap().build();
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
