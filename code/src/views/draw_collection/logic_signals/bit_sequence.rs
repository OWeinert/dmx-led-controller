use std::ops::Add;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::Drawable;

use super::connection::*;
use super::ends::*;
use super::logic_level::*;

pub enum BitTypes {
    OnlyByte(u8),
    FullUart(u16),
}

pub struct BitSequence {
    pub bits: BitTypes,
    pub start: Point,
    pub bit_width: i32,
    pub height: i32,
}

impl BitSequence {
    fn get_bit(&self, bit_nr: i32) -> bool {
        return match self.bits {
            BitTypes::OnlyByte(bits) => bits >> bit_nr & 1 == 1,
            BitTypes::FullUart(bits) => bits >> (10 - bit_nr) & 1 == 1,
        };
    }
    fn get_x_for_bit(&self, bit_nr: i32) -> i32 {
        return self.start.x + bit_nr * self.bit_width;
    }
    fn get_color(&self, bit_nr: i32) -> Rgb888 {
        return match self.bits {
            BitTypes::OnlyByte(_) => Rgb888::WHITE,
            BitTypes::FullUart(_) => match bit_nr {
                0 => Rgb888::GREEN,
                9 | 10 => Rgb888::RED,
                _ => Rgb888::WHITE,
            },
        };
    }
}

impl Drawable for BitSequence {
    type Color = Rgb888;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Rgb888>,
    {
        let end_bit = match self.bits {
            BitTypes::OnlyByte(..) => 8,
            BitTypes::FullUart(..) => 11,
        };
        for i in 0..end_bit {
            LogicLevel {
                start: Point {
                    x: self.get_x_for_bit(i),
                    y: self.start.y,
                },
                length: self.bit_width,
                height: self.height,
                bit_state: self.get_bit(i),
                color: self.get_color(i),
            }
            .draw(target)?;
        }

        // draw connections
        for i in 0..end_bit - 1 {
            let x = self.get_x_for_bit(i + 1);
            Connection {
                start: Point { x, y: self.start.y },
                height: self.height,
                bit_states: (self.get_bit(i), self.get_bit(i + 1)),
            }
            .draw(target)?;
        }

        // draw ends
        for (bit_index, bit_height) in [(0, 0), (end_bit, end_bit - 1)] {
            let offset = Point {
                x: bit_index * self.bit_width,
                y: 0,
            };
            Ends {
                height: self.height,
                start: self.start.add(offset),
                state: self.get_bit(bit_height),
            }
            .draw(target)?;
        }

        Ok(())
    }
}
