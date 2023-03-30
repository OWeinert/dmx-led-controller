use std::ops::Add;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{Rgb888, WebColors};
use embedded_graphics::{Drawable, Pixel};

pub struct Ends {
    pub start: Point,
    pub height: i32,
    pub state: bool,
}

impl Drawable for Ends {
    type Color = Rgb888;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Rgb888>,
    {
        // draw ends
        let y = if self.state { 0 } else { self.height };
        Pixel(self.start.add(Point { x: 0, y }), Rgb888::CSS_DARK_GRAY).draw(target)?;
        Ok(())
    }
}
