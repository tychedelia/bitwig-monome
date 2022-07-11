use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::time::Duration;
use monome::{Monome, MonomeEvent};
use crate::bitwig::clip::Clip;
use crate::{ClipEvent, ClipState, ControlMessage};
use crate::bitwig::message::{BitwigMessage, ClipMessage};

pub(crate) struct Grid {
    rx: Receiver<BitwigMessage>,
    tx: Sender<ControlMessage>,
    grid: Vec<Clip>,
    monome: Monome,
}

impl Grid {
    pub fn new(tx: Sender<ControlMessage>, rx: Receiver<BitwigMessage>, monome: Monome) -> Self {
        Self {
            rx,
            tx,
            grid: vec![Clip::new(); 128],
            monome,
        }
    }

    pub(crate) fn update_intensities(&mut self) {
        self.grid.iter_mut().for_each(|x| x.update_intensity());
    }

    pub(crate) fn clip_mut(&mut self, track: u8, scene: u8) -> &mut Clip {
        &mut self.grid[(scene as usize - 1) * 16 + (track as usize - 1)]
    }

    pub(crate) fn clip(&self, track: u8, scene: u8) -> &Clip {
        &self.grid[(scene as usize - 1) * 16 + (track as usize - 1)]
    }

    pub(crate) fn get_intensities(&self) -> Vec<u8> {
        self.grid.iter().map(|x| x.intensity).collect()
    }

    pub fn run(mut self) {
        self.monome.set_all_intensity(&self.get_intensities());

        loop {
            loop {
                // State Transitions
                match self.rx.try_recv() {
                    Ok(BitwigMessage::Clip(msg)) => {
                        let s = self.clip_mut(msg.track, msg.scene);
                        match (msg.event, msg.active) {
                            (ClipEvent::Playing, true) => s.state = ClipState::Playing,
                            (ClipEvent::Playing, false) => {
                                match s.state {
                                    ClipState::Playing => s.state = ClipState::Filled,
                                    _ => {}
                                }
                            }
                            (ClipEvent::Stopping, true) => s.state = ClipState::Stopping,
                            (ClipEvent::Stopping, false) => {
                                match s.state {
                                    ClipState::Playing | ClipState::Stopping => s.state = ClipState::Filled,
                                    _ => {}
                                }
                            }
                            (ClipEvent::Content, true) => s.state = ClipState::Filled,
                            (ClipEvent::Content, false) => s.state = ClipState::Empty,
                            (ClipEvent::Queued, true) => s.state = ClipState::Queued,
                            (ClipEvent::Queued, false) => {
                                match s.state {
                                    ClipState::Playing | ClipState::Queued => s.state = ClipState::Filled,

                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    Ok(_) => {},
                    Err(err) => match err {
                        TryRecvError::Empty => break,
                        TryRecvError::Disconnected => panic!("channel closed!"),
                    },
                }
            }

            loop {
                let e = self.monome.poll();

                match e {
                    Some(MonomeEvent::GridKey { x, y, direction: _ }) => {
                        let x = x as usize;
                        let y = y as usize;
                        let track = (x + 1) as u8;
                        let scene = (y + 1) as u8;
                        let s = self.clip(track, scene);

                        if let ClipState::Filled = s.state {
                            self.send(ControlMessage::Launch(track, scene));
                        }
                        if let ClipState::Playing = s.state {
                            self.send(ControlMessage::Stop(track, scene));
                        }
                    }
                    _ => {
                        break;
                    }
                }
            }

            self.update_intensities();
            self.monome.set_all_intensity(&self.get_intensities());
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    fn send(&self, msg: ControlMessage) {
        self.tx.send(msg).unwrap();
    }
}
