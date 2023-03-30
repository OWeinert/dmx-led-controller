use std::time::Instant;
use cascade::cascade;
use measurements::Frequency;
use rpi_led_matrix::{LedMatrix, LedMatrixOptions, LedRuntimeOptions};
use crate::dmx_analyzer::dmx_state_machine::{Bit, DmxOutput, ResetSequence};
use crate::dmx_analyzer::DmxAnalyzer;
use crate::views::dmx_channel_1::ParameterDmxInfoScreen;
use crate::views::{RenderEngineProps, ViewController, Views};

mod dmx_analyzer;
mod views;

//use views::

fn main() {
    let options = cascade! {
        LedMatrixOptions::new();
        ..set_rows(64);
        ..set_cols(64);
        ..set_pwm_lsb_nanoseconds(300);
        ..set_hardware_pulsing(false);
        ..set_brightness(50).unwrap();
        ..set_refresh_rate(false);
    };
    let rt_options = cascade! {
        LedRuntimeOptions::new();
        ..set_gpio_slowdown(3);
    };
    let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
    let mut canvas = matrix.offscreen_canvas();

    let sample_frequency = Frequency::from_megahertz(2.0);
    let controller = DmxAnalyzer::new(true, false, sample_frequency);

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

    let str = [
        "assets/objects/cube.obj",
        "assets/objects/video_ship.obj",
        "assets/objects/teapot.obj",
    ];
    let mut view = ViewController::new(str[1], &mut canvas);
    let mut last: Instant = Instant::now();

    loop {
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

        let received = controller.rx.try_recv();
        if received.is_ok() {
            let received = received.unwrap();
            println!("length:{}, {:02X?}", received.channels.len(), received);
            parameter_dmx = dmx_analyzer::Parameter {
                dmx_output: received,
            };
        }

        let props = match parameter_dmx.dmx_output.channels[100] {
            0..=49 => Views::ResetSequence(ParameterDmxInfoScreen {
                dmx_output: parameter_dmx.dmx_output,
                frequency: sample_frequency,
            }),
            50..=99 => Views::Channel1Timing(ParameterDmxInfoScreen {
                dmx_output: parameter_dmx.dmx_output,
                frequency: sample_frequency,
            }),
            100.. => Views::RenderEngine(RenderEngineProps {
                parameter_render_engine: parameter_3d_engine,
                parameter_dmx_channels: parameter_dmx.clone(),
            }),
        };

        view.on_user_update(&mut canvas, props);
        last = now;
        canvas = matrix.swap(canvas);
        canvas.clear();

    }
}
