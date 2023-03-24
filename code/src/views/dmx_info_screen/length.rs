use std::fmt::Debug;
use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::PrimitiveStyle,
    text::Text,
};
use embedded_graphics::mono_font::iso_8859_1::FONT_4X6;
use embedded_graphics::primitives::Line;

#[derive(Debug)]
pub struct LengthVisualization {
    x_range: (i32, i32),
    start: Point,
}


impl LengthVisualization {
    pub fn new(x_range: (i32, i32), start: Point) -> Self {
        return LengthVisualization{
            x_range,
            start,
        }
    }
}


impl Drawable for LengthVisualization {
    type Color = Rgb888;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
        where
            D: DrawTarget<Color = Rgb888>,
    {
        let y = self.start.y;
        let (x0, x1) = self.x_range;
        let x_start = self.start.x + x0;
        let x_end = self.start.x + x1 - 1;
        let style = PrimitiveStyle::with_stroke(Rgb888::CSS_DARK_GRAY, 1);

        Line::new(Point::new(x_start, y + 1), Point::new(x_end, y + 1))
            .into_styled(style)
            .draw(target)?;
        Line::new(Point::new(x_start, y), Point::new(x_start, y + 2))
            .into_styled(style)
            .draw(target)?;
        Line::new(Point::new(x_end, y), Point::new(x_end, y + 2))
            .into_styled(style)
            .draw(target)?;

        Text::new(
            "1,3µs",
            Point::new(x_start + 1, y+ 7),
            MonoTextStyle::new(&FONT_4X6, Rgb888::WHITE),
        ).draw(target)?;

        Ok(())
    }
}
