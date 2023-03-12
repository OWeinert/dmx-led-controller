mod render_engine;

use cascade::cascade;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder,
    SimulatorDisplay,
    SimulatorEvent,
    Window,
    sdl2::Keycode};
use render_engine::{Engine, Parameter};
use std::time::{Instant};
mod logic_analyzer;

use std::process::Command;
use std::sync::mpsc;
use std::thread;
use crate::logic_analyzer::{DmxPacket, get_dmx_data};

fn main() {
    // set dmx output -> ch1: 0xAD, ch2: 0xBE, ch3: 0xD0
    let mut command = Command::new("./../uDMX commandline/uDMX");
    let mut start_dmx = command.arg("0");   // start address, channel 1 for dmx analyzer

    let my_array = [11, 22, 33, 44, 55, 66];
    for item in my_array.iter() {
        start_dmx.arg(item.to_string());
    }
    let output = start_dmx.output().unwrap();
    if !output.status.success() {
        println!("{}", String::from_utf8(output.stdout).unwrap());
        panic!("couldnt start uDMX, try reinserting the usb to dmx adapter")
    }

    let (tx, rx) = mpsc::channel::<DmxPacket>();
    thread::spawn(move || {
        get_dmx_data(tx);
    });

    'running: loop {
        let received = rx.try_recv();
        if received.is_ok() {
            let received = received.unwrap();
            println!("{:?}", received);
        }
    }
}
