use std::sync::mpsc::{Sender};

#[repr(C)]
struct RustObject {
    a: i32,
    b: u32,
    sender: Sender<u16>
}

extern "C" fn callback(target: *mut RustObject, a: i32) {
    unsafe {
        (*target).a = a;
        (*target).sender.send(a as u16).expect("couldnt send data");
    }
}

#[link(name = "saleaeLogic", kind = "static")]
extern {
    fn mainC(target: *mut RustObject, cb: extern fn(*mut RustObject, i32)) -> i32;
}

pub fn start_logic_analyzer(tx: Sender<u16>) {

    // Create the object that will be referenced in the callback:
    let mut rust_object = Box::new(RustObject { a: 5, b: 4,  sender: tx});
    unsafe {
        mainC(&mut *rust_object, callback);
    }
    println!("mama, ich habe fertig!");
}
