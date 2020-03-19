#[macro_use]
extern crate clap;

mod subscriber;

use clap::Arg;
use rosc::{self, OscMessage};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use subscriber::Subscriber;

#[derive(Debug)]
pub struct Artwork {
    address: Ipv4Addr,
    name: String,
    port: u16,
}

impl Artwork {
    fn from(message: OscMessage, addr: SocketAddrV4) -> Self {
        let name = message
            .args
            .get(0)
            .unwrap()
            .clone()
            .string()
            .unwrap_or_else(|| "unnamed".to_owned());
        let port: u16 = message.args.get(1).unwrap().clone().int().unwrap() as u16;

        Self {
            address: *addr.ip(),
            name,
            port,
        }
    }
}

struct HeartBeat {
    chan: Receiver<Artwork>,
}
impl HeartBeat {
    fn new(chan: Receiver<Artwork>) -> Self {
        Self { chan }
    }

    fn run(&mut self) {
        loop {
            if let Ok(data) = self.chan.recv() {
                println!("{:?}", data);
            }
        }
    }
}

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("port")
                .long("port")
                .short("p")
                .takes_value(true)
                .default_value("9999"),
        )
        .get_matches();

    let port: u16 = matches
        .value_of("port")
        .unwrap()
        .parse()
        .expect("Not an integer");

    let (tx, rx) = channel();

    // Start Subscriber Listener
    let subscriber = thread::spawn(move || {
        let mut proc = Subscriber::new(tx, port);
        proc.run();
    });

    let heartbeat = thread::spawn(move || {
        let mut proc = HeartBeat::new(rx);
        proc.run();
    });

    subscriber.join().unwrap();
    heartbeat.join().unwrap();
}
