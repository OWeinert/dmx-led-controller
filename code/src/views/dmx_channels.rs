use std::fmt::Debug;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use crate::dmx_analyzer::Parameter;

pub struct DmxChannels {

}

impl DmxChannels {
    pub fn on_user_update<D>(&mut self, display: &mut D, parameter: Parameter)
        where
            D: DrawTarget<Color=Rgb888>,
            D::Error: Debug,
    {
        let _screen_width = display.bounding_box().size.width;
        let screen_height = display.bounding_box().size.height;

        let channels = parameter.channels.channels;
        for (index, value) in channels.iter().enumerate() {
            let x_start = index as i32;
            let y_start = screen_height as i32;
            let y_end = y_start - (*value as f32 * (64.0 / 255.0)) as i32;

            Line::new(Point::new(x_start, y_start), Point::new(x_start, y_end))
                .into_styled(PrimitiveStyle::with_stroke(Rgb888::WHITE, 1))
                .draw(display).unwrap();
        }
    }
}
