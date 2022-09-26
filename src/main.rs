mod render_engine;

use cascade::cascade;
use rpi_led_matrix::{LedMatrix, LedMatrixOptions, LedRuntimeOptions};
use render_engine::draw;
use std::time::{Instant};

fn main() {
    let now = Instant::now();
    let options = cascade! {
        LedMatrixOptions::new();
        ..set_rows(64);
        ..set_cols(64);
        ..set_pwm_lsb_nanoseconds(300);
        ..set_hardware_pulsing(true);
        ..set_brightness(50).unwrap();
    };
    let rt_options = cascade! {
        LedRuntimeOptions::new();
        ..set_gpio_slowdown(3);
    };
    let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
    let mut canvas = matrix.offscreen_canvas();

    loop {
        draw(&mut canvas, now.elapsed().as_secs_f32());
        canvas = matrix.swap(canvas);
        canvas.clear();
    }
}
