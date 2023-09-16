use embedded_graphics::Pixel;
use embedded_graphics::pixelcolor::Rgb888;
use crate::rpc::dlcl_rpc::{FrameDto, PixelDto};

#[derive(Clone, PartialEq)]
pub struct Frame {
    pixels: Vec<Rgb888>,
    frame_time: usize
}

impl Frame {
    pub fn new(pixels: Vec<Rgb888>, frame_time: usize) -> Self {
        let frame = Frame {
            pixels,
            frame_time
        };
        frame
    }

    pub fn pixels(&self) -> &Vec<Rgb888> {
        &self.pixels
    }

    pub fn frame_time(&self) -> usize {
        self.frame_time
    }
}

#[derive(Clone, PartialEq)]
pub struct Animation {
    frames: Vec<Frame>,
    frame_index: usize
}

impl Animation {
    pub fn frame_index(&self) -> usize {
        self.frame_index
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }

    pub fn next_frame(&mut self) -> &Frame {
        let frame = &self.frames[self.frame_index];
        if self.frame_index < self.frames.len() - 1 {
            self.frame_index += 1;
        }
        frame
    }
}