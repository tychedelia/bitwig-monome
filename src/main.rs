#![feature(div_duration)]

mod message;
mod osc_recv;
mod osc_send;
mod device;
mod bitwig;


use crate::message::{ClipEvent, ControlMessage};
use crate::osc_send::OscSend;
use monome::{Monome, MonomeDeviceType};
use osc_recv::OscRecv;
use std::error::Error;
use std::net::SocketAddr;
use clap::Parser;

use std::sync::mpsc::{channel};
use std::thread;


use tracing_subscriber::{EnvFilter, fmt};
use tracing_subscriber::layer::SubscriberExt;
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
    // install tracing
    let subscriber = tracing_subscriber::registry()
        .with(
            EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .with(fmt::Layer::new().pretty().with_writer(std::io::stdout));
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global collector");

    // run clap
    let args: Args = Args::parse();

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

    let monome = Monome::new("/bitwig-monome".to_string()).unwrap();
    match monome.device_type() {
        MonomeDeviceType::Grid => {
            let grid = Grid::new(tx_out, rx_in, monome);
            grid.run();
        }
        MonomeDeviceType::Arc => {}
        MonomeDeviceType::Unknown => {
            tracing::error!(?monome, "unknown device");
            panic!("unknown monome device");
        }
    }

    Ok(())
}
