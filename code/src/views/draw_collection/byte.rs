use std::fmt::Debug;

use embedded_graphics::mono_font::ascii::FONT_5X8;
use embedded_graphics::text::{Alignment, Baseline, TextStyleBuilder};
use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::Rgb888, prelude::*, text::Text};

use crate::views::draw_collection::logic_signals::bit_sequence::{BitSequence, BitTypes};

#[derive(Debug)]
pub struct Byte {
    pub byte: u8,
    pub start: Point,
    pub bit_width: i32,
    pub height: i32,
}

impl Byte {
    pub fn new(bits: u8, start: Point) -> Self {
        Byte {
            byte: bits,
            start,
            bit_width: 6,
            height: 9,
        }
    }
    fn get_bit(&self, bit_nr: i32) -> bool {
        return self.byte >> bit_nr & 1 == 1;
    }
    fn get_x_for_bit(&self, bit_nr: i32) -> i32 {
        return self.start.x + bit_nr * self.bit_width;
    }
}

impl Drawable for Byte {
    type Color = Rgb888;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Rgb888>,
    {
        BitSequence {
            bits: BitTypes::OnlyByte(self.byte),
            start: self.start,
            bit_width: self.bit_width,
            height: self.height,
        }
        .draw(target)?;

        let text_style = TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Middle)
            .build();
        for i in 0..8 {
            let mut color = Rgb888::RED;
            let mut text = "0";
            if self.get_bit(i) {
                color = Rgb888::GREEN;
                text = "1";
            }
            let x: i32 = self.get_x_for_bit(i);
            Text::with_text_style(
                text,
                Point::new(x + self.bit_width / 2, self.start.y + self.height / 2),
                MonoTextStyle::new(&FONT_5X8, color),
                text_style,
            )
            .draw(target)?;
        }
        Ok(())
    }
}
