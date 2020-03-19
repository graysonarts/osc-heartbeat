#[macro_use]
extern crate clap;

use clap::Arg;
use rosc::{OscMessage, OscPacket, OscType};
use std::net::{SocketAddrV4, UdpSocket};

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name("address").required(true))
        .get_matches();

    let arg = matches.value_of("address").unwrap();
    let addr: SocketAddrV4 = arg.parse().unwrap();
    let host_addr: SocketAddrV4 = "0.0.0.0:0".parse().unwrap();

    let sock = UdpSocket::bind(host_addr).unwrap();

    let msg_buf = rosc::encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/hello".to_string(),
        args: vec![
            OscType::String("TestArtwork".to_owned()),
            OscType::Int(9000),
        ],
    }))
    .unwrap();

    sock.send_to(&msg_buf, addr).unwrap();
}
