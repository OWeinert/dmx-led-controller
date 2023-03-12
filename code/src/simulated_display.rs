#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod render_engine;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};

fn main() {
    unsafe {
        mainC();
    };
}
