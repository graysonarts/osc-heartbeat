use super::Artwork;
use rosc::{self, OscPacket};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::sync::mpsc::Sender;

pub struct Subscriber {
    chan: Sender<Artwork>,
    port: u16,
}
impl Subscriber {
    pub fn new(chan: Sender<Artwork>, port: u16) -> Self {
        Self { chan, port }
    }

    pub fn run(&mut self) {
        let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), self.port);
        let sock = UdpSocket::bind(addr).unwrap();
        println!("Listening on {}", self.port);

        let mut buf = [0u8; rosc::decoder::MTU];

        loop {
            match sock.recv_from(&mut buf) {
                Ok((size, addr)) => {
                    let packet = rosc::decoder::decode(&buf[..size]).unwrap(); // TODO: Reject bad packets
                    self.handle_packet(packet, addr);
                }
                Err(e) => {
                    println!("Error receiving from socket: {}", e);
                    // Maybe exit app?
                }
            }
        }
    }

    fn handle_packet(&mut self, packet: OscPacket, addr: SocketAddr) {
        let address = match addr {
            SocketAddr::V4(a) => Some(a),
            SocketAddr::V6(_) => None,
        };

        if address.is_none() {
            return;
        }

        match packet {
            OscPacket::Message(m) => {
                if m.addr == "/hello" {
                    let a = "10.0.0.194:9000".parse();
                    let artwork = Artwork::from(m, a.unwrap());
                    self.chan.send(artwork).unwrap();
                } else {
                    println!("Unknown Message: {:?}", m);
                }
            }
            _ => {
                println!("Unhandled type {:?}", packet);
            }
        };
    }
}
