use std::fmt::Debug;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb888;
use dmx_channels::DmxChannels;
use render_engine::Engine;
use Views::RenderEngine;
use crate::views::dmx_info_screen::{DmxInfoScreen, ParameterDmxInfoScreen};

pub mod dmx_channels;
pub mod render_engine;
pub mod dmx_info_screen;

pub struct ViewController {
    dmx_channels: DmxChannels,
    render_engine: Engine,
    dmx_info_screen: DmxInfoScreen
}

pub enum Views {
    RenderEngine(RenderEngineProps),
    Channel1Timing(ParameterDmxInfoScreen),
}

pub struct RenderEngineProps {
    pub(crate) parameter_render_engine: render_engine::Parameter,
    pub(crate) parameter_dmx_channels: crate::dmx_analyzer::Parameter
}

impl ViewController {
    pub fn new<D>(path: &str, display: &mut D) -> ViewController
        where
            D: DrawTarget<Color = Rgb888>,
            D::Error: Debug,
    {
        let render_engine = Engine::new(path, display);
        let dmx_channels = DmxChannels {};
        let dmx_info_screen = DmxInfoScreen{};
        return ViewController{render_engine, dmx_channels, dmx_info_screen };
    }

    pub fn on_user_update<D>(&mut self, display: &mut D, view: Views)
        where
            D: DrawTarget<Color = Rgb888>,
            D::Error: Debug,
    {
        match view {
            RenderEngine(props) => {
                self.render_engine.on_user_update(display, props.parameter_render_engine);
                self.dmx_channels.on_user_update(display, props.parameter_dmx_channels);
            }
            Views::Channel1Timing(parameter) => {
                self.dmx_info_screen.on_user_update(display, parameter);
            }
        }
    }
}
