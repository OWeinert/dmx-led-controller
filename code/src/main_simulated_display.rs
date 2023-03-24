mod dmx_analyzer;
mod views;

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
use std::time::{Instant};

use dmx_analyzer::{DmxAnalyzer};
use dmx_analyzer::logic_analyzer::DmxPacket;
use views::Views;
use views::ViewController;
use crate::views::dmx_info_screen::ParameterDmxInfoScreen;
use crate::views::RenderEngineProps;

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

    let str = [
        "assets/objects/cube.obj",
        "assets/objects/video_ship.obj",
        "assets/objects/teapot.obj"
    ];
    let mut view = ViewController::new(str[1], &mut display);
    let mut last: Instant = Instant::now();

    let controller = DmxAnalyzer::new(false, false);

    let mut parameter_dmx = dmx_analyzer::Parameter {
        channels: DmxPacket{channels: [0; 512]}
    };
    'running: loop {
        let now = Instant::now();
        let mut parameter_3d_engine = views::render_engine::Parameter{
            eye: Default::default(),
            rotation: 0.01,
            elapsed_time: now - last,
            print_state: false,
            rgb: [
                parameter_dmx.channels.channels[1] as f32 / 255.0,
                parameter_dmx.channels.channels[2] as f32 / 255.0,
                parameter_dmx.channels.channels[3] as f32 / 255.0
            ],

        };

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } =>
                    match keycode {
                        Keycode::Right => parameter_3d_engine.eye.x = -1.0,
                        Keycode::Left => parameter_3d_engine.eye.x = 1.0,
                        Keycode::Down => parameter_3d_engine.eye.z = -1.0,
                        Keycode::Up => parameter_3d_engine.eye.z = 1.0,
                        Keycode::W => parameter_3d_engine.eye.y = -0.1,
                        Keycode::S => parameter_3d_engine.eye.y = 0.1,
                        Keycode::N => parameter_3d_engine.rotation = 0.1,
                        Keycode::M => parameter_3d_engine.rotation = -0.1,
                        Keycode::P => parameter_3d_engine.print_state = true,
                        _ => {}
                    },
                _ => {}
            }
        }

        let received = controller.rx.try_recv();
        if received.is_ok() {
            let received = received.unwrap();
            println!("length:{}, {:02X?}", received.channels.len(), received);
            parameter_dmx = dmx_analyzer::Parameter {
                channels: received
            };
        }

       // let props = Views::RenderEngine(RenderEngineProps{parameter_render_engine: parameter_3d_engine, parameter_dmx_channels: parameter_dmx.clone()});
        let props = Views::Channel1Timing(ParameterDmxInfoScreen{});
        view.on_user_update(&mut display, props);
        last = now;
        window.update(&display);
        display.clear(Rgb888::new(0, 0, 0)).unwrap();
    }
}
