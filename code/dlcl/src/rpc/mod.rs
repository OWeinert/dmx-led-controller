pub mod dlcl_rpc {
    tonic::include_proto!("dlcl_rpc");
}

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


pub enum RpcOp {
    DrawDirect(Vec<Pixel<Rgb888>>),
    DrawOnLayer (usize, Vec<Pixel<Rgb888>>),
    DrawFullLayer (usize, Vec<Rgb888>),
    PushAnimation (usize, Animation)
}

pub struct DlclDrawService<'a> {
    layer_manager: &'a LayerManager
}

#[tonic::async_trait]
impl DlclDraw for DlclDrawService<'static> {
    async fn get_animated_layers(&self, _request: Request<EmptyRequest>) -> Result<Response<AnimatedLayersResponse>, Status> {
        let layers = self.layer_manager.get_anim_layer_ids().iter()
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

        let mut status = DlclStatus::Success;
        let mut message = "";

        while let Some(animation) = stream.next().await {
            let animation = animation?;
            todo!();
        }

        let response = StatusResponse {
            status: status.into(),
            message: String::from(message)
        };
        Ok(Response::new(response))
    }
}

impl DlclDrawService<'_> {
    // todo
}