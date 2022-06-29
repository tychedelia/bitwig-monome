#![feature(div_duration)]

mod device;
mod bitwig;


use bitwig::message::{ClipEvent, ControlMessage};
use bitwig::osc_send::OscSend;
use monome::{Monome, MonomeDeviceType};
use bitwig::osc_recv::OscRecv;
use std::error::Error;
use std::net::SocketAddr;
use clap::Parser;

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use tracing_subscriber::{EnvFilter, fmt};
use tracing_subscriber::layer::SubscriberExt;
use crate::bitwig::clip::ClipState;
use crate::device::grid::Grid;
use crate::device::arc::Arc;

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

    // io channels
    let (tx_in, rx_in) = channel();
    let (tx_out, rx_out) = channel();

    // initialize osc listeners
    thread::spawn(move || {
        let r = OscRecv::new(tx_in, args.bitwig_addr);
        r.run();
    });
    thread::spawn(move || {
        let bind_addr  = "127.0.0.1:0".parse().unwrap();
        let s = OscSend::new(rx_out, bind_addr, args.osc_addr);
        s.run();
    });
    tracing::info!(?args, "listening");

    // wait for device to connect
    let monome = loop {
        match Monome::new("/bitwig-monome".to_string()) {
            Ok(m) => break m,
            Err(e) => {
                match e.as_str() {
                    "No devices detected" => {
                        thread::sleep(Duration::from_secs(1));
                        continue
                    }
                    _ => panic!("{}", e)
                }
            },
        };
    };

    // get initial state from bitwig
    tx_out.send(ControlMessage::Refresh)?;

    // select connected device
    match monome.device_type() {
        MonomeDeviceType::Grid => {
            tracing::info!(?monome, "found grid device");
            let grid = Grid::new(tx_out, rx_in, monome);
            grid.run();
        }
        MonomeDeviceType::Arc => {
            tracing::info!(?monome, "found arc device");
            let arc = Arc::new(tx_out, rx_in, monome);
            arc.run();
        }
        MonomeDeviceType::Unknown => {
            tracing::error!(?monome, "unknown device");
            panic!("unknown monome device!");
        }
    }

    Ok(())
}
