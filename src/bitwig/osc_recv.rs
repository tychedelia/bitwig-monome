use crate::bitwig::message::{BitwigMessage, ClipEvent, ClipMessage, DeviceMessage, TrackEvent, TrackMessage};
use rosc::OscPacket::{Bundle, Message};
use rosc::{OscBundle, OscMessage, OscPacket, OscType};
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::{Sender};


#[derive(Debug)]
pub(crate) struct OscRecv {
    pub(crate) tx: Sender<BitwigMessage>,
    bind_addr: SocketAddr,
}

impl OscRecv {
    pub(crate) fn new(tx: Sender<BitwigMessage>, bind_addr: SocketAddr) -> Self {
        Self {
            tx,
            bind_addr,
        }
    }

    #[tracing::instrument]
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

    #[tracing::instrument]
    fn handle_message(&self, msg: OscMessage) {
        match msg.addr.as_str() {
            "/update" | "/beat/str" | "/time/str" | "/play" => {}
            s if s.starts_with("/track/selected") => {}
            s if s.starts_with("/track") && s.contains("clip") => self.handle_clip_message(msg),
            s if s.starts_with("/track") => self.handle_track_message(msg),
            s if s.starts_with("/primary") => self.handle_primary_message(msg),
            _ => tracing::trace!("receive message")
        }
    }

    fn handle_primary_message(&self, msg: OscMessage) {
        match msg.addr.as_str() {
            s if s.ends_with("value") => {
                let param = Self::parse_param(&msg);
                self.tx.send(BitwigMessage::Primary(DeviceMessage::new(param, Self::arg_to_int(&msg.args[0]))));
            }
            _ => {}
        }
    }

    fn handle_track_message(&self, msg: OscMessage) {
        match msg.addr.as_str() {
            s if s.ends_with("selected") => {
                let track = Self::parse_track(&msg);
                let active = Self::arg_to_bool(&msg.args[0]);
                self.tx.send(BitwigMessage::Track(TrackMessage::new(track, active, TrackEvent::Selected)));
            }
            _ => {}
        }
    }

    fn handle_clip_message(&self, msg: OscMessage) {
        let (track, scene) = Self::parse_track_and_scene(&msg);
        let active = Self::arg_to_bool(&msg.args[0]);
        match msg.addr.as_str() {
            s if s.ends_with("isPlayingQueued") => {
                self.tx
                    .send(BitwigMessage::Clip(ClipMessage::new(track, scene, active, ClipEvent::Queued)));
            }
            s if s.ends_with("hasContent") => {
                self.tx
                    .send(BitwigMessage::Clip(ClipMessage::new(track, scene, active, ClipEvent::Content)));
            }
            s if s.ends_with("isSelected") => {
                self.tx
                    .send(BitwigMessage::Clip(ClipMessage::new(track, scene, active, ClipEvent::Selected)));
            }
            s if s.ends_with("isStopQueued") => {
                self.tx
                    .send(BitwigMessage::Clip(ClipMessage::new(track, scene, active, ClipEvent::Stopping)));
            }
            s if s.ends_with("isPlaying") => {
                self.tx
                    .send(BitwigMessage::Clip(ClipMessage::new(track, scene, active, ClipEvent::Playing)));
            }
            _ => {}
        }
    }

    fn arg_to_int(arg: &OscType) -> i32 {
        match arg {
            OscType::Int(i) => *i,
            _ => panic!("unexpected arg type!")
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

    fn parse_param(msg: &OscMessage) -> u8 {
        let parts: Vec<_> = msg.addr.split('/').collect();
        let param = parts[3].parse().unwrap_or_else(|_| panic!("{:?}", msg.addr));
        param
    }

    fn parse_track(msg: &OscMessage) -> u8 {
        let parts: Vec<_> = msg.addr.split('/').collect();
        let track = parts[2].parse().unwrap_or_else(|_| panic!("{:?}", msg.addr));
        track
    }

    fn parse_track_and_scene(msg: &OscMessage) -> (u8, u8) {
        let parts: Vec<_> = msg.addr.split('/').collect();
        let track = parts[2].parse().unwrap_or_else(|_| panic!("{:?}", msg.addr));
        let scene = parts[4].parse().unwrap_or_else(|_| panic!("{:?}", msg.addr));
        (track, scene)
    }
}
