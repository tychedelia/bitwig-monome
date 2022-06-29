use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use monome::Monome;
use crate::ControlMessage;
use crate::bitwig::message::{BitwigMessage, ClipMessage, TrackEvent};


pub struct Arc {
    tx: Sender<ControlMessage>,
    rx: Receiver<BitwigMessage>,
    monome: Monome,
    arc: [i32; 4],
}

impl Arc {
    pub fn new(tx: Sender<ControlMessage>, rx: Receiver<BitwigMessage>, monome: Monome) -> Self {
        Self {
            tx,
            rx,
            monome,
            arc: [0,0,0,0]
        }
    }

    fn update_state(&mut self, rotary: u8, value: i32) {
        self.arc[(rotary - 1) as usize] = value;
    }

    fn update_encoders(&mut self) {
        self.monome.ring_set(0, self.arc[0] as u32, 255);
        self.monome.ring_set(1, self.arc[1] as u32, 255);
        self.monome.ring_set(2, self.arc[2] as u32, 255);
        self.monome.ring_set(3, self.arc[3] as u32, 255);
    }

    pub fn run(mut self) {
        loop {
            loop {
                match self.rx.try_recv() {
                    Ok(msg) => match msg {
                        BitwigMessage::Clip(_) => {}
                        BitwigMessage::Track(msg) => {
                            match msg.event {
                                TrackEvent::Selected => {
                                    // self.tx.send(ControlMessage::Refresh);
                                }
                            }
                        }
                        BitwigMessage::Primary(msg) => {
                            match msg.param {
                                0..=3 => {
                                    self.update_state(msg.param, msg.value);
                                }
                                _ => {}
                            }
                        }
                    }
                    Err(err) => match err {
                        TryRecvError::Empty => break,
                        TryRecvError::Disconnected => panic!("channel closed!"),
                    },
                }
            }

            self.update_encoders();
        }
    }
}
