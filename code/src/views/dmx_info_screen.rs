use std::fmt::Debug;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::{Drawable};
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::iso_8859_1::FONT_4X6;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::text::{Alignment, Text};

mod bit_stream_visualization;
use bit_stream_visualization::*;
mod length;
use length::*;

pub struct ParameterDmxInfoScreen {

}

pub struct DmxInfoScreen {

}

impl DmxInfoScreen {
    pub fn on_user_update<D>(&mut self, display: &mut D, parameter: ParameterDmxInfoScreen)
        where
            D: DrawTarget<Color=Rgb888>,
            D::Error: Debug,
    {
        let mut bit_stream = BitStream{ start: Point{x: 1, y: 1}};
        bit_stream.on_user_update(display, 0b10011000);

    }
}

struct BitStream {
    start: Point,
}

impl BitStream {
    pub fn on_user_update<D>(&mut self, display: &mut D, channel_value: u8)  where
        D: DrawTarget<Color=Rgb888>,
        D::Error: Debug,
    {
        let mut y = self.start.y + 4;
        let style = MonoTextStyle::new(&FONT_4X6, Rgb888::WHITE);
        Text::new(
            "Ch1:155",
            Point::new(self.start.x, y),
            style
        )
            .draw(display)
            .unwrap();

        y = y + 3;
        BitStreamVisualization::new(channel_value, Point{ x: 1, y}).draw(display).unwrap();
        y = y + 10;
        LengthVisualization::new((0, 6 + 1), Point{x: 1, y}).draw(display).unwrap();
        y = y + 10;
        LengthVisualization::new((0, 6 * 8 + 1), Point{x: 1, y}).draw(display).unwrap();
        y = y + 25;
        Text::with_alignment(
            "Samplerate: \n2MHz; +/- 2µs\n",
            Point::new(self.start.x + 58, y),
            style,
            Alignment::Right
        )
            .draw(display)
            .unwrap();
    }
}
