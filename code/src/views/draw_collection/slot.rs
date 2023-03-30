use std::ops::Add;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::Drawable;

use crate::views::draw_collection::logic_signals::bit_sequence::{BitSequence, BitTypes};
use crate::views::draw_collection::logic_signals::proceeding_signal::ProceedingSignal;

use super::logic_signals::connection::*;
use super::logic_signals::logic_level::*;

#[derive(Debug)]
pub struct Slot {
    start: Point,
    height: i32,
}

impl Slot {
    pub fn new(start: Point) -> Self {
        Slot { start, height: 9 }
    }
}

impl Drawable for Slot {
    type Color = Rgb888;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Rgb888>,
    {
        let mut x_offset = 0;

        x_offset += 4;
        let logic_levels = [2, 10, 5];

        let mut offset = x_offset;
        let mut logic_level = true;
        for length in logic_levels {
            LogicLevel {
                start: self.start.add(Point { x: offset, y: 0 }),
                height: self.height,
                bit_state: logic_level,
                length,
                color: Rgb888::WHITE,
            }
            .draw(target)?;
            offset += length;
            logic_level = !logic_level;
        }
        let mut offset = x_offset;
        for length in logic_levels {
            offset += length;
            Connection {
                start: self.start.add(Point { x: offset, y: 0 }),
                height: self.height,
                bit_states: (false, true),
            }
            .draw(target)?;
        }

        x_offset += logic_levels.iter().sum::<i32>();
        BitSequence {
            bits: BitTypes::FullUart(0b0_00000000_11),
            start: self.start.add(Point::new(x_offset, 0)),
            bit_width: 3,
            height: 9,
        }
        .draw(target)?;
        x_offset += 11 * 3;
        ProceedingSignal {
            start: self.start.add(Point::new(0, 0)),
            length: 5,
        }
        .draw(target)?;
        ProceedingSignal {
            start: self.start.add(Point::new(x_offset, 0)),
            length: 5,
        }
        .draw(target)?;
        //Ends{height: self.height, start: self.start.add(Point::new(4, 0)), state: true}.draw(target)?;

        Ok(())
    }
}
