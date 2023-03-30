use std::process::Command;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::{fmt::Debug, thread};

use measurements::Frequency;

use logic_analyzer::get_dmx_data;

use crate::dmx_analyzer::dmx_state_machine::DmxOutput;

pub mod dmx_state_machine;
pub mod logic_analyzer;

pub struct DmxAnalyzer {
    pub rx: Receiver<DmxOutput>,
}

#[derive(Debug, Clone, Copy)]
pub struct Parameter {
    pub dmx_output: DmxOutput,
}

impl DmxAnalyzer {
    pub fn new(from_device: bool, set_u_dmx_output: bool, sample: Frequency) -> DmxAnalyzer {
        if set_u_dmx_output {
            // set dmx output -> ch1: 0xAD, ch2: 0xBE, ch3: 0xD0
            let mut command = Command::new("./../uDMX commandline/uDMX");
            let start_dmx = command.arg("0"); // start address, channel 1 for dmx analyzer

            let my_array = [
                0b01100101, 0xC0, 0xFF, 0x10, 0x14, 0x18, 0x1C, 0x20, 0x24, 0x28, 0x2C, 0x30, 0x34, 0x38,
                0x3C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x10, 0x30, 0x10, 0x40, 0x15, 0x20, 0x05, 0x40,
            ];
            for item in my_array.iter() {
                start_dmx.arg(item.to_string());
            }

            let output = start_dmx.output().unwrap();
            if !output.status.success() {
                println!("{}", String::from_utf8(output.stdout).unwrap());
                panic!("couldnt start uDMX, try reinserting the usb to dmx adapter")
            }
        }

        let (tx, rx) = mpsc::channel::<DmxOutput>();
        thread::spawn(move || {
            get_dmx_data(tx, from_device, sample);
        });
        return DmxAnalyzer { rx };
    }
}
