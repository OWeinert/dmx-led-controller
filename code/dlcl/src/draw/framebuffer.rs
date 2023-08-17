use std::{slice::Iter, sync::Mutex};
use std::iter::Map;
use std::vec::IntoIter;

use embedded_graphics::{pixelcolor::Rgb888, prelude::{Point, DrawTarget, PixelIteratorExt, RgbColor}, Pixel};
use once_cell::sync::Lazy;

#[derive(Clone)]
pub struct FrameBuffer {
    data: Vec<Rgb888>,
    width: usize,
    height: usize
}

impl FrameBuffer {

    /// Creates a new framebuffer of size 0
    ///
    /// ## Returns
    ///
    /// 'Self' - The framebuffer
    ///
    pub fn new_empty() -> Self {
        let frame_buf = FrameBuffer {
            data: vec![],
            width: 0,
            height: 0
        };
        frame_buf
    }
    
    /*
    fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let frame_buf = FrameBuffer {
            data: vec![Rgb888::BLACK; size],
            width: width,
            height: height
        };
        frame_buf
    }
    */

    /// Resizes the framebuffer to *new_width* and *new_height*
    /// and fills empty positions with the *fill_color*
    ///
    /// ## Arguments
    ///
    /// * 'self' - The framebuffer
    /// * 'new_width' - The new width
    /// * 'new_height' - The new height
    /// * 'fill_color' - The fill color for empty positions
    ///
    pub fn resize(&mut self, new_width: usize, new_height: usize, fill_color: Rgb888) {
        let new_size = new_width * new_height;
        self.data.resize(new_size, fill_color);
        self.width = new_width;
        self.height = new_height;
    }

    pub fn iter(&self) -> Iter<'_, Rgb888> {
        self.data.iter()
    }

    pub fn clear(&mut self) {
        self.data.fill_with(|| Rgb888::BLACK);
    }

    /// Sets the *color* at *pos* in the framebuffer
    ///
    /// ## Arguments
    ///
    /// * 'self' - The framebuffer
    /// * 'pos' - The position
    /// * 'color' - The color
    ///
    pub fn set_pixel_color(&mut self, pos: Point, color: Rgb888) {
        let width = self.width as i32;
        let i: usize = (pos.y * width + pos.x) as usize;
        self.data[i] = color;
    }

    pub fn data(&mut self) -> &mut Vec<Rgb888> {
        &mut self.data
    }

    /// Converts the framebuffer into an iterator containing
    /// the pixels
    ///
    /// ## Arguments
    ///
    /// * 'self' - the framebuffer
    ///
    /// ## Returns
    ///
    /// 'IntoIter\<Pixel\<Rgb888\>\>' - The pixel iterator
    ///
    pub fn to_pixels(&mut self) -> IntoIter<Pixel<Rgb888>>{
        let buf_iter = self.iter();
        let mut i = 0;
        let width: i32 = self.width as i32;
        let height: i32 = self.height as i32;
        let pixels = buf_iter.map(|color| {
            let x: i32 = i % width;
            let y: i32 = i / height;
            i += 1;
            return Pixel(Point::new(x, y), color.to_owned());
        }).collect::<Vec<Pixel<Rgb888>>>().into_iter();
        pixels
    }
}

/// The global framebuffer
pub static GLOBALE_FRAMEBUF: Lazy<Mutex<FrameBuffer>> = Lazy::new(|| Mutex::new(FrameBuffer::new_empty()));

/// Sets the size of the frame buffer
/// 
/// ## Arguments
/// 
/// * 'width' - Width of the frame buffer
/// * 'height' - Height of the frame buffer
/// 
pub fn set_framebuf_size(width: usize, height: usize) {
    GLOBALE_FRAMEBUF.lock().unwrap().resize(width, height, Rgb888::BLACK);
}

/// Draws the frame buffer to the *target*
/// 
/// ## Arguments
/// 
/// * 'target' - The draw target
/// 
pub fn draw_framebuf<D>(target: &mut D)
where 
    D: DrawTarget<Color = Rgb888> {
        
    let mut buf = GLOBALE_FRAMEBUF.lock().unwrap();
    let pixels = buf.to_pixels();
    match pixels.draw(target) {
        Ok(_) => {},
        Err(_) => println!("Failed to draw frame buffer!")
    };
}

/// Clears the frame buffer
/// 
pub fn clear_framebuf() {
    GLOBALE_FRAMEBUF.lock().unwrap().clear();
}