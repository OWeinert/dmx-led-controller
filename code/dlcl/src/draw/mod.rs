pub mod framebuffer;
pub mod layer;

use std::collections::HashSet;
use std::convert::Infallible;
use std::sync::{Mutex, MutexGuard};
use embedded_graphics::{prelude::*, pixelcolor::Rgb888, primitives::{PrimitiveStyle, Line, Circle}};
use embedded_graphics_simulator::sdl2::Keycode::Mute;
use once_cell::sync::{Lazy, OnceCell};
use crate::draw::layer::Layer;
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
pub fn draw_pixel_layer(pos: Point, color: Rgb888, layer: &Layer) {
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
pub fn draw_line_layer(start_pos: Point, end_pos: Point, style: PrimitiveStyle<Rgb888>, layer: &Layer) {
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
pub fn draw_circle_layer(top_left: Point, diameter: u32, style: PrimitiveStyle<Rgb888> , layer: &Layer){
    let mut buf = layer.framebuf();
    Circle::new(top_left, diameter)
        .into_styled(style).pixels()
        .for_each(|p| {
            buf.set_pixel_color(p.0, p.1);
        });
}
/// Draws the *layers* to the global framebuffer
///
/// ## Arguments
///
/// * 'layers' - The slice of layers
///
pub fn draw_layers(layers: &mut [&Layer]) {
    let mut global_buf = GLOBAL_FRAMEBUF.lock().unwrap();

    // sort layers by draw order and check draw order for duplicates
    layers.sort_by(|a, b| a.draw_order().cmp(&b.draw_order()));
    draw_order_checkdup(layers);
    for layer in layers {
        layer.framebuf().to_pixels()
            .for_each(|p| {
            // only draw the color Black to the framebuffer if transparent_black is enabled on the layer
            if (layer.transparent_black() && p.1 != Rgb888::BLACK) || !layer.transparent_black() {
                global_buf.set_pixel_color(p.0, p.1);
            }
        });
    }
}

/// Checks draw order for duplicates
///
fn draw_order_checkdup(layers: &[&Layer]) {
    let draw_orders = layers.iter().map(|l| l.draw_order());
    let mut hashset = HashSet::new();
    let unique = draw_orders.into_iter().all(move |d| hashset.insert(d));
    assert!(unique, "Draw order contains duplicates!");
}