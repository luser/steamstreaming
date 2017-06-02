extern crate openssl;
extern crate protobuf;
extern crate rand;
extern crate rustc_serialize;

mod proto;

use openssl::ssl::{SslMethod, SslConnectorBuilder, SSL_VERIFY_NONE};
use proto::steammessages_remoteclient_discovery::{CMsgRemoteClientBroadcastHeader, CMsgRemoteClientBroadcastDiscovery, CMsgRemoteClientBroadcastStatus, ERemoteClientBroadcastMsg};
use protobuf::{parse_from_bytes, CodedInputStream, CodedOutputStream, Message};
use rustc_serialize::hex::{FromHex, ToHex};
use std::io::{self, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream, ToSocketAddrs, UdpSocket};

const MAGIC: [u8; 8] = [0xFF, 0xFF, 0xFF, 0xFF, 0x21, 0x4C, 0x5F, 0xA0];
const DISCOVERY_PORT: u16 = 27036;
pub const CONTROL_PORT: u16 = 27036;
const PSK_IDENTITY: &'static str = "steam";

pub fn discover(client_id: u64, sequence_number: u32) -> io::Result<()> {
    let sock = UdpSocket::bind((Ipv4Addr::new(0,0,0,0), 0))?;
    sock.set_broadcast(true)?;
    let mut data: Vec<u8> = vec![];
    {
        let mut os = CodedOutputStream::vec(&mut data);
        let mut header = CMsgRemoteClientBroadcastHeader::new();
        header.set_client_id(client_id);
        header.set_msg_type(ERemoteClientBroadcastMsg::k_ERemoteClientBroadcastMsgDiscovery);
        os.write_raw_bytes(&MAGIC).unwrap();
        let len = header.compute_size();
        println!("header len: {}", len);
        os.write_raw_little_endian32(len).unwrap();
        header.write_to(&mut os)?;
        let mut body = CMsgRemoteClientBroadcastDiscovery::new();
        body.set_seq_num(sequence_number);
        let len = body.compute_size();
        println!("body len: {}", len);
        os.write_raw_little_endian32(len).unwrap();
        body.write_to(&mut os)?;
        os.flush().unwrap();
    }

    // Send a broadcast packet.
    let dest = SocketAddrV4::new(Ipv4Addr::new(255,255,255,255), DISCOVERY_PORT);
    let sent = sock.send_to(&data, dest)?;
    if sent != data.len() {
        panic!("Only sent {}/{} bytes!", sent, data.len());
    } else {
        println!("Sent {} bytes", data.len());
    }
    let mut buf = [0; 8192];
    while let Ok((len, addr)) = sock.recv_from(&mut buf) {
        println!("Got {} bytes from {:?}", len, addr);
        let mut is = CodedInputStream::from_bytes(&buf[..len]);
        let magic = is.read_raw_bytes(MAGIC.len() as u32).unwrap();
        assert_eq!(magic, MAGIC);
        let len = is.read_raw_little_endian32().unwrap();
        println!("header len: {}", len);
        let bytes = is.read_raw_bytes(len).unwrap();
        let header: CMsgRemoteClientBroadcastHeader = parse_from_bytes(&bytes).unwrap();
        println!("header: {:?}", header);
        let msg_type = header.get_msg_type();
        if msg_type == ERemoteClientBroadcastMsg::k_ERemoteClientBroadcastMsgStatus {
            let len = is.read_raw_little_endian32().unwrap();
            println!("body len: {}", len);
            let bytes = is.read_raw_bytes(len).unwrap();
            let body: CMsgRemoteClientBroadcastStatus = parse_from_bytes(&bytes).unwrap();
            println!("body: {:?}", body);
        } else {
            println!("Unexpected message type: {:?}", msg_type);
        }
    }
    Ok(())
}

pub fn connect<A>(host: A, psk: &str, _client_id: u64)
    where A: ToSocketAddrs,
{
    let psk = psk.from_hex().unwrap();
    assert_eq!(psk.len(), 32);
    let mut builder = SslConnectorBuilder::new(SslMethod::tls()).unwrap();
    {
        let ssl_builder = builder.builder_mut();
        ssl_builder.set_verify(SSL_VERIFY_NONE);
        ssl_builder.set_cipher_list("PSK-AES128-CBC-SHA").unwrap();
        ssl_builder.set_psk_callback(move |_ssl, _hint, mut identity, mut psk_buf| {
            identity.write_all(PSK_IDENTITY.as_bytes()).unwrap();
            identity.write(&[0]).unwrap();
            psk_buf.write_all(&psk).unwrap();
            Ok(psk.len())
        });
    }
    let connector = builder.build();
    let stream = TcpStream::connect(host).unwrap();
    let mut stream = connector.danger_connect_without_providing_domain_for_certificate_verification_and_server_name_indication(stream).unwrap();
    let mut buf = [0; 8192];
    let bytes_read = stream.read(&mut buf).unwrap();
    println!("read {} bytes:", bytes_read);
    println!("{}", &buf[..bytes_read].to_hex());
}

#[cfg(test)]
mod tests {
}
