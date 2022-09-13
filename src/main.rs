use cascade::cascade;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, Triangle},
    text::Text,
};
use rpi_led_matrix::{LedMatrix, LedMatrixOptions, LedRuntimeOptions};

fn main() {
    let options = cascade! {
        LedMatrixOptions::new();
        ..set_rows(64);
        ..set_cols(64);
        ..set_pwm_lsb_nanoseconds(500);
        ..set_hardware_pulsing(true);
        ..set_brightness(50).unwrap();
    };
    let rt_options = cascade! {
        LedRuntimeOptions::new();
        ..set_gpio_slowdown(3);
    };
    let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
    let mut canvas = matrix.canvas();

    // Create styles used by the drawing operations.
    let fill = PrimitiveStyle::with_fill(Rgb888::new(50, 25, 0));
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb888::new(0xff, 0xff, 0xff));

    // Draw a triangle.
    Triangle::new(
        Point::new(1, 62),
        Point::new(31, 1),
        Point::new(62, 62)
    )
    .into_styled(fill)
    .draw(&mut canvas)
    .unwrap();

    Text::new("Conrad", Point::new(14, 45), text_style)
        .draw(&mut canvas)
        .unwrap();

    Text::new("Klaus", Point::new(14, 55), text_style)
        .draw(&mut canvas)
        .unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
