use std::any::Any;
use std::collections::VecDeque;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use iter_tools::Itertools;
use draw::draw_layer;
use crate::draw;
use crate::draw::animation::Animation;
use crate::draw::framebuffer::GLOBAL_FRAMEBUF;
use crate::rpc::RpcOp;
use super::framebuffer::FrameBuffer;

///
/// Represents a Layer which can be drawn to.
///
pub trait Layer: Send + Sync {

    ///
    /// Prepares the layer for drawing (e.g. modify pixels before drawing, etc.)
    /// Function highly depends on the Layer implmentation.
    ///
    /// ## Arguments
    ///
    /// 'self' - The Layer
    ///
    fn prep(&mut self);

    ///
    /// Returns true if the Layer sets the color black to be transparent instead of true black
    ///
    /// ## Returns
    ///
    /// 'bool' -
    ///
    fn transparent_black(&self) -> bool;

    fn framebuf(&mut self) -> &mut FrameBuffer;

    fn layer_type(&self) -> LayerType;

    fn get_index(&self) -> usize;

    fn set_index(&mut self, index: usize);

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(PartialEq)]
pub enum LayerType {
    DirectDraw,
    Animated
}



///
/// Represents a Layer which can be drawn to directly
///
pub struct DirectDrawLayer {
    framebuf: FrameBuffer,
    transparent_black: bool,
    index: usize,
}

impl DirectDrawLayer {

    ///
    /// Creates a new DirectDrawLayer
    ///
    /// ## Arguments
    ///
    /// *'transparent_black' - If the Layer should render the color black as transparent
    /// *'framebuf_w' - The framebuffer width
    /// *'framebuf_h' - The framebuffer height
    ///
    /// ## Returns
    ///
    /// 'Self' - The DirectDrawLayer
    ///
    pub fn new(transparent_black: bool,
               framebuf_w: usize, framebuf_h: usize) -> Self {
        let layer = DirectDrawLayer {
            framebuf: FrameBuffer::new(framebuf_w, framebuf_h, Rgb888::BLACK),
            transparent_black,
            index: 0
        };
        layer
    }
}

impl Layer for DirectDrawLayer {
    fn prep(&mut self) {}

    fn transparent_black(&self) -> bool {
        self.transparent_black
    }
    
    fn framebuf(&mut self) -> &mut FrameBuffer {
        &mut self.framebuf
    }

    fn layer_type(&self) -> LayerType {
        LayerType::DirectDraw
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}



///
/// Represens a Layer which plays given animations from an animation queue
///
pub struct AnimatedLayer {
    framebuf: FrameBuffer,
    transparent_black: bool,
    loop_animations: bool,
    animation_queue: VecDeque<Animation>,
    current_animation: Option<Animation>,
    index: usize
}

impl AnimatedLayer {

    ///
    /// Creates a new AnimatedLayer
    ///
    /// ## Arguments
    ///
    /// *'transparent_black' - If the Layer should render the color black as transparent
    /// *'loop_animations' - If the animation queue should loop
    /// *'framebuf_w' - The framebuffer width
    /// *'framebuf_h' - The framebuffer height
    ///
    /// ## Returns
    ///
    /// 'Self' - The AnimatedLayer
    ///
    pub fn new(transparent_black: bool, loop_animations: bool,
           framebuf_h: usize, framebuf_w: usize) -> Self {
        let layer = AnimatedLayer {
            framebuf: FrameBuffer::new(framebuf_w, framebuf_h, Rgb888::BLACK),
            transparent_black,
            loop_animations,
            animation_queue: VecDeque::new(),
            current_animation: Option::None,
            index: 0
        };
        layer
    }

    ///
    /// Consumes and draws the current frame of the current animation
    ///
    fn draw_frame(&mut self) {
        // load next animation when no animation is loaded or the current loaded is one finished
        if self.current_animation.is_none()
            || (self.current_animation.is_some()
            && self.current_animation.clone().unwrap().frame_index() >= self.current_animation.as_mut().unwrap().len()) {
            self.next_animation();
        }
        let animation = self.current_animation.as_mut().unwrap();

        // get next frame and draw it's pixels to the framebuffer
        let frame = animation.next_frame();
        self.framebuf.set_data(&frame.pixels());
    }

    ///
    /// Queues an animation
    ///
    pub fn enqueue(&mut self, animation: Animation) {
        self.animation_queue.push_back(animation.clone());
    }

    ///
    /// Dequeues an animation
    ///
    fn dequeue(&mut self) -> Animation {
        self.animation_queue.pop_back().unwrap()
    }

    ///
    /// Skips to the next animation
    ///
    fn next_animation(&mut self) {
        self.current_animation = Some(self.dequeue());
        if self.loop_animations {
            self.enqueue(self.current_animation.clone().unwrap());
        }
    }
}

impl Layer for AnimatedLayer {
    fn prep(&mut self) {
        self.draw_frame();
    }

    fn transparent_black(&self) -> bool {
        self.transparent_black
    }

    fn framebuf(&mut self) -> &mut FrameBuffer {
        &mut self.framebuf
    }

    fn layer_type(&self) -> LayerType {
        LayerType::Animated
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}



///
/// A manager for handling the layers and interactions with them
///
pub struct LayerManager {
    layers: Vec<Box<dyn Layer>>,
    rpc_queue: Option<VecDeque<RpcOp>>,
    rpc_enabled: bool
}

impl LayerManager {

