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

fn main() {
    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(0)
        .scale(2)
        .build();
    let mut display = SimulatorDisplay::new(
        Size::new(300, 300)
    );
    let mut window = cascade! {
        Window::new("Led Matrix", &output_settings);
        ..update(&display);
    };

    let str = [
        "src/objects/cube.obj",
        "src/objects/video_ship.obj",
        "src/objects/teapot.obj"
    ];
    let mut engine = Engine::new(&str[0], &mut display);
    let mut last: Instant = Instant::now();

    'running: loop {
        let now = Instant::now();
        let mut parameter = Parameter{
            eye: Default::default(),
            rotation: 0.01,
            elapsed_time: now - last,
            print_state: false
        };
        for event in window.events() {
            match event {
                SimulatorEvent::Quit =>
                    break 'running,
                SimulatorEvent::KeyDown { keycode, .. } =>
                    match keycode {
                        Keycode::Right => parameter.eye.x = -1.0,
                        Keycode::Left => parameter.eye.x = 1.0,
                        Keycode::Down => parameter.eye.z = -1.0,
                        Keycode::Up => parameter.eye.z = 1.0,
                        Keycode::W => parameter.eye.y = -0.1,
                        Keycode::S => parameter.eye.y = 0.1,
                        Keycode::N => parameter.rotation = 0.1,
                        Keycode::M => parameter.rotation = -0.1,
                        Keycode::P => parameter.print_state = true,
                        _ => {}
                    },
                _ => {}
            }
        }
        engine.on_user_update(&mut display, parameter);
        last = now;
        window.update(&display);
        display.clear(Rgb888::new(0, 0, 0)).unwrap();
    }
}
