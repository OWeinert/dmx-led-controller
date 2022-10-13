mod render_engine;

use cascade::cascade;
use std::time::{Instant};
use render_engine::{Engine, Parameter};
use rpi_led_matrix::{LedMatrix, LedMatrixOptions, LedRuntimeOptions};

fn main() {
    let options = cascade! {
        LedMatrixOptions::new();
        ..set_rows(64);
        ..set_cols(64);
        ..set_pwm_lsb_nanoseconds(300);
        ..set_hardware_pulsing(true);
        ..set_brightness(50).unwrap();
        ..set_refresh_rate(false);
    };
    let rt_options = cascade! {
        LedRuntimeOptions::new();
        ..set_gpio_slowdown(3);
    };
    let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
    let mut canvas = matrix.offscreen_canvas();

    let str = [
        "src/objects/cube.obj",
        "src/objects/video_ship.obj",
        "src/objects/teapot.obj"
    ];
    let mut engine = Engine::new(&str[0], &mut canvas);
    let mut last: Instant = Instant::now();

    loop {
        let now = Instant::now();
        let parameter = Parameter{
            eye: Default::default(),
            rotation: 0.015,
            elapsed_time: now - last,
            print_state: false
        };

        engine.on_user_update(&mut canvas, parameter);
        last = now;
        canvas = matrix.swap(canvas);
        canvas.clear();
    }
}
