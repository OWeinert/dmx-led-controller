use core::time;
//use std::time::Instant;
use std::thread;
use std::env;

use cascade::cascade;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*, primitives::PrimitiveStyle};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use dlcl::interop::python;
use dlcl::draw::{self, framebuffer, layer};
use relative_path::RelativePath;
use dlcl::draw::layer::{DEFAULT_LAYER_0, DEFAULT_LAYER_1};

fn main() {
    let disp_width = 64;
    let disp_height = 64;

    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(2)
        .scale(10)
        .build();
    let mut display = SimulatorDisplay::new(Size::new(disp_width, disp_height));
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
    let py_script = python::load_py_script(py_path.as_path());

    // call the setup method on the script
    let _ = python::call_setup(&py_script);

    framebuffer::set_framebuf_size(disp_width as usize, disp_height as usize);
    DEFAULT_LAYER_0.lock().unwrap().set_size(disp_width as usize, disp_height as usize);
    DEFAULT_LAYER_1.lock().unwrap().set_size(disp_width as usize, disp_height as usize);

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

        //python::call_loop(&py_script);

        draw::draw_circle_layer(Point::new(8,8),
                                16,
                                PrimitiveStyle::with_fill(Rgb888::RED),
                                &layer::DEFAULT_LAYER_0.lock().unwrap());
        draw::draw_circle_layer(Point::new(16,8),
                                16,
                                PrimitiveStyle::with_stroke(Rgb888::BLUE, 1),
                                &layer::DEFAULT_LAYER_1.lock().unwrap());

        draw::draw_layers(&mut [&DEFAULT_LAYER_0.lock().unwrap(), &DEFAULT_LAYER_1.lock().unwrap()]);
        framebuffer::draw_framebuf(&mut display);

        window.update(&display);
        display.clear(Rgb888::new(0, 0, 0)).unwrap();
        framebuffer::clear_framebuf();
        thread::sleep(time::Duration::from_micros(16666));
    }
}
