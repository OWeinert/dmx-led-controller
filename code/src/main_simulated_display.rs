use core::time;
use std::time::Instant;
use std::thread;

use cascade::cascade;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*, primitives::{Triangle, StyledDrawable, PrimitiveStyle}};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use strum::EnumCount;


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

    /*
    let str = [
        "assets/objects/cube.obj",
        "assets/objects/video_ship.obj",
        "assets/objects/teapot.obj",
    ];
    */
    //let mut last: Instant = Instant::now();

    let anim = [
        Triangle::new(Point::new(1, 1), Point::new(12, 4), Point::new(6, 20)),
        Triangle::new(Point::new(1, 4), Point::new(10, 6), Point::new(10, 10)),
        Triangle::new(Point::new(2, 7), Point::new(7, 9), Point::new(6, 12)),
        Triangle::new(Point::new(4, 9), Point::new(3, 1), Point::new(2, 16)),];

    let mut index : usize = 0;

    'running: loop {
        let now = Instant::now();
        /* 
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
        */

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }

        /*
        //let props = match parameter_dmx.dmx_output.channels[99] {
        let props = Views::RenderEngine(RenderEngineProps {
            parameter_render_engine: parameter_3d_engine,
            parameter_dmx_channels: parameter_dmx.clone(),
        });
        view.on_user_update(&mut display, props);
        */
        anim[index].draw_styled(&PrimitiveStyle::with_fill(Rgb888::GREEN), &mut display).unwrap();
        index += 1;
        if index > 3 {
            index = 0;
        }

        //last = now;
        window.update(&display);
        display.clear(Rgb888::new(0, 0, 0)).unwrap();
        thread::sleep(time::Duration::from_millis(20));
    }
}
