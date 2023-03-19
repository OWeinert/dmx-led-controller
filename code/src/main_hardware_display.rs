mod controller;
mod logic_analyzer;

use cascade::cascade;

use controller::{Parameter, Controller};
use rpi_led_matrix::{LedMatrix, LedMatrixOptions, LedRuntimeOptions};

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
    let mut controller = Controller::new();

    loop {
        let received = controller.rx.try_recv();
        if received.is_ok() {
            let received = received.unwrap();
            println!("length:{}, {:02X?}", received.channels.len(), received);
            let parameter = Parameter {
                channels: received
            };
            controller.on_user_update(&mut canvas, parameter);
            canvas = matrix.swap(canvas);
            canvas.clear();
        }
    }
}
