use pyo3::prelude::*;

#[pyclass]
struct PyRgb888 {
    r: u8,
    g: u8,
    b: u8
}

#[pyfunction]
fn draw_pixel(x: usize, y: usize, color: &PyRgb888) {
    
}

#[pymodule]
fn dlcl(py: Python<'_>, m: &PyModule) -> PyResult<()> {


    Ok(())
}
