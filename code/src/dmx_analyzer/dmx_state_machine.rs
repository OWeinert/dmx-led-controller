use std::num::Wrapping;
use super::logic_analyzer::{DecoderAnnotation, Dmx512AnnotatorPayload};

pub struct ChannelData {
    channel: [u8; 512],
}

#[derive(Copy, Clone, Debug)]
pub struct ChannelId {
    pub(crate) channel_id: u16,
}

#[derive(Copy, Clone, Debug)]
pub struct Bit {
    pub start_sample: u64,
    pub end_sample: u64,
    pub bit: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct ResetSequence {
    pub mark_after_break: u64,
    pub space_for_break: u64,
}

#[derive(Copy, Clone, Debug)]
pub struct DmxOutput {
    pub reset_sequence: ResetSequence,
    pub bits: [Bit; 8],
    pub channels: [u8; 512],
}

pub enum DmxStateMachineState {
    Idle(),
    Break(ResetSequence),
    MarkAfterBreak(ResetSequence),
    StartFirstChannel(ResetSequence),
    StartBit(ResetSequence),
    Bit1(ResetSequence, [Bit; 8]),
    Bit2(ResetSequence, [Bit; 8]),
    Bit3(ResetSequence, [Bit; 8]),
    Bit4(ResetSequence, [Bit; 8]),
    Bit5(ResetSequence, [Bit; 8]),
    Bit6(ResetSequence, [Bit; 8]),
    Bit7(ResetSequence, [Bit; 8]),
    Bit8(ResetSequence, [Bit; 8]),
    StopBit0(ResetSequence, [Bit; 8]),
    StopBit1(ResetSequence, [Bit; 8]),
    // InterFrame(),
    // InterPacket(),
    ChannelNr(ResetSequence, [Bit; 8], ChannelData, ChannelId),
    Data(ResetSequence, [Bit; 8], ChannelData),
    End(DmxOutput),
    Error(),
}

pub trait Transition: Sized {
    fn transition(self, input: DecoderAnnotation) -> DmxStateMachineState;
}

impl Transition for DmxStateMachineState {
    /*
     * The state machine is quite linear, so the current approach is enough.
     * If the statemachine gets more complex, this library can make it simpler
     * https://github.com/eugene-babichenko/rust-fsm/blob/master/rust-fsm/tests/circuit_breaker.rs
     * The library also supports handy output modules.
     */
    fn transition(self, input: DecoderAnnotation) -> DmxStateMachineState {
        use Dmx512AnnotatorPayload as A;
        use DmxStateMachineState as S;
        return match (self, &input.payload) {
            (S::Idle(), A::Break) => {
                let samples = Wrapping(input.end_sample) - Wrapping(input.start_sample);
                S::Break(ResetSequence {
                    space_for_break: samples.0,
                    mark_after_break: 0,
                })
            },
            (S::Break(reset), A::MarkAfterBreak) => {
                let samples = input.end_sample - input.start_sample;
                S::MarkAfterBreak(ResetSequence {
                    space_for_break: reset.space_for_break,
                    mark_after_break: samples,
                })
            },
            (
                S::MarkAfterBreak(reset),
                A::Startbit | A::Bit(..) | A::Stopbit | A::Startcode | A::Data(..),
            ) => S::MarkAfterBreak(reset),
            (S::MarkAfterBreak(reset), A::InterFrame) => S::StartFirstChannel(reset),
            (S::StartFirstChannel(reset), A::Startbit) => S::StartBit(reset),
            (S::StartBit(reset), A::Bit(bit)) => {
                let mut bits = [Bit {
                    start_sample: 0,
                    end_sample: 0,
                    bit: *bit,
                }; 8];
                bits[0] = Bit {
                    start_sample: input.start_sample,
                    end_sample: input.end_sample,
                    bit: *bit,
                };
                S::Bit1(reset, bits)
            }
            (S::Bit1(reset, mut bits), A::Bit(bit)) => {
                bits[1] = Bit {
                    start_sample: input.start_sample,
                    end_sample: input.end_sample,
                    bit: *bit,
                };
                S::Bit2(reset, bits)
            }
            (S::Bit2(reset, mut bits), A::Bit(bit)) => {
                bits[2] = Bit {
                    start_sample: input.start_sample,
                    end_sample: input.end_sample,
                    bit: *bit,
                };
                S::Bit3(reset, bits)
            }
            (S::Bit3(reset, mut bits), A::Bit(bit)) => {
                bits[3] = Bit {
                    start_sample: input.start_sample,
                    end_sample: input.end_sample,
                    bit: *bit,
                };
                S::Bit4(reset, bits)
            }
            (S::Bit4(reset, mut bits), A::Bit(bit)) => {
                bits[4] = Bit {
                    start_sample: input.start_sample,
                    end_sample: input.end_sample,
                    bit: *bit,
                };
                S::Bit5(reset, bits)
            }
            (S::Bit5(reset, mut bits), A::Bit(bit)) => {
                bits[5] = Bit {
                    start_sample: input.start_sample,
                    end_sample: input.end_sample,
                    bit: *bit,
                };
                S::Bit6(reset, bits)
            }
            (S::Bit6(reset, mut bits), A::Bit(bit)) => {
                bits[6] = Bit {
                    start_sample: input.start_sample,
                    end_sample: input.end_sample,
                    bit: *bit,
                };
                S::Bit7(reset, bits)
            }
            (S::Bit7(reset, mut bits), A::Bit(bit)) => {
                bits[7] = Bit {
                    start_sample: input.start_sample,
                    end_sample: input.end_sample,
                    bit: *bit,
                };
                S::Bit8(reset, bits)
            }
            (S::Bit8(reset, bits), A::Stopbit) => S::StopBit0(reset, bits),
            (S::StopBit0(reset, bits), A::Stopbit) => S::StopBit1(reset, bits),
            (S::StopBit1(reset, bits), A::ChannelNr(..)) => S::ChannelNr(
                reset,
                bits,
                ChannelData { channel: [0; 512] },
                ChannelId { channel_id: 1 },
            ),
            (S::ChannelNr(reset, bits, mut channel, channel_id), A::Data(data)) => {
                channel.channel[channel_id.channel_id as usize - 1] = *data;
                S::Data(reset, bits, channel)
            }
            (S::Data(reset, bits, channel), A::ChannelNr(channel_hm)) => S::ChannelNr(
                reset,
                bits,
                channel,
                ChannelId {
                    channel_id: *channel_hm,
                },
            ),
            (S::Data(reset, bits, channel), A::ErrorCode(_)) => S::End(DmxOutput {
                reset_sequence: reset,
                bits,
                channels: channel.channel,
            }),
            (end @ S::End(..), ..) => end,
            (_, A::ErrorCode(_)) => S::Error(),
            (state, _) => state,
        };
    }
}
