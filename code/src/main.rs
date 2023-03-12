mod render_engine;

use cascade::cascade;
use std::time::{Instant};
use render_engine::{Engine, Parameter};
use rpi_led_matrix::{LedMatrix, LedMatrixOptions, LedRuntimeOptions};
mod logic_analyzer;


use std::process::Command;
use std::sync::mpsc;
use std::thread;
use crate::logic_analyzer::{DmxPacket, get_dmx_data};


fn main() {

}
