use core::time;
//use std::time::Instant;
use std::thread;
use std::env;

use cascade::cascade;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*, primitives::{Triangle, StyledDrawable, PrimitiveStyle}};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use dlcl::interop;
use relative_path::RelativePath;

fn main() {
    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(2)
        .scale(10)
        .build();
    let mut display = SimulatorDisplay::new(Size::new(64, 64));
    let mut window = cascade! {
        Window::new("Dmx Led Controller", &output_settings);
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

    // load the python script "interop_test.py" from the given relative path
    let root = env::current_dir().unwrap();
    let py_path_rel = RelativePath::new("../python/interop_test.py");
    let py_path = py_path_rel.to_logical_path(&root);
    let py_script = interop::python::load_py_script(py_path.as_path());

    // call the setup method on the script
    let _ = interop::python::call_setup(&py_script);

    'running: loop {
        //let now = Instant::now();
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

        interop::python::call_loop(&py_script);

        window.update(&display);
        display.clear(Rgb888::new(0, 0, 0)).unwrap();
        thread::sleep(time::Duration::from_micros(16666));
    }
}
