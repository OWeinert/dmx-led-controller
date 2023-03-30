use std::fmt::Debug;
use std::time::Duration;

use embedded_graphics::primitives::Line;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*, primitives::PrimitiveStyle};

use crate::views::draw_collection::text::text::TextSmall;

#[derive(Debug)]
pub struct Label {
    width: i32,
    start: Point,
    text: String,
}

impl Label {
    pub fn from_duration(width: i32, start: Point, duration: Duration) -> Self {
        return Label {
            width,
            start,
            text: Self::get_string(duration),
        };
    }

    fn get_string(duration: Duration) -> String {
        format!("{} µs", duration.as_nanos() as f32 / 1000.0)
    }
}

impl Drawable for Label {
    type Color = Rgb888;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Rgb888>,
    {
        let y = self.start.y;
        let width = self.width - 1;
        let x_start = self.start.x;
        let x_end = self.start.x + width;
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
        TextSmall {
            text: &self.text,
            color: Rgb888::WHITE,
            start: Point::new(x_start + 2, y + 7),
        }
        .draw(target)?;

        Ok(())
    }
}
