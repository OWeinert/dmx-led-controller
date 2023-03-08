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

#[link(name="saleaeLogic", kind="static")]
extern {
    fn testcall(v: f32);
}

fn main() {
    println!("Hello, world from Rust!");

    unsafe {
        testcall(std::f64::consts::PI as f32);
    };
}
