use openssl;
use protobuf;
use rustc_serialize;
use std::net;
use std::io;

error_chain! {
    foreign_links {
        FromHex(rustc_serialize::hex::FromHexError);
        Io(io::Error);
        Ssl(openssl::error::ErrorStack);
        SslHandshakeTcp(openssl::ssl::HandshakeError<net::TcpStream>);
        Protobuf(protobuf::ProtobufError);
    }
}