    ///
    /// Creates a new LayerManager
    ///
    /// ## Arguments
    ///
    /// * 'rpc_enabled' - enable gRpc interaction
    ///
    /// ## Returns
    ///
    /// 'Self' - The LayerManager
    ///
    pub fn new(rpc_enabled: bool) -> Self {
        let rpc_queue: Option<VecDeque<RpcOp>> = if rpc_enabled {
            Some(VecDeque::new())
        }
        else {
            None
        };
        LayerManager {
            layers: vec![],
            rpc_queue,
            rpc_enabled
        }
    }

    ///
    /// If gRpc interaction is enabled
    ///
    /// ## Returns
    ///
    /// * 'bool' - True if gRpc is enabled
    ///
    pub fn rpc_enabled(&self) -> bool {
        self.rpc_enabled
    }

    ///
    /// Registers a layer in the LayerManager
    ///
    /// ## Arguments
    ///
    /// * 'layer' - The layer to be registered
    ///
    /// ## Returns
    ///
    /// 'Result<(), Err>' - Result containing Errors if the registration failed
    ///
    pub fn register_layer(&mut self, mut layer: impl Layer + 'static) -> Result<(), &str> {
        let global_fbuf = GLOBAL_FRAMEBUF.lock().unwrap();
        if layer.framebuf().width() != global_fbuf.width()
            || layer.framebuf().height() != global_fbuf.height() {

            return Err("layer framebuffer size mismatch!");
        }
        layer.set_index(self.layers.len());
        self.layers.push(Box::new(layer));
        self.sort_layers();
        Ok(())
    }

    ///
    /// Updates the LayerManager.
    /// Has to be called once per Frame/Draw Call
    ///
    pub fn update(&mut self) {
        if self.rpc_enabled {
            match self.process_rpc_ops() {
                Ok(_) => {},
                Err(msg) => println!("{}", msg)
            }
        }
        self.push_layers();
    }

    ///
    /// Returns a reference to the layer with the given id
    ///
    /// ## Arguments
    ///
    /// * 'id' - The layer id
    ///
    /// ## Returns
    ///
    /// '&dyn Layer' - Layer reference
    ///
    pub fn layer_by_id(&mut self, id: usize) -> &mut Box<dyn Layer> {
        &mut self.layers[id]
    }

    ///
    /// Checks if a layer with the given *id* exists
    ///
    /// ## Arguments
    ///
    /// * 'id' - The layer's id
    ///
    /// ## Returns
    ///
    /// * 'bool' - True if the layer exists
    ///
    pub fn layer_exists(&self, id: usize) -> bool {
        // if the id is less than the layers vector's length than the layer must exist
        self.layers.len() < id
    }

    ///
    /// Returns the layer ids of all animated layers
    ///
    /// ## Returns
    ///
    /// 'Vec\<usize\>' - The layer ids
    ///
    pub(crate) fn get_anim_layer_ids(&self) -> Vec<usize> {
        let ids = self.layers.iter()
            .filter(|l| l.layer_type() == LayerType::Animated)
            .map(|l| l.get_index())
            .collect_vec();
        ids
    }

    ///
    /// Processes the RpcOps in the queue
    ///
    fn process_rpc_ops(&mut self) -> Result<(), &str>{
        if !self.rpc_enabled || self.rpc_queue.is_none() {
            return Err("rpc is disabled or the rpc_queue is not initialized!");
        }

        let rpc_queue = self.rpc_queue.as_mut().unwrap();
        while rpc_queue.len() > 0 {
            let rpc_op = rpc_queue.pop_front().unwrap();
            match rpc_op {
                RpcOp::DrawDirect(pixels) => {
                    pixels.iter().for_each(|p| {
                        draw::draw_pixel_direct(p.0, p.1);
                    });
                },
                RpcOp::DrawOnLayer(layer_id, pixels) => {
                    let layer = self.layers[layer_id].as_mut();
                    pixels.iter().for_each(|p| {
                        draw::draw_pixel_layer(p.0, p.1, layer);
                    });
                },
                RpcOp::DrawFullLayer(layer_id, frame) => {
                    let layer = self.layers[layer_id].as_mut();
                    draw::draw_frame_layer(&frame, layer);
                },
                RpcOp::PushAnimation(layer_id, anim) => {
                    let layer: &mut AnimatedLayer = self.layers[layer_id].as_mut()
                        .as_any_mut()
                        .downcast_mut::<AnimatedLayer>()
                        .unwrap();
                    layer.enqueue(anim);
                }
            }
        }
        return Ok(());
    }

    ///
    /// Push an RpcOp to the queue
    ///
    /// ## Arguments
    ///
    /// * 'rpc_op' - The rpc operation
    ///
    pub(crate) fn push_rpc_op(&mut self, rpc_op: RpcOp) {
        self.rpc_queue.as_mut()
            .unwrap()
            .push_back(rpc_op);
    }

    ///
    /// Pushes the layers of the LayerManager to the global framebuffer
    ///
    fn push_layers(&mut self) {
        for layer in &mut self.layers {
            layer.prep();
            draw_layer(layer.as_mut());
        }
    }

    ///
    /// Sorts layers by id
    ///
    fn sort_layers(&mut self) {
        self.layers.sort_by(|a, b| a.get_index().partial_cmp(&b.get_index()).unwrap());
    }
}