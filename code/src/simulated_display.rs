#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod render_engine;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};


#[repr(transparent)]
struct RustObject {
    a: i32,
    // Other members...
}

extern "C" fn callback(target: *mut RustObject, a: i32) {
    println!("I'm called from C with value {0}", a);
    unsafe {
        // Update the value in RustObject with the value received from the callback:
        (*target).a = a;
    }
}

#[link(name = "saleaeLogic", kind = "static")]
extern {
    fn mainC(target: *mut RustObject,
                         cb: extern fn(*mut RustObject, i32)) -> i32;
}

fn main() {
    // Create the object that will be referenced in the callback:
    let mut rust_object = Box::new(RustObject { a: 5 });

    unsafe {
        mainC(&mut *rust_object, callback);
    }
    println!("{}", rust_object.a);
}
