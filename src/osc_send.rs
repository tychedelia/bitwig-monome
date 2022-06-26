use crate::{ControlMessage};
use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::net::{SocketAddr, UdpSocket};

use std::sync::mpsc::{Receiver};

pub struct OscSend {
    pub(crate) rx: Receiver<ControlMessage>,
    sock: UdpSocket,
    to_addr: SocketAddr,
}

impl OscSend {
    pub(crate) fn new(
        rx: Receiver<ControlMessage>,
        bind_addr: SocketAddr,
        to_addr: SocketAddr,
    ) -> Self {
        Self {
            rx,
            sock: UdpSocket::bind(bind_addr).unwrap(),
            to_addr,
        }
    }

    fn send_message(&self, addr: impl Into<String>, args: Vec<OscType>) {
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: addr.into(),
            args,
        }))
        .unwrap();
        self.sock.send_to(&msg_buf, self.to_addr).unwrap();
    }

    pub(crate) fn run(self) {
        loop {
            match self.rx.recv() {
                Ok(msg) => match msg {
                    ControlMessage::Refresh => self.send_message("/refresh", vec![]),
                    ControlMessage::Launch(track, scene) => {
                        self.send_message(format!("/track/{track}/clip/{scene}/launch"), vec![])
                    }
                    ControlMessage::Stop(track, _scene) => {
                        self.send_message(format!("/track/{track}/clip/stop"), vec![])
                    }
                    _ => {}
                },
                Err(_) => panic!("channel closed!"),
            }
        }
    }
}
