pub mod framebuffer;
pub mod layer;
pub mod animation;

use std::collections::HashSet;
use std::convert::Infallible;
use std::ops::Deref;
use std::sync::{Mutex, MutexGuard};
use embedded_graphics::{prelude::*, pixelcolor::Rgb888, primitives::{PrimitiveStyle, Line, Circle}};
use embedded_graphics_simulator::sdl2::Keycode::Mute;
use iter_tools::Itertools;
use once_cell::sync::{Lazy, OnceCell};
use crate::draw::layer::{AnimatedLayer, Layer};
use crate::draw::framebuffer::GLOBAL_FRAMEBUF;

/// Draws a pixel with *color* at *pos* in the frame buffer
/// 
/// ## Arguments
/// 
/// * 'pos' - The pixel position
/// * 'color' - The pixel color
/// 
pub fn draw_pixel_direct(pos: Point, color: Rgb888) {
    GLOBAL_FRAMEBUF.lock().unwrap().set_pixel_color(pos, color);
}

/// Draws a line with *color* from *start_pos* 
/// to *end_pos* to the frame buffer
/// 
/// ## Arguments
/// 
/// * 'start_pos' - Start position of the line
/// * 'end_pos' - End position of the line
/// * 'style' - The draw style. Also contains the line's color
/// 
pub fn draw_line_direct(start_pos: Point, end_pos: Point, style: PrimitiveStyle<Rgb888>) {
    let mut buf = GLOBAL_FRAMEBUF.lock().unwrap();
    Line::new(start_pos, end_pos)
        .into_styled(style).pixels()
        .for_each(|p| {
            buf.set_pixel_color(p.0, p.1);
        });
}

/// Draws a circle of *diameter* at *top_left* in the frame buffer
/// 
/// ## Arguments
/// 
/// * 'top_left' - Top left position of the circle
/// * 'diameter' - The circle diameter
/// * 'style' - The draw style. Also contains the circle's color
/// 
pub fn draw_circle_direct(top_left: Point, diameter: u32, style: PrimitiveStyle<Rgb888> ){
    let mut buf = GLOBAL_FRAMEBUF.lock().unwrap();
    Circle::new(top_left, diameter)
            .into_styled(style).pixels()
            .for_each(|p| {
                buf.set_pixel_color(p.0, p.1);
            });
}


/// Draws a pixel with color at pos in the *layer*
///
/// ## Arguments
///
/// * 'pos' - The pixel position
/// * 'color' - The pixel color
///
pub fn draw_pixel_layer(pos: Point, color: Rgb888, layer: &mut dyn Layer) {
    let mut buf = layer.framebuf();
    buf.set_pixel_color(pos, color);
}

/// Draws a line with *color* from *start_pos*
/// to *end_pos* on the layer
///
/// ## Arguments
///
/// * 'start_pos' - Start position of the line
/// * 'end_pos' - End position of the line
/// * 'style' - The draw style. Also contains the line's color
/// * 'layer' - The layer
///
pub fn draw_line_layer(start_pos: Point, end_pos: Point, style: PrimitiveStyle<Rgb888>, layer: &mut dyn Layer) {
    let mut buf = layer.framebuf();
    Line::new(start_pos, end_pos)
        .into_styled(style).pixels()
        .for_each(|p| {
            buf.set_pixel_color(p.0, p.1);
        });
}

/// Draws a circle of *diameter* at *top_left* on the layer
///
/// ## Arguments
///
/// * 'top_left' - Top left position of the circle
/// * 'diameter' - The circle diameter
/// * 'style' - The draw style. Also contains the circle's color
/// * 'layer' - The layer
///
pub fn draw_circle_layer(top_left: Point, diameter: u32, style: PrimitiveStyle<Rgb888> , layer: &mut dyn Layer){
    let mut buf = layer.framebuf();
    Circle::new(top_left, diameter)
        .into_styled(style).pixels()
        .for_each(|p| {
            buf.set_pixel_color(p.0, p.1);
        });
}
/// Draws a *layer* to the global framebuffer
///
/// ## Arguments
///
/// * 'layer' - The layer
///
pub fn draw_layer(layer: &mut dyn Layer) {
    let mut global_buf = GLOBAL_FRAMEBUF.lock().unwrap();
    layer.prep_layer();
    layer.framebuf().to_pixels()
        .for_each(|p| {
            // only draw the color Black to the framebuffer if transparent_black is enabled on the layer
            if (layer.transparent_black() && p.1 != Rgb888::BLACK) || !layer.transparent_black() {
                global_buf.set_pixel_color(p.0, p.1);
            }
    });
}