use std::sync::{Mutex, MutexGuard};
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};

use once_cell::sync::Lazy;

use super::framebuffer::FrameBuffer;

pub struct Layer {
    layer_mode: LayerMode,
    draw_order: u8,
    transparent_black: bool,
    framebuf: Lazy<Mutex<FrameBuffer>>
}

impl Layer {
    pub fn layer_mode(&self) -> LayerMode {
        self.layer_mode
    }

    pub fn set_layer_mode(& mut self, layer_mode: LayerMode) {
        self.layer_mode = layer_mode;
    }

    pub fn draw_order(&self) -> u8 {
        self.draw_order
    }

    pub fn set_draw_order(&mut self, draw_order: u8) {
        self.draw_order = draw_order
    }

    pub fn transparent_black(&self) -> bool {
        self.transparent_black
    }

    pub fn set_transparent_black(&mut self, transparent_black: bool) {
        self.transparent_black = transparent_black
    }

    pub fn framebuf(&self) -> MutexGuard<'_, FrameBuffer> {
        self.framebuf.lock().unwrap()
    }

    pub fn set_size(&mut self, width: usize, height: usize) {
        self.framebuf().resize(width, height, Rgb888::BLACK);
    }
 }

#[derive(Copy, Clone)]
pub enum LayerMode {
    DirectDraw,
    Animated,
    Script
}

pub static DEFAULT_LAYER_0: Lazy<Mutex<Layer>> = Lazy::new(|| Mutex::new(Layer {
    layer_mode: LayerMode::DirectDraw,
    draw_order: 0,
    transparent_black: false,
    framebuf: Lazy::new(|| Mutex::new(FrameBuffer::new_empty()))
}));

pub static DEFAULT_LAYER_1: Lazy<Mutex<Layer>> = Lazy::new(|| Mutex::new(Layer {
    layer_mode: LayerMode::DirectDraw,
    draw_order: 1,
    transparent_black: true,
    framebuf: Lazy::new(|| Mutex::new(FrameBuffer::new_empty()))
}));

pub static DEFAULT_LAYER_2: Lazy<Mutex<Layer>> = Lazy::new(|| Mutex::new(Layer {
    layer_mode: LayerMode::Animated,
    draw_order: 2,
    transparent_black: true,
    framebuf: Lazy::new(|| Mutex::new(FrameBuffer::new_empty()))
}));