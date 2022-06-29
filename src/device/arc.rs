use std::sync::mpsc::{Receiver, Sender};
use monome::Monome;
use crate::ControlMessage;
use crate::message::ClipMessage;

pub struct Arc {
    tx: Sender<ControlMessage>,
    rx: Receiver<ClipMessage>,
    monome: Monome,
}

impl Arc {
    pub fn new(tx: Sender<ControlMessage>, rx: Receiver<ClipMessage>, monome: Monome) -> Self {
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
