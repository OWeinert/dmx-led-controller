use rpi_led_matrix::{LedMatrix, LedColor, LedMatrixOptions, LedRuntimeOptions};
use cascade::cascade;

/// Draw a simple line from (0,0) to (canvas.width, canvas.height).
fn main() {
    let rt_options = cascade! {
        LedRuntimeOptions::new();
        ..set_gpio_slowdown(3);
    };
    let options = cascade! {
        LedMatrixOptions::new();
        ..set_rows(64);
        ..set_cols(64);
        ..set_pwm_lsb_nanoseconds(130);
    };

    let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
    let mut canvas = matrix.offscreen_canvas();
    let (width, height) = canvas.canvas_size();
    canvas.clear();
    canvas.draw_line(
        0,
        0,
        width - 1,
        height - 1,
        &LedColor { red: 255, green: 255, blue: 255 }
    );
    let _ = matrix.swap(canvas);
    loop {}
}
