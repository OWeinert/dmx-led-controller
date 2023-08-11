use std::{path::Path, fs};

use pyo3::{PyAny, Py, Python, types::PyModule, PyResult};

/// Loads the Python script at the given path string into a Py\<PyModule\> smart pointer
/// 
/// ## Arguments
/// 
/// * 'path' - A String object containing the path
/// 
/// ## Returns
/// 
/// * Py\<PyModule\> - The Python scrip
/// 
pub fn load_py_script_str(path: String) -> Py<PyModule> {
    let py_path = Path::new(&path);
    return load_py_script(py_path);
}

/// Loads the Python script at the given path into a Py\<PyModule\> smart pointer
/// 
/// ## Arguments
/// 
/// * 'path' - A String object containing the path
/// 
/// ## Returns
/// 
/// * Py\<PyModule\> - The Python scrip
/// 
pub fn load_py_script(path: &Path) -> Py<PyModule> {
    let py_file = match fs::read_to_string(path) {
        Ok(p) => p,
        Err(error) => {
            println!("Python file not found! path: {} || err: {}", path.to_string_lossy(), error);
            panic!();
        }
    };
    let from_python: Py<PyModule> = Python::with_gil(|py| {
        let script: Py<PyModule> = PyModule::from_code(py, &py_file, "", "")
            .expect("Failed to build PyModule!")
            .into();
        return script;
    });
    return from_python;
}

/// Calls the "setup" function in the given PyModule
/// 
/// ## Arguments
/// 
/// * 'py_module' - The Python script
pub fn call_setup(py_module: &Py<PyModule>) { 
    match call_func0(py_module, "setup") {
        Ok(_) => return,
        Err(error) => println!("Failed to call function \"setup\" ! {}", error)
    }
}

/// Calls the "loop" function in the given PyModule
/// 
/// ## Arguments
/// 
/// * 'py_module' - The Python script
pub fn call_loop(py_module: &Py<PyModule>) {
    match call_func0(py_module, "loop") {
        Ok(_) => return,
        Err(error) => println!("Failed to call function \"loop\" ! {}", error)
    }
}

/// Calls the give function in the given PyModule
/// 
/// ## Arguments
/// 
/// * 'py_module' - The Python script
/// * 'func_name' - The name of the function
/// 
/// ## Returns
/// 
/// * PyResult\<Py\<PyAny\>\> - The return value of the function
/// 
pub fn call_func0(py_module: &Py<PyModule>, func_name: &str) -> PyResult<Py<PyAny>> {
    let result = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let py_module = py_module.as_ref(py);
        let py_setup: Py<PyAny> = py_module.getattr(func_name)?.into();
        py_setup.call0(py)
    });
    return result;
}