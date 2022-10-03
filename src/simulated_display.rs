mod render_engine;

use cascade::cascade;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use render_engine::{Engine};
use std::time::{Instant};

fn main() {
    let now = Instant::now();
    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(6)
        .scale(6)
        .build();
    let mut display = SimulatorDisplay::new(Size::new(64, 64));
    let mut window = cascade! {
        Window::new("Led Matrix", &output_settings);
        ..update(&display);
    };

    let str = [
        "src/objects/cube.obj",
        "src/objects/video_ship.obj",
        "src/objects/teapot.obj"
    ];
    let engine = Engine::new(&str[0], &mut display);

    'running: loop {
        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running;
        }
        engine.draw(&mut display, now.elapsed().as_secs_f32());
        window.update(&display);
        display.clear(Rgb888::new(0, 0, 0)).unwrap();
    }
}
