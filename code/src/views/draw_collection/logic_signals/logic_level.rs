use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use embedded_graphics::Drawable;

pub struct LogicLevel {
    pub start: Point,
    pub length: i32,
    pub height: i32,
    pub bit_state: bool,
    pub color: Rgb888,
}

impl Drawable for LogicLevel {
    type Color = Rgb888;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Rgb888>,
    {
        let x: i32 = self.start.x;
        let y: i32 = if self.bit_state {
            self.start.y
        } else {
            self.start.y + self.height
        };

        Line::new(Point::new(x, y), Point::new(x + self.length, y))
            .into_styled(PrimitiveStyle::with_stroke(self.color, 1))
            .draw(target)?;

        Ok(())
    }
}
