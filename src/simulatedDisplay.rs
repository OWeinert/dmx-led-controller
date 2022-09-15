mod draw;

use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use draw::draw;

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(64, 64));
    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(6)
        .scale(6)
        .build();
    let mut window = Window::new("Led Matrix", &output_settings);
    draw(&mut display);

    window.update(&display);
    'running: loop {
        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
