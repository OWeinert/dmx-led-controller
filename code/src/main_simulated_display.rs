mod controller;
mod logic_analyzer;

use cascade::cascade;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use controller::{Controller, Parameter};

fn main() {
    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(2)
        .scale(10)
        .build();
    let mut display = SimulatorDisplay::new(Size::new(64, 64));
    let mut window = cascade! {
        Window::new("Led Matrix", &output_settings);
        ..update(&display);
    };

    let mut controller = Controller::new();

    'running: loop {
        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
        let received = controller.rx.try_recv();
        if received.is_ok() {
            let received = received.unwrap();
            println!("length:{}, {:02X?}", received.channels.len(), received);
            let parameter = Parameter {
                channels: received
            };
            controller.on_user_update(&mut display, parameter);
            window.update(&display);
            display.clear(Rgb888::new(0, 0, 0)).unwrap();
        }
    }
}
