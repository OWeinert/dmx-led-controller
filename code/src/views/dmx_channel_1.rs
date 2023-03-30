use std::fmt::Debug;
use std::time::Duration;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::iso_8859_1::FONT_6X9;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::text::{Baseline, Text};
use embedded_graphics::Drawable;
use measurements::Frequency;

use crate::dmx_analyzer::dmx_state_machine::DmxOutput;

use super::draw_collection::text::sample_rate::SampleRateHM;
use super::draw_collection::byte::*;
use super::draw_collection::text::label::*;

pub struct ParameterDmxInfoScreen {
    pub dmx_output: DmxOutput,
    pub frequency: Frequency,
}

pub struct DmxInfoScreen {}

impl DmxInfoScreen {
    pub fn on_user_update<D>(&mut self, display: &mut D, parameter: ParameterDmxInfoScreen)
    where
        D: DrawTarget<Color = Rgb888>,
        D::Error: Debug,
    {
        let mut bit_stream = BitStream {
            start: Point { x: 1, y: 1 },
        };
        bit_stream.on_user_update(display, parameter);
    }
}

struct BitStream {
    start: Point,
}

impl BitStream {
    pub fn on_user_update<D>(&mut self, display: &mut D, parameter: ParameterDmxInfoScreen)
    where
        D: DrawTarget<Color = Rgb888>,
        D::Error: Debug,
    {
        let mut y = self.start.y + 1;
        let character_large_style = MonoTextStyle::new(&FONT_6X9, Rgb888::WHITE);
        Text::with_baseline(
            &*format!("Ch1:{}", parameter.dmx_output.channels[0]),
            Point::new(self.start.x + 1, y),
            character_large_style,
            Baseline::Top,
        )
        .draw(display)
        .unwrap();

        y = y + 13;
        let bit_x = 7;
        Byte::new(parameter.dmx_output.channels[0], Point { x: bit_x, y })
            .draw(display)
            .unwrap();
        y = y + 12;
        let period_duration =
            Duration::from_nanos((1.0 / parameter.frequency.as_gigahertz()) as u64);
        let samples =
            parameter.dmx_output.bits[0].end_sample - parameter.dmx_output.bits[0].start_sample;
        let duration = Duration::from_nanos(samples * period_duration.as_nanos() as u64);
        Label::from_duration(6 + 1, Point { x: bit_x, y }, duration)
            .draw(display)
            .unwrap();
        y = y + 10;
        let samples =
            parameter.dmx_output.bits[7].end_sample - parameter.dmx_output.bits[0].start_sample;
        let duration = period_duration.clone().mul_f32(samples as f32);
        Label::from_duration(6 * 8 + 1, Point { x: bit_x, y }, duration)
            .draw(display)
            .unwrap();
        SampleRateHM {
            frequency: parameter.frequency,
        }
        .draw(display)
        .unwrap();
    }
}
