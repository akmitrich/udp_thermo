#![allow(unused, dead_code)]

use std::net::{SocketAddr, UdpSocket};

use crate::thermo::{ThermoObserver, Thermometer};

pub struct UdpThermo {
    socket: UdpSocket,
    receiver: SocketAddr,
}

impl UdpThermo {
    pub fn new(addr: &str) -> Self {
        let socket = UdpSocket::bind("127.0.0.1:4083").expect("unable to bind");
        let receiver = addr.parse::<SocketAddr>().expect("invalid addr");
        Self { socket, receiver }
    }
}

impl ThermoObserver for UdpThermo {
    fn observe(&self, thermo: &Thermometer) {
        let bytes = thermo.get_temperature().to_be_bytes();
        let sent = self.socket.send_to(&bytes, self.receiver);
        if let Err(e) = sent {
            eprintln!(
                "Unexpected error while sending {}. Error: {}",
                thermo.get_temperature(),
                e
            );
        } else {
            println!("Sent: {}", thermo.get_temperature());
        }
    }
}
