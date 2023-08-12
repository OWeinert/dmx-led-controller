use std::{sync::Mutex, slice::Iter};

use embedded_graphics::{pixelcolor::Rgb888, prelude::{RgbColor, DrawTarget, Point, PixelIteratorExt}, Pixel, primitives::{Line, StyledDrawable, PrimitiveStyle, PointsIter, Circle, Primitive}};
use once_cell::sync::Lazy;

pub struct FrameBuffer {
    data: Vec<Rgb888>,
    width: usize,
    height: usize
}

impl FrameBuffer {
    fn new_empty() -> Self {
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

    fn resize(&mut self, new_width: usize, new_height: usize, fill_color: Rgb888) {
        let new_size = new_width * new_height;
        self.data.resize(new_size, fill_color);
        self.width = new_width;
        self.height = new_height;
    }

    fn iter(&self) -> Iter<'_, Rgb888> {
        self.data.iter()
    }

    fn clear(&mut self) {
        self.data.fill_with(|| Rgb888::BLACK);
    }

    fn set_pixel_color(&mut self, pos: Point, color: Rgb888) {
        let width = self.width as i32;
        let i: usize = (pos.y * width + pos.x) as usize;
        self.data[i] = color;
    }

}

static FRAME_BUF: Lazy<Mutex<FrameBuffer>> = Lazy::new(|| Mutex::new(FrameBuffer::new_empty()));

/// Sets the size of the frame buffer
/// 
/// ## Arguments
/// 
/// * 'width' - Width of the frame buffer
/// * 'height' - Height of the frame buffer
/// 
pub fn set_framebuf_size(width: usize, height: usize) {
    FRAME_BUF.lock().unwrap().resize(width, height, Rgb888::BLACK);
}

/// Draws the frame buffer to the give target
/// 
/// ## Arguments
/// 
/// * 'target' - The draw target
/// 
pub fn draw_framebuf<D>(target: &mut D)
    where D: DrawTarget<Color = Rgb888> {
        let buf = FRAME_BUF.lock().unwrap();
        let buf_iter = buf.iter();
        
        let mut i = 0;
        let width: i32 = buf.width as i32;
        let height: i32 = buf.height as i32;

        let pixels = buf_iter.map(|color| {
            let x: i32 = i % width;
            let y: i32 = i / height;
            i += 1;
            return Pixel(Point::new(x, y), color.to_owned());
        });
        match pixels.draw(target) {
            Ok(_) => {},
            Err(_) => println!("Failed to draw frame buffer!")
        };
}

/// Clears the frame buffer
/// 
pub fn clear_framebuf() {
    FRAME_BUF.lock().unwrap().clear();
}

/// Draws a pixel with *color* at *pos* in the frame buffer
/// 
/// ## Arguments
/// 
/// * 'pos' - The pixel position
/// * 'color' - The pixel color
/// 
pub fn draw_pixel(pos: Point, color: Rgb888) {
    FRAME_BUF.lock().unwrap().set_pixel_color(pos, color);
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
pub fn draw_line(start_pos: Point, end_pos: Point, style: PrimitiveStyle<Rgb888>) {
    let mut buf = FRAME_BUF.lock().unwrap();
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
pub fn draw_circle(top_left: Point, diameter: u32, style: PrimitiveStyle<Rgb888> ){
    let mut buf = FRAME_BUF.lock().unwrap();
    Circle::new(top_left, diameter)
            .into_styled(style).pixels()
            .for_each(|p| {
                buf.set_pixel_color(p.0, p.1);
            });
}