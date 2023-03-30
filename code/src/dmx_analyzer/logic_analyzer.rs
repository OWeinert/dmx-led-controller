use core::convert::TryFrom;
use std::convert::From;
use std::ffi::c_void;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

use derive_try_from_primitive::TryFromPrimitive;
use measurements::Frequency;
use scan_fmt::scan_fmt;

use CLib::{runAnalyzer, srd_proto_data, srd_proto_data_annotation, CallbackData};

use super::dmx_state_machine::*;

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(unused)]
mod CLib {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

struct RustData {
    sender: Sender<DecoderAnnotation>,
}

/**
 *  Defined in libsigrokdecode/decoders/dmx512/pd.py
 */
#[derive(TryFromPrimitive)]
#[repr(i32)]
#[derive(Debug, PartialEq)]
pub enum Dmx512AnnotatorCode {
    Bit = 0,
    Break = 1,
    MarkAfterBreak = 2,
    Startbit = 3,
    Stopbit = 4,
    Startcode = 5,
    Channel = 6,
    Interframe = 7,
    Interpacket = 8,
    Data = 9,
    ErrorCode = 10,
}

#[derive(Debug)]
pub enum Dmx512AnnotatorPayload {
    Bit(bool),
    Break,
    MarkAfterBreak,
    Startbit,
    Stopbit,
    Startcode,
    ChannelNr(u16),
    InterFrame,
    InterPacket,
    Data(u8),
    ErrorCode(String),
}

#[derive(Debug)]
pub struct DecoderAnnotation {
    pub start_sample: u64,
    pub end_sample: u64,
    pub payload: Dmx512AnnotatorPayload,
}

unsafe extern "C" fn on_decoder_data(
    rust_data: *mut std::os::raw::c_void,
    decoder_protokoll_data: *mut srd_proto_data,
) {
    let protokoll_data = decoder_protokoll_data.as_ref().unwrap();
    let annotation_data = (protokoll_data.data as *const srd_proto_data_annotation)
        .as_ref()
        .unwrap();
    let annotation_payload_text =
        std::ffi::CStr::from_ptr(annotation_data.ann_text.as_ref().unwrap().as_ref().unwrap())
            .to_str()
            .unwrap();

    let annotation_type = Dmx512AnnotatorCode::try_from(annotation_data.ann_class).unwrap();

    let payload_test: Dmx512AnnotatorPayload = match annotation_type {
        Dmx512AnnotatorCode::Bit => {
            let bit = scan_fmt!(annotation_payload_text, "{1d}", u8).unwrap() == 1;
            Dmx512AnnotatorPayload::Bit(bit)
        }
        Dmx512AnnotatorCode::Channel => {
            let channel = scan_fmt!(annotation_payload_text, "Channel {3d}", u16).unwrap();
            Dmx512AnnotatorPayload::ChannelNr(channel)
        }
        Dmx512AnnotatorCode::Data => {
            let (data_dec, _) =
                scan_fmt!(annotation_payload_text, "{3d} / {4x}}", u8, [hex u8]).unwrap();
            Dmx512AnnotatorPayload::Data(data_dec)
        }
        Dmx512AnnotatorCode::ErrorCode => Dmx512AnnotatorPayload::ErrorCode(String::from("error")),
        Dmx512AnnotatorCode::Break => Dmx512AnnotatorPayload::Break,
        Dmx512AnnotatorCode::MarkAfterBreak => Dmx512AnnotatorPayload::MarkAfterBreak,
        Dmx512AnnotatorCode::Startbit => Dmx512AnnotatorPayload::Startbit,
        Dmx512AnnotatorCode::Stopbit => Dmx512AnnotatorPayload::Stopbit,
        Dmx512AnnotatorCode::Startcode => Dmx512AnnotatorPayload::Startcode,
        Dmx512AnnotatorCode::Interframe => Dmx512AnnotatorPayload::InterFrame,
        Dmx512AnnotatorCode::Interpacket => Dmx512AnnotatorPayload::InterPacket,
    };

    let annotations = DecoderAnnotation {
        start_sample: protokoll_data.start_sample,
        end_sample: protokoll_data.end_sample,
        payload: payload_test,
    };
    // println!("{:?}", annotations);
    let target = (&mut *rust_data as *mut _ as *mut RustData)
        .as_ref()
        .unwrap();
    target.sender.send(annotations).unwrap();
}

fn start_logic_analyzer(tx: Sender<DecoderAnnotation>, from_device: bool, sample: Frequency) {
    let mut rust_data = Box::new(RustData { sender: tx.clone() });
    let mut callback_data = Box::new(CallbackData {
        rustData: &mut *rust_data as *mut _ as *mut c_void,
        onDecoderAnnotation: Some(on_decoder_data),
    });
    unsafe {
        runAnalyzer(&mut *callback_data, from_device, sample.as_hertz() as u64);
    }
}

pub fn get_dmx_data(tx: Sender<DmxOutput>, from_device: bool, sample: Frequency) {
    let (tx_internal, rx_internal) = mpsc::channel();
    loop {
        let tx_clone = tx_internal.clone();
        let thread_join_handle = thread::spawn(move || {
            start_logic_analyzer(tx_clone, from_device, sample);
        });

        let mut dmx_state_machine = DmxStateMachineState::Idle();
        'receiving: loop {
            let received = rx_internal.try_recv();
            if received.is_ok() {
                let annotation = received.unwrap();
                //println!("{:?}", annotation);
                dmx_state_machine = dmx_state_machine.transition(annotation);
            }
            if thread_join_handle.is_finished() {
                if let DmxStateMachineState::End(output) = dmx_state_machine {
                    //println!("Mark after Break length: {}", mab.packet.end_sample - mab.packet.start_sample);
                    tx.send(output).unwrap();
                }
                //loop {}
                //thread::sleep(Duration::from_millis(10000));
                break 'receiving;
            }
        }
    }
}
