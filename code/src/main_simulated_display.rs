use std::time::Instant;

use cascade::cascade;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use measurements::Frequency;
use strum::EnumCount;

use dmx_analyzer::DmxAnalyzer;
use views::ViewController;
use views::Views;

use crate::dmx_analyzer::dmx_state_machine::{Bit, DmxOutput, ResetSequence};
use crate::views::dmx_channel_1::ParameterDmxInfoScreen;
use crate::views::RenderEngineProps;

mod dmx_analyzer;
mod views;

fn main() {
    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(2)
        .scale(10)
        .build();
    let mut display = SimulatorDisplay::new(Size::new(64, 64));
    let mut window = cascade! {
        Window::new("Dmx Analyzer", &output_settings);
        ..update(&display);
    };

    let str = [
        "assets/objects/cube.obj",
        "assets/objects/video_ship.obj",
        "assets/objects/teapot.obj",
    ];
    let mut view = ViewController::new(str[1], &mut display);
    let mut last: Instant = Instant::now();

    let sample_frequency = Frequency::from_megahertz(24.0);
    let controller = DmxAnalyzer::new(false, false, sample_frequency);

    let bit = Bit {
        start_sample: 0,
        end_sample: 0,
        bit: true,
    };
    let mut parameter_dmx = dmx_analyzer::Parameter {
        dmx_output: DmxOutput {
            reset_sequence: ResetSequence {
                mark_after_break: 0,
                space_for_break: 0,
            },
            bits: [bit; 8],
            channels: [0; 512],
        },
    };
    let mut view_nr = 0;
    'running: loop {
        let now = Instant::now();
        let mut parameter_3d_engine =
            views::render_engine_with_dmx_overlay::render_engine::Parameter {
                eye: Default::default(),
                rotation: 0.01,
                elapsed_time: now - last,
                print_state: false,
                rgb: [
                    parameter_dmx.dmx_output.channels[0] as f32 / 255.0,
                    parameter_dmx.dmx_output.channels[1] as f32 / 255.0,
                    parameter_dmx.dmx_output.channels[2] as f32 / 255.0,
                ],
            };

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown {
                    keycode, repeat, ..
                } => match keycode {
                    Keycode::Right => parameter_3d_engine.eye.x = -1.0,
                    Keycode::Left => parameter_3d_engine.eye.x = 1.0,
                    Keycode::Down => parameter_3d_engine.eye.z = -1.0,
                    Keycode::Up => parameter_3d_engine.eye.z = 1.0,
                    Keycode::W => parameter_3d_engine.eye.y = -0.1,
                    Keycode::S => parameter_3d_engine.eye.y = 0.1,
                    Keycode::N => parameter_3d_engine.rotation = 0.1,
                    Keycode::M => parameter_3d_engine.rotation = -0.1,
                    Keycode::P => parameter_3d_engine.print_state = true,
                    Keycode::Space => {
                        if repeat {
                            break;
                        }
                        view_nr += 1;
                        if view_nr >= Views::COUNT {
                            view_nr = 0;
                        }
                    }
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
                dmx_output: received,
            };
        }

        //let props = match parameter_dmx.dmx_output.channels[99] {
        let props = match view_nr {
            0 => Views::ResetSequence(ParameterDmxInfoScreen {
                dmx_output: parameter_dmx.dmx_output,
                frequency: sample_frequency,
            }),
            1 => Views::Channel1Timing(ParameterDmxInfoScreen {
                dmx_output: parameter_dmx.dmx_output,
                frequency: sample_frequency,
            }),
            2 => Views::RenderEngine(RenderEngineProps {
                parameter_render_engine: parameter_3d_engine,
                parameter_dmx_channels: parameter_dmx.clone(),
            }),
            _ => panic!(),
        };
        view.on_user_update(&mut display, props);
        last = now;
        window.update(&display);
        display.clear(Rgb888::new(0, 0, 0)).unwrap();
    }
}
