pub mod dlcl_rpc {
    tonic::include_proto!("dlcl_rpc");
}

use std::error::Error;
use std::sync::Mutex;
use embedded_graphics::Pixel;
use embedded_graphics::pixelcolor::Rgb888;
use iter_tools::Itertools;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status, Streaming};
use dlcl_rpc::dlcl_draw_server::DlclDraw;
use crate::draw::animation::{Animation, Frame};
use crate::draw::layer::LayerManager;
use crate::rpc::dlcl_rpc::{AnimatedLayersResponse, AnimationDto, DlclStatus, EmptyRequest, FrameDto, StatusResponse};

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


pub enum RpcOp {
    DrawDirect(Vec<Pixel<Rgb888>>),
    DrawOnLayer (usize, Vec<Pixel<Rgb888>>),
    DrawFullLayer (usize, Vec<Rgb888>),
    PushAnimation (usize, Animation)
}

pub struct DlclDrawService {
    layer_manager: Mutex<LayerManager>
}

#[tonic::async_trait]
impl DlclDraw for DlclDrawService {
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

    async fn push_animations(&self, request: Request<Streaming<AnimationDto>>) -> Result<Response<StatusResponse>, Status> {
        let mut stream = request.into_inner();
        let proc = |anim_dto: Result<AnimationDto, Status>| -> Result<(), Box<dyn Error>> {
            let anim_dto = anim_dto?;
            let anim = anim_dto.clone().into();
            let rpc_op = RpcOp::PushAnimation(anim_dto.layer as usize, anim);
            self.layer_manager.lock().unwrap()
                .push_rpc_op(rpc_op);
            Ok(())
        };
        while let Some(anim_dto) = stream.next().await {
            match proc(anim_dto) {
                Ok(()) => {},
                Err(err) => println!("Error while handling PushAnimation request: {}", err)
            }
        }

        let response = StatusResponse {
            status: DlclStatus::Success.into(),
            message: String::from("")
        };
        Ok(Response::new(response))
    }
}

impl DlclDrawService {

    pub fn new<F>(layer_manager: Mutex<LayerManager>) -> DlclDrawService {
        let service = DlclDrawService {
            layer_manager
        };
        service
    }

}