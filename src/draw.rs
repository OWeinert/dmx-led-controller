use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, Triangle},
    text::Text,
};
use std::fmt::Debug;

pub fn draw<D>(display: &mut D)
where
    D: DrawTarget<Color = Rgb888>,
    D::Error: Debug,
{
    // Create styles used by the drawing operations.
    let fill = PrimitiveStyle::with_fill(Rgb888::new(0, 150, 0));
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb888::new(0xff, 0xff, 0xff));

    // Draw a triangle.
    Triangle::new(Point::new(1, 62), Point::new(31, 1), Point::new(62, 62))
        .into_styled(fill)
        .draw(display)
        .unwrap();

    Text::new("Conrad", Point::new(14, 45), text_style)
        .draw(display)
        .unwrap();

    Text::new("Klaus", Point::new(14, 55), text_style)
        .draw(display)
        .unwrap();
}
