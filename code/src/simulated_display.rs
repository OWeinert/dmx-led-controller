#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod render_engine;
mod logicAnalyzer;

use std::sync::mpsc;
use std::thread;
use crate::logicAnalyzer::start_logic_analyzer;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        start_logic_analyzer(tx);
    });

    loop {
        let received = rx.try_recv();
        if received.is_ok() {
            let received = received.unwrap();
            println!("received: {}", received);
        }
    }
}
