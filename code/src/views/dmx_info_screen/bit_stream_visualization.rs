use embedded_graphics::mono_font::iso_8859_10::FONT_4X6;
use embedded_graphics::primitives::Line;
use embedded_graphics::text::Alignment;
use embedded_graphics::{
    mono_font::MonoTextStyle, pixelcolor::Rgb888, prelude::*, primitives::PrimitiveStyle,
    text::Text,
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct BitStreamVisualization {
    bits: u8,
    start: Point,
    bit_width: i32,
    height: i32,
}

impl BitStreamVisualization {
    pub fn new(bits: u8, start: Point) -> Self {
        BitStreamVisualization {
            bits,
            start,
            bit_width: 6,
            height: 8,
        }
    }
    fn get_bit(&self, bit_nr: i32) -> bool {
        return self.bits >> bit_nr & 1 == 1;
    }
    fn get_x_for_bit(&self, bit_nr: i32) -> i32 {
        return self.start.x + bit_nr * self.bit_width;
    }
    fn get_y_for_bit(&self, bit_nr: i32) -> i32 {
        let y_start = self.start.y;
        if self.get_bit(bit_nr) {
            return y_start;
        }
        return y_start + self.height;
    }
}

impl Drawable for BitStreamVisualization {
    type Color = Rgb888;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Rgb888>,
    {
        for i in 0..8 {
            let mut color = Rgb888::RED;
            let mut text = "0";
            if self.get_bit(i) {
                color = Rgb888::GREEN;
                text = "1";
            }

            let x: i32 = self.get_x_for_bit(i);
            Text::with_alignment(
                text,
                Point::new(x + self.bit_width / 2, self.start.y + self.height / 2 + 2),
                MonoTextStyle::new(&FONT_4X6, color),
                Alignment::Center,
            )
            .draw(target)?;

            let y: i32 = self.get_y_for_bit(i);
            Line::new(Point::new(x, y), Point::new(x + self.bit_width, y))
                .into_styled(PrimitiveStyle::with_stroke(Rgb888::WHITE, 1))
                .draw(target)?;
        }

        // draw connections
        for i in 0..7 {
            let x = self.get_x_for_bit(i + 1);
            Line::new(
                Point::new(x, self.get_y_for_bit(i)),
                Point::new(x, self.get_y_for_bit(i + 1)),
            )
            .into_styled(PrimitiveStyle::with_stroke(Rgb888::CSS_DARK_GRAY, 1))
            .draw(target)?;
        }
        // draw ends
        for (bit_index, bit_height) in [(0, 0), (8, 7)] {
            Pixel(
                Point::new(self.get_x_for_bit(bit_index), self.get_y_for_bit(bit_height)),
                Rgb888::CSS_DARK_GRAY,
            )
            .draw(target)?;
        }

        Ok(())
    }
}
