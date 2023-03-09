#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod render_engine;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};

fn main() {
    let args = ["","--driver", "fx2lafw", "--show"].map(|arg| CString::new(arg).unwrap() );
    let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();

    unsafe {
        mainC(c_args.len() as c_int, c_args.as_ptr() as *mut *mut i8);
    };
}
