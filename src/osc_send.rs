use std::net::{SocketAddr, SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::sync::mpsc::{Receiver, RecvError};
use rosc::{encoder, OscMessage, OscPacket, OscType};
use crate::{ClipMessage, ControlMessage};

pub struct OscSend {
    pub(crate) rx: Receiver<ControlMessage>,
    sock: UdpSocket,
    to_addr: SocketAddr,
}

impl OscSend {
    pub(crate) fn new(rx: Receiver<ControlMessage>, bind_addr: SocketAddr, to_addr: SocketAddr) -> Self {
        Self {
            rx,
            sock: UdpSocket::bind(bind_addr).unwrap(),
            to_addr,
        }
    }

    fn send_message(&self, addr: &str, args: Vec<OscType>) {
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: addr.to_string(),
            args,
        })).unwrap();
        self.sock.send_to(&msg_buf, self.to_addr).unwrap();
    }

    pub(crate) fn run(self) {
        loop {
            match self.rx.recv() {
                Ok(msg) => {
                    match msg {
                        ControlMessage::Refresh => self.send_message("/refresh", vec![]),
                    }
                }
                Err(_) => panic!("channel closed!")
            }
        }
    }
}
