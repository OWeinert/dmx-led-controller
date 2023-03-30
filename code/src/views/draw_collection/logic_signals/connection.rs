use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{Rgb888, WebColors};
use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use embedded_graphics::Drawable;

pub struct Connection {
    pub start: Point,
    pub height: i32,
    pub bit_states: (bool, bool),
}

impl Drawable for Connection {
    type Color = Rgb888;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Rgb888>,
    {
        // draw connections
        let y_low = self.start.y + self.height;
        let y_high = self.start.y;
        let (y0, y1) = match self.bit_states {
            (true, true) => (y_high, y_high),
            (false, false) => (y_low, y_low),
            _ => (y_low, y_high),
        };

        Line::new(Point::new(self.start.x, y0), Point::new(self.start.x, y1))
            .into_styled(PrimitiveStyle::with_stroke(Rgb888::CSS_DARK_GRAY, 1))
            .draw(target)?;

        Ok(())
    }
}
