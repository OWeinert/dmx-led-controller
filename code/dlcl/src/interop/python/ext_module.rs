use embedded_graphics::{pixelcolor::Rgb888, prelude::Point};
use pyo3::prelude::*;

use crate::draw;

#[pyclass]
struct PyRgb888 {
    r: u8,
    g: u8,
    b: u8
}

impl PyRgb888 {
    fn to_rgb888(&self) -> Rgb888 {
        Rgb888::new(self.r, self.g, self.b)
    }
}

/// Python binding for *draw::draw_pixel*
/// 
#[pyfunction]
fn draw_pixel(x: i32, y: i32, color: &PyRgb888) {
    let rs_col = color.to_rgb888();
    draw::draw_pixel(Point::new(x, y), rs_col);
}

/// The Python module configuration for dlcl
/// 
#[pymodule]
fn dlcl(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(draw_pixel, m)?)?;
    Ok(())
}
