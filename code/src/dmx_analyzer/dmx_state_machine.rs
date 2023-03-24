use super::logic_analyzer::{DecoderAnnotation, Dmx512AnnotatorPayload};

pub struct ChannelData {
    pub(crate) channel: [u8; 512],
}

pub struct ChannelId {
    pub(crate) channel_id: u16,
}

#[derive(Copy, Clone, Debug)]
pub struct Bit {
    pub start_sample: u64,
    pub end_sample: u64,
    pub bit: bool,
}

pub enum DmxStateMachineState {
    Idle(),
    Break(),
    MarkAfterBreak(),
    StartFirstChannel(),
    StartBit(),
    Bit1([Bit; 8]),
    Bit2([Bit; 8]),
    Bit3([Bit; 8]),
    Bit4([Bit; 8]),
    Bit5([Bit; 8]),
    Bit6([Bit; 8]),
    Bit7([Bit; 8]),
    StopBit0([Bit; 8]),
    StopBit1([Bit; 8]),
   // InterFrame(),
   // InterPacket(),
    ChannelNr([Bit; 8], ChannelData, ChannelId),
    Data([Bit; 8], ChannelData),
    End([Bit; 8], ChannelData),
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
            (S::Idle(), A::Break) => S::Break(),
            (S::Break(), A::MarkAfterBreak) => S::MarkAfterBreak(),
            (S::MarkAfterBreak(),
                A::Startbit | A::Bit(..) | A::Stopbit | A::Startcode | A::Data(..)) => S::MarkAfterBreak(),
            (S::MarkAfterBreak(), A::InterFrame) => S::StartFirstChannel(),
            (S::StartFirstChannel(), A::Startbit) => S::StartBit(),
            (S::StartBit(), A::Bit(bit)) => {
                let mut bits = [Bit { start_sample: 0, end_sample: 0, bit: *bit }; 8];
                bits[0] = Bit {start_sample: input.start_sample, end_sample: input.end_sample, bit: *bit };
                S::Bit1(bits)
            },
            (S::Bit1(mut bits), A::Bit(bit)) => {
                bits[2] = Bit {start_sample: input.start_sample, end_sample: input.end_sample, bit: *bit };
                S::Bit2(bits)
            },
            (S::Bit2(mut bits), A::Bit(bit)) => {
                bits[3] = Bit {start_sample: input.start_sample, end_sample: input.end_sample, bit: *bit };
                S::Bit3(bits)
            },
            (S::Bit3(mut bits), A::Bit(bit)) => {
                bits[4] = Bit {start_sample: input.start_sample, end_sample: input.end_sample, bit: *bit };
                S::Bit4(bits)
            },
            (S::Bit4(mut bits), A::Bit(bit)) => {
                bits[5] = Bit {start_sample: input.start_sample, end_sample: input.end_sample, bit: *bit };
                S::Bit5(bits)
            },
            (S::Bit5(mut bits), A::Bit(bit)) => {
                bits[6] = Bit {start_sample: input.start_sample, end_sample: input.end_sample, bit: *bit };
                S::Bit6(bits)
            },
            (S::Bit6(mut bits), A::Bit(bit)) => {
                bits[7] = Bit {start_sample: input.start_sample, end_sample: input.end_sample, bit: *bit };
                S::Bit7(bits)
            },
            (S::Bit7(bits),  A::Stopbit) => S::StopBit0(bits),
            (S::StopBit0(bits), A::Stopbit) => S::StopBit1(bits),
            (S::StopBit1(bits), A::ChannelNr(..)) => S::ChannelNr(
                bits,
                ChannelData { channel: [0; 512] },
                ChannelId { channel_id: 1 },
            ),
            (S::ChannelNr(bits, mut channel, channel_id), A::Data(data)) => {
                channel.channel[channel_id.channel_id as usize] = *data;
                S::Data(bits, channel)
            },
            (S::Data(bits, channel), A::ChannelNr(channel_hm)) => S::ChannelNr(
                bits,
                channel,
                ChannelId {
                    channel_id: *channel_hm,
                },
            ),
            (S::Data(bits, channel), A::ErrorCode(_)) => S::End(bits, channel),
            (end @ S::End(..), ..) => end,
            (_, A::ErrorCode(_)) => S::Error(),
            (state, _) => state,
        };
    }
}
