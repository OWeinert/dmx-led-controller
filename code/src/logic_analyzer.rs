#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(unused)]
mod CLib { include!(concat!(env!("OUT_DIR"), "/bindings.rs")); }

use core::convert::TryFrom;
use derive_try_from_primitive::TryFromPrimitive;
use std::ffi::{c_void};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use scan_fmt::scan_fmt;
use crate::logic_analyzer::CLib::{CallbackData, GMainLoop, runAnalyzer, srd_decoder_annotation_row, srd_proto_data, srd_proto_data_annotation};

struct RustData {
    sender: Sender<DecoderAnnotation>,
}

/**
 *  Defined in libsigrokdecode/decoders/dmx512/pd.py
 */
#[derive(TryFromPrimitive)]
#[repr(i32)]
#[derive(Debug, PartialEq)]
enum Dmx512Annotator {
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
enum AnnotationPayload {
    Data(u8),
    ChannelId(u16),
    Bit(bool),
    AnnotationError(String),
}

#[derive(Debug)]
struct DecoderAnnotation {
    start_sample: u64,
    end_sample: u64,
    annotation_type: Dmx512Annotator,
    payload: Option<AnnotationPayload>,
}

#[derive(Debug)]
pub struct DmxPacket {
    channels: Vec<u8>,
}

unsafe extern "C" fn on_decoder_data(
    rust_data: *mut std::os::raw::c_void,
    decoder_protokoll_data: *mut srd_proto_data,
) {
    let protokoll_data = decoder_protokoll_data
        .as_ref()
        .unwrap();
    let annotation_data = (protokoll_data.data as *const srd_proto_data_annotation)
        .as_ref()
        .unwrap();
    let annotation_payload_text = std::ffi::CStr::from_ptr(
        annotation_data.ann_text.as_ref().unwrap().as_ref().unwrap()
    ).to_str().unwrap();

    let annotation_type = Dmx512Annotator::try_from(annotation_data.ann_class)
        .unwrap();
    let payload = match annotation_type {
        Dmx512Annotator::Bit => {
            let bit = scan_fmt!(annotation_payload_text, "{1d}", u8).unwrap() == 1;
            Some(AnnotationPayload::Bit(bit))
        }
        Dmx512Annotator::Channel => {
            let channel = scan_fmt!(annotation_payload_text, "Channel {3d}", u16).unwrap();
            Some(AnnotationPayload::ChannelId(channel))
        }
        Dmx512Annotator::Data => {
            let (data_dec, _) = scan_fmt!(annotation_payload_text, "{3d} / {4x}}", u8, [hex u8]).unwrap();
            Some(AnnotationPayload::Data(data_dec))
        }
        Dmx512Annotator::ErrorCode =>
            Some(AnnotationPayload::AnnotationError(String::from("error"))),
        _ => None
    };

    let annotations = DecoderAnnotation {
        start_sample: protokoll_data.start_sample,
        end_sample: protokoll_data.end_sample,
        annotation_type,
        payload,
    };
    // println!("{:?}", annotations);
    let target = (&mut *rust_data as *mut _ as *mut RustData).as_ref().unwrap();
    target.sender
        .send(annotations)
        .unwrap();
}

fn start_logic_analyzer(tx: Sender<DecoderAnnotation>) {
    let mut rust_data = Box::new(RustData { sender: tx.clone() });
    let mut callback_data = Box::new(
        CallbackData {
            rustData: &mut *rust_data as *mut _ as *mut c_void,
            onDecoderAnnotation: Some(on_decoder_data),
        }
    );
    unsafe {
        runAnalyzer(&mut *callback_data);
    }
    let hmm: *const GMainLoop ;
}

pub fn get_dmx_data(tx: Sender<DmxPacket>) {
    let (tx_internal, rx_internal) = mpsc::channel();
    loop {
        let tx_clone = tx_internal.clone();
        let thread_join_handle = thread::spawn(move || {
            start_logic_analyzer(tx_clone);
        });
        let mut data = Vec::<u8>::new();
        'receiving: loop {
            let received = rx_internal.try_recv();
            if received.is_ok() {
                let annotation = received.unwrap();
                //println!("{:?}", annotation);
                match annotation.annotation_type {
                    Dmx512Annotator::Data => {
                        if let Some(AnnotationPayload::Data(value)) = annotation.payload {
                            data.push(value)
                        }
                    },
                    _ => { continue }
                }
            }
            if thread_join_handle.is_finished() {
                tx.send(DmxPacket { channels: data }).unwrap();
                thread::sleep(Duration::from_millis(1000));
                break 'receiving;
            }
        }
    }
}
