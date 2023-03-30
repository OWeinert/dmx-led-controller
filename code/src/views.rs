use std::fmt::Debug;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb888;
use strum_macros::EnumCount;

use render_engine::Engine;
use render_engine_with_dmx_overlay::render_engine;
use render_engine_with_dmx_overlay::DmxChannels;
use Views::RenderEngine;

use crate::views::dmx_channel_1::{DmxInfoScreen, ParameterDmxInfoScreen};
use crate::views::dmx_reset_sequence::DmxResetSequence;
use crate::views::Views::{Channel1Timing, ResetSequence};

pub mod dmx_channel_1;
pub mod dmx_reset_sequence;
pub mod draw_collection;
pub mod render_engine_with_dmx_overlay;

pub struct ViewController {
    dmx_channels: DmxChannels,
    render_engine: Engine,
    dmx_info_screen: DmxInfoScreen,
    dmx_reset_sequence: DmxResetSequence,
}

#[derive(EnumCount)]
pub enum Views {
    RenderEngine(RenderEngineProps),
    Channel1Timing(ParameterDmxInfoScreen),
    ResetSequence(ParameterDmxInfoScreen),
}

pub struct RenderEngineProps {
    pub parameter_render_engine: render_engine::Parameter,
    pub parameter_dmx_channels: crate::dmx_analyzer::Parameter,
}

impl ViewController {
    pub fn new<D>(path: &str, display: &mut D) -> ViewController
    where
        D: DrawTarget<Color = Rgb888>,
        D::Error: Debug,
    {
        let render_engine = Engine::new(path, display);
        let dmx_channels = DmxChannels {};
        let dmx_info_screen = DmxInfoScreen {};
        let dmx_reset_sequence = DmxResetSequence {};
        return ViewController {
            render_engine,
            dmx_channels,
            dmx_info_screen,
            dmx_reset_sequence,
        };
    }

    pub fn on_user_update<D>(&mut self, display: &mut D, view: Views)
    where
        D: DrawTarget<Color = Rgb888>,
        D::Error: Debug,
    {
        match view {
            ResetSequence(parameter) => {
                self.dmx_reset_sequence.on_user_update(display, parameter);
            }
            Channel1Timing(parameter) => {
                self.dmx_info_screen.on_user_update(display, parameter);
            }
            RenderEngine(props) => {
                self.render_engine
                    .on_user_update(display, props.parameter_render_engine);
                self.dmx_channels
                    .on_user_update(display, props.parameter_dmx_channels);
            }
        }
    }
}
