use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use crate::draw;

pub mod clayer;
pub mod cframebuffer;

#[no_mangle]
#[repr(C)]
pub struct CRgb888 {
    r: u8,
    g: u8,
    b: u8,
}

impl CRgb888 {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        CRgb888 { r, g, b }
    }

    pub fn r(&self) -> u8 {  self.r }

    pub fn g(&self) -> u8 {self.g }

    pub fn b(&self) -> u8 {self.b }
}

impl From<Rgb888> for CRgb888 {
    fn from(rgb888: Rgb888) -> Self {
        CRgb888::new(rgb888.r(), rgb888.g(), rgb888.b())
    }
}

impl From<CRgb888> for Rgb888 {
    fn from(crgb888: CRgb888) -> Self {
        Rgb888::new(crgb888.r(), crgb888.g(), crgb888.b())
    }
}

#[no_mangle]
pub extern "C" fn c_draw_pixel_direct(x: i32, y: i32, color: CRgb888) {
    draw::draw_pixel_direct(Point::new(x, y), color.into());
}