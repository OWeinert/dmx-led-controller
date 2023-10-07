pub mod dlcl_rpc {
    tonic::include_proto!("dlcl_rpc");
}

use std::error::Error;
use std::sync::Mutex;
use std::io::Write;
use std::pin::Pin;
use embedded_graphics::Pixel;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::Point;
use iter_tools::Itertools;
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};
use dlcl_rpc::dlcl_draw_server::DlclDraw;
use unbox_box::BoxExt as _;
use crate::draw;
use crate::draw::animation::{Animation, Frame};
use crate::draw::layer::LayerManager;
use crate::rpc::dlcl_rpc::{AnimatedLayersResponse, AnimationDto, DlclStatus, EmptyRequest, FrameDto, StatusResponse, LayerPixelDto, PixelDto};

impl Into<Frame> for FrameDto {
    fn into(self) -> Frame {
        let pixels = self.pixels.iter().map(|dto|
            Rgb888::new(dto.r as u8, dto.g  as u8, dto.b  as u8));
        let frame = Frame::new(pixels.collect_vec());
        frame
    }
}

impl Into<Animation> for AnimationDto {
    fn into(self) -> Animation {
        let frames = self.frames.iter()
            .map(|dto| (*dto).clone().into())
            .collect_vec();
        let anim = Animation::from_frames(frames);
        anim
    }
}

///
/// Operations which were send via rpc and
/// will be executed by the LayerManager in order of receival
///
pub enum RpcOp {
    DrawDirect(Vec<Pixel<Rgb888>>),
    DrawOnLayer (usize, Vec<Pixel<Rgb888>>),
    DrawFullLayer (usize, Vec<Rgb888>),
    PushAnimation (usize, Animation)
}

///
/// The rpc service which handles requests
/// and forwards them to the LayerManager
///
pub struct DlclDrawService {
    layer_manager: Mutex<LayerManager>
}

#[tonic::async_trait]
impl DlclDraw for DlclDrawService {

    ///
    /// Rpc request for fetching of animated layer ids
    ///
    async fn get_animated_layers(&self, _request: Request<EmptyRequest>) -> Result<Response<AnimatedLayersResponse>, Status> {
        let layers = self.layer_manager.lock().unwrap()
            .get_anim_layer_ids()
            .iter()
            .map(|i| {
                let i_u32: u32 = *i as u32;
                i_u32
            })
            .collect_vec();
        let response = AnimatedLayersResponse {
            layers
        };
        Ok(Response::new(response))
    }

    ///
    /// Rpc request for pushing animations in the animation queue
    ///
    async fn push_animations(&self, request: Request<Streaming<AnimationDto>>) -> Result<Response<StatusResponse>, Status> {
        let mut stream = request.into_inner();

        // Closure for processing an animation
        let proc = |anim_dto: Result<AnimationDto, Status>| -> Result<(), Box<dyn Error>> {
            let anim_dto = anim_dto?;
            let anim = anim_dto.clone().into();
            let rpc_op = RpcOp::PushAnimation(anim_dto.layer as usize, anim);
            self.layer_manager.lock().unwrap()
                .push_rpc_op(rpc_op);
            Ok(())
        };

        let mut status = DlclStatus::Success;
        let mut message = String::from("Success");
        while let Some(anim_dto) = stream.next().await {
            match proc(anim_dto) {
                Ok(()) => continue,
                Err(err) => {
                    let err_msg = format!("Error while handling PushAnimation request: {}", err);
                    std::io::stderr().write(err_msg.as_bytes()).unwrap();
                    status = DlclStatus::ErrorUndefined; // TODO: Add Specific Error
                    message = err_msg;
                    break;
                }
            }
        }

        let response = StatusResponse {
            status: status.into(),
            message: message
        };
        Ok(Response::new(response))
    }

    async fn draw_on_layer(&self, request: Request<Streaming<LayerPixelDto>>) -> Result<Response<StatusResponse>, Status> {

        let mut status = DlclStatus::Success;
        let mut stream = request.into_inner();
        while let Some(pixel_dto) = stream.next().await {
            let pixel_dto = pixel_dto.clone().unwrap();
            let layer_id = pixel_dto.layer as usize;
            if !self.layer_manager.lock().unwrap().layer_exists(layer_id) {
                status = DlclStatus::ErrorUndefined;
            }
            else {
                let pixels = pixel_dto.pixels;
                pixels.iter().for_each(|p| {
                    draw::draw_pixel_layer(
                        Point::new(p.x as i32, p.y as i32),
                        Rgb888::new(p.r as u8, p.g as u8, p.b as u8),
                        self.layer_manager.lock().unwrap()
                                .layer_by_id(layer_id).unbox_mut());
                })
            }
        }
        let response = StatusResponse {
            status: status.into(),
            message: String::new()
        };
        Ok(Response::new(response))
    }

    async fn draw_full_layer(&self, request: Request<FrameDto>) -> Result<Response<StatusResponse>, Status> {
        todo!()
    }

    async fn draw_direct(&self, request: Request<Streaming<PixelDto>>) -> Result<Response<StatusResponse>, Status> {
        todo!()
    }

    type GetAnimationQueueStream = Pin<Box<dyn Stream<Item = std::result::Result<AnimationDto, tonic::Status>> + Send + 'static>>;

    async fn get_animation_queue(&self, request: Request<EmptyRequest>) -> Result<Response<Self::GetAnimationQueueStream>, Status> {
        todo!()
    }
}

impl DlclDrawService {

    ///
    /// Creates a new DlclDrawService
    ///
    /// ## Arguments
    ///
    /// * 'layer_manager' - The LayerManager as a Mutex
    ///
    /// ## Returns
    ///
    /// 'Self' - The DlclDrawService
    ///
    pub fn new(layer_manager: Mutex<LayerManager>) -> DlclDrawService {
        let service = DlclDrawService {
            layer_manager
        };
        service
    }

    pub fn layer_manager(&self) -> &Mutex<LayerManager> {
        &self.layer_manager
    }
}