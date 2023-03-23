use crate::logic_analyzer::{DecoderAnnotation, Dmx512AnnotatorCode, Dmx512AnnotatorPayload};

pub struct ChannelData {
    pub(crate) channel: [u8; 512],
}

pub struct ChannelId {
    pub(crate) channel_id: u16,
}

pub struct MarkAfterBreak {
    pub(crate) packet: DecoderAnnotation,
}

pub enum DmxStateMachineState {
    Idle(),
    Break(ChannelData),
    MarkAfterBreak(ChannelData),
    Channel(ChannelData, MarkAfterBreak, ChannelId),
    Data(ChannelData, MarkAfterBreak),
    End(ChannelData, MarkAfterBreak),
    Error(),
}

pub trait Transition: Sized {
    fn transition(self, input: DecoderAnnotation) -> DmxStateMachineState;
}

impl Transition for DmxStateMachineState {
    fn transition(self, input: DecoderAnnotation) -> DmxStateMachineState {
        use Dmx512AnnotatorPayload as A;
        use DmxStateMachineState as S;

        return match (self, &input.payload) {
            (S::Idle(), A::Break) => S::Break(ChannelData { channel: [0; 512] }),
            (S::Break(test), A::MarkAfterBreak) => S::MarkAfterBreak(test),
            (S::MarkAfterBreak(test1), A::Channel(_)) => S::Channel(
                test1,
                MarkAfterBreak { packet: input },
                ChannelId { channel_id: 1 },
            ),
            (S::Channel(mut channel, mab, channel_id), A::Data(data)) => {
                channel.channel[channel_id.channel_id as usize] = *data;
                S::Data(channel, mab)
            }
            (S::Data(channel, mab), A::Channel(channel_hm)) => S::Channel(
                channel,
                mab,
                ChannelId {
                    channel_id: *channel_hm,
                },
            ),
            (S::Data(channel, mab), A::ErrorCode(_)) => S::End(channel, mab),
            (end @ S::End(..), ..) => end,
            (_, A::ErrorCode(_)) => S::Error(),
            (state, _) => state,
        };
    }
    // TODO, can I use this: https://github.com/eugene-babichenko/rust-fsm/blob/master/rust-fsm/tests/circuit_breaker.rs ?
    // maybe add output module

}
