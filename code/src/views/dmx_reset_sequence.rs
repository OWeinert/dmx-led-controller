use std::fmt::Debug;
use std::time::Duration;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::Drawable;

use super::dmx_channel_1::ParameterDmxInfoScreen;
use super::draw_collection::slot::Slot;
use super::draw_collection::text::label::Label;
use super::draw_collection::text::sample_rate::SampleRateHM;
use super::draw_collection::text::text::TextSmall;

pub struct DmxResetSequence {}

impl DmxResetSequence {
    pub fn on_user_update<D>(&mut self, display: &mut D, parameter: ParameterDmxInfoScreen)
    where
        D: DrawTarget<Color = Rgb888>,
        D::Error: Debug,
    {
        TextSmall {
            text: "Reset Sequence",
            color: Rgb888::WHITE,
            start: Point::new(2, 6),
        }
        .draw(display)
        .unwrap();

        Slot::new(Point { x: 2, y: 10 }).draw(display).unwrap();

        let period_duration =
            Duration::from_nanos((1.0 / parameter.frequency.as_gigahertz()) as u64);
        let duration = Duration::from_nanos(
            parameter.dmx_output.reset_sequence.space_for_break * period_duration.as_nanos() as u64,
        );
        Label::from_duration(11, Point::new(1 + 7, 21), duration)
            .draw(display)
            .unwrap();
        let duration = Duration::from_nanos(
            parameter.dmx_output.reset_sequence.mark_after_break
                * period_duration.as_nanos() as u64,
        );
        Label::from_duration(6, Point::new(1 + 17, 31), duration)
            .draw(display)
            .unwrap();

        TextSmall {
            text: "Start/",
            color: Rgb888::GREEN,
            start: Point::new(1, 46),
        }
        .draw(display)
        .unwrap();
        TextSmall {
            text: "Stop",
            color: Rgb888::RED,
            start: Point::new(25, 46),
        }
        .draw(display)
        .unwrap();
        TextSmall {
            text: "Bits",
            color: Rgb888::WHITE,
            start: Point::new(41, 46),
        }
        .draw(display)
        .unwrap();

        SampleRateHM {
            frequency: parameter.frequency,
        }
        .draw(display)
        .unwrap();
    }
}
