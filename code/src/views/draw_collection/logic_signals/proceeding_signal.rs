use std::ops::Add;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{Rgb888, WebColors};
use embedded_graphics::{Drawable, Pixel};

pub struct ProceedingSignal {
    pub start: Point,
    pub length: i32,
}

impl Drawable for ProceedingSignal {
    type Color = Rgb888;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Rgb888>,
    {
        let range = (0..self.length).filter(|i| i % 2 == 0);
        for i in range {
            Pixel(self.start.add(Point::new(i, 0)), Rgb888::CSS_DARK_GRAY).draw(target)?;
        }
        Ok(())
    }
}
