use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::sync::mpsc::{Receiver, RecvError};
use rosc::{encoder, OscMessage, OscPacket};
use crate::{ClipMessage, ControlMessage};

pub struct OscSend {
    pub(crate) rx: Receiver<ControlMessage>,
}

impl OscSend {
    pub(crate) fn run(self) {
        let sock = UdpSocket::bind("127.0.0.1:9001").unwrap();
        let to_addr = SocketAddrV4::from_str("127.0.0.1:8000").unwrap();

        loop {
            match self.rx.recv() {
                Ok(msg) => {


                    // let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
                    //     addr: "/3".to_string(),
                    //     args: vec![],
                    // }))
                    //     .unwrap();
                }
                Err(_) => panic!("channel closed!")
            }
        }
    }
}
