#![feature(div_duration)]

mod message;
mod osc_recv;
mod osc_send;
mod device;
mod bitwig;

use crate::message::{ClipEvent, ControlMessage};
use crate::osc_send::OscSend;
use monome::{Monome, MonomeEvent};
use osc_recv::OscRecv;
use std::error::Error;
use std::net::SocketAddr;
use clap::Parser;

use std::sync::mpsc::{channel, TryRecvError};
use std::thread;
use std::time::{Duration};
use crate::bitwig::clip::ClipState;
use crate::device::grid::Grid;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value = "127.0.0.1:9000")]
    pub bitwig_addr: SocketAddr,

    #[clap(short, long, value_parser, default_value = "127.0.0.1:8000")]
    pub osc_addr: SocketAddr,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();
    let mut grid = Grid::new();

    let mut monome = Monome::new("/prefix".to_string()).unwrap();
    monome.set_all_intensity(&grid.get_intensities());

    let refresh = std::time::Duration::from_millis(10);

    let (tx_in, rx_in) = channel();
    let (tx_out, rx_out) = channel();

    thread::spawn(move || {
        let r = OscRecv::new(tx_in, args.bitwig_addr);
        r.run();
    });
    thread::spawn(move || {
        let bind_addr  = "127.0.0.1:0".parse().unwrap();
        let s = OscSend::new(rx_out, bind_addr, args.osc_addr);
        s.run();
    });

    tx_out.send(ControlMessage::Refresh)?;
    loop {
        loop {
            // State Transitions
            match rx_in.try_recv() {
                Ok(msg) => {
                    let s = grid.get_state(msg.track, msg.scene);
                    match (msg.event, msg.active) {
                        (ClipEvent::Playing, true) => s.state = ClipState::Playing,
                        (ClipEvent::Playing, false) => {
                            match s.state {
                                ClipState::Playing => s.state = ClipState::Filled,
                                _ => panic!("{:?}", s.state)
                            }
                        }
                        (ClipEvent::Stopping, true) => s.state = ClipState::Stopping,
                        (ClipEvent::Stopping, false) => {
                            match s.state {
                                ClipState::Playing => s.state = ClipState::Filled,
                                _ => panic!("{:?}", s.state)
                            }
                        }
                        (ClipEvent::Content, true) => s.state = ClipState::Filled,
                        (ClipEvent::Content, false) => s.state = ClipState::Empty,
                        (ClipEvent::Queued, true) => s.state = ClipState::Queued,
                        (ClipEvent::Queued, false) => {
                            match s.state {
                                ClipState::Playing => s.state = ClipState::Filled,
                                _ => panic!("{:?}", s.state)
                            }
                        }
                        _ => {}
                    }
                }
                Err(err) => match err {
                    TryRecvError::Empty => break,
                    TryRecvError::Disconnected => panic!("channel closed"),
                },
            }
        }

        loop {
            let e = monome.poll();

            match e {
                Some(MonomeEvent::GridKey { x, y, direction: _ }) => {
                    let x = x as usize;
                    let y = y as usize;
                    let track = (x + 1) as u8;
                    let scene = (y + 1) as u8;
                    let s = grid.get_state(track, scene);

                    if let ClipState::Filled = s.state {
                        tx_out.send(ControlMessage::Launch(track, scene)).unwrap();
                    }
                    if let ClipState::Playing = s.state {
                        tx_out.send(ControlMessage::Stop(track, scene)).unwrap();
                    }
                }
                _ => {
                    break;
                }
            }
        }

        grid.update_intensities();
        monome.set_all_intensity(&grid.get_intensities());
        std::thread::sleep(refresh);
    }
}
