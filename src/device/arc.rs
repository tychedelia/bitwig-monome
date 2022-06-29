use std::sync::mpsc::{Receiver, Sender};
use monome::Monome;
use crate::ControlMessage;
use crate::bitwig::message::{BitwigMessage, ClipMessage};

pub struct Arc {
    tx: Sender<ControlMessage>,
    rx: Receiver<BitwigMessage>,
    monome: Monome,
}

impl Arc {
    pub fn new(tx: Sender<ControlMessage>, rx: Receiver<BitwigMessage>, monome: Monome) -> Self {
        Self {
            tx,
            rx,
            monome,
        }
    }

    pub fn run(self) {
        loop {

        }
    }
}
