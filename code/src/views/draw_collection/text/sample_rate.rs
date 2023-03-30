use std::time::Duration;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::iso_8859_1::FONT_4X6;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Alignment, Baseline, LineHeight, Text, TextStyleBuilder};
use embedded_graphics::Drawable;
use measurements::Frequency;

pub struct SampleRateHM {
    pub frequency: Frequency,
}

impl Drawable for SampleRateHM {
    type Color = Rgb888;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Rgb888>,
    {
        let character_small_style = MonoTextStyle::new(&FONT_4X6, Rgb888::WHITE);
        Text::with_baseline(
            &*format!("{} MHz", self.frequency.as_megahertz()),
            Point::new(1, (64 - 1) as i32),
            character_small_style,
            Baseline::Bottom,
        )
        .draw(target)?;

        let period_duration = Duration::from_nanos((1.0 / self.frequency.as_gigahertz()) as u64);

        let text = &*format!(
            "Sampling:
            +/-{}ns",
            period_duration.as_nanos()
        );
        let text_style = TextStyleBuilder::new()
            .alignment(Alignment::Right)
            .baseline(Baseline::Bottom)
            .line_height(LineHeight::Pixels(character_small_style.line_height() + 1))
            .build();
        Text::with_text_style(
            text,
            Point::new(
                64 - 1,
                (64 - (character_small_style.line_height() + 2)) as i32,
            ),
            character_small_style,
            text_style,
        )
        .draw(target)?;
        Ok(())
    }
}
