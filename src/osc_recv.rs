use crate::message::{ClipEvent, ClipMessage};
use rosc::OscPacket::{Bundle, Message};
use rosc::{OscBundle, OscMessage, OscPacket, OscType};
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::{Sender};


pub(crate) struct OscRecv {
    pub(crate) tx: Sender<ClipMessage>,
    bind_addr: SocketAddr,
}

impl OscRecv {
    pub(crate) fn new(tx: Sender<ClipMessage>, bind_addr: SocketAddr) -> Self {
        Self {
            tx,
            bind_addr,
        }
    }

    pub(crate) fn run(self) {
        let sock = UdpSocket::bind(self.bind_addr).unwrap();
        let mut buf = [0u8; 8192];

        loop {
            match sock.recv_from(&mut buf) {
                Ok((size, _addr)) => {
                    let (_, packet) = rosc::decoder::decode_udp(&buf[..size]).unwrap();
                    self.handle_packet(packet);
                }
                Err(e) => {
                    println!("Error receiving from socket: {}", e);
                    break;
                }
            }
        }
    }

    fn handle_packet(&self, packet: OscPacket) {
        match packet {
            Bundle(bundle) => self.handle_bundle(bundle),
            Message(msg) => self.handle_message(msg),
        }
    }

    fn handle_bundle(&self, bundle: OscBundle) {
        bundle
            .content
            .into_iter()
            .for_each(|x| self.handle_packet(x));
    }

    fn handle_message(&self, msg: OscMessage) {
        match msg.addr.as_str() {
            "/update" | "/beat/str" | "/time/str" | "/play" => {}
            s if s.starts_with("/track/selected") => {}
            s if s.starts_with("/track") && s.contains("clip") => self.handle_track_message(msg),
            _ => {} // println!("{} {:?}", msg.addr, msg.args),
        }
    }

    fn handle_track_message(&self, msg: OscMessage) {
        let (track, scene) = Self::parse_track_and_scene(&msg);
        let active = Self::arg_to_bool(&msg.args[0]);
        match msg.addr.as_str() {
            s if s.ends_with("isPlayingQueued") => {
                self.tx
                    .send(ClipMessage::new(track, scene, active, ClipEvent::Queued));
            }
            s if s.ends_with("hasContent") => {
                self.tx
                    .send(ClipMessage::new(track, scene, active, ClipEvent::Content));
            }
            s if s.ends_with("isSelected") => {
                self.tx
                    .send(ClipMessage::new(track, scene, active, ClipEvent::Selected));
            }
            s if s.ends_with("isStopQueued") => {
                self.tx
                    .send(ClipMessage::new(track, scene, active, ClipEvent::Stopping));
            }
            s if s.ends_with("isPlaying") => {
                self.tx
                    .send(ClipMessage::new(track, scene, active, ClipEvent::Playing));
            }
            _ => {}
        }
    }

    fn arg_to_bool(arg: &OscType) -> bool {
        match arg {
            OscType::Int(i) => match i {
                0 => false,
                1 => true,
                _ => panic!("unexpected bool int!"),
            },
            OscType::Bool(b) => *b,
            _ => false,
        }
    }

    fn parse_track_and_scene(msg: &OscMessage) -> (u8, u8) {
        let parts: Vec<_> = msg.addr.split('/').collect();
        let track = parts[2].parse().unwrap_or_else(|_| panic!("{:?}", msg.addr));
        let scene = parts[4].parse().unwrap_or_else(|_| panic!("{:?}", msg.addr));
        (track, scene)
    }
}
