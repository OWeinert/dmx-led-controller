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

/// The Python module configuration for dlcl
/// 
#[pymodule]
fn dlcl(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    Ok(())
}
