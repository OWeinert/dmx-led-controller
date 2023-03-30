use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::iso_8859_1::FONT_4X6;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;

pub struct TextSmall<'a> {
    pub(crate) text: &'a str,
    pub color: Rgb888,
    pub start: Point,
}

impl Drawable for TextSmall<'_> {
    type Color = Rgb888;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Rgb888>,
    {
        Text::new(
            self.text,
            self.start,
            MonoTextStyle::new(&FONT_4X6, self.color),
        )
        .draw(target)?;

        Ok(())
    }
}
