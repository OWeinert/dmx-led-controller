pub mod dlcl_rpc {
    tonic::include_proto!("dlcl_rpc");
}

use embedded_graphics::Pixel;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::Point;
use iter_tools::Itertools;
use tokio_stream::{Stream, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};
use dlcl_rpc::dlcl_draw_server::{DlclDraw, DlclDrawServer};
use crate::draw;
use crate::draw::animation::{Animation, Frame};
use crate::draw::layer;
use crate::draw::layer::{AnimatedLayer, LayerManager, LayerType};
use crate::rpc::dlcl_rpc::{AnimatedLayersResponse, AnimationDto, DlclStatus, EmptyRequest, FrameDto, PixelDto, StatusResponse};

impl Into<Frame> for FrameDto {
    fn into(self) -> Frame {
        let pixels = self.pixels.iter().map(|dto|
            Rgb888::new(dto.r as u8, dto.g  as u8, dto.b  as u8));
        let frame = Frame::new(pixels.collect_vec(), self.frame_time as usize);
        frame
    }
}


pub enum RpcOp {
    DrawDirect { pixels: Vec<Pixel<Rgb888>> },
    DrawOnLayer { layer_id: usize, pixels: Vec<Pixel<Rgb888>> },
    DrawFullLayer { layer_id: usize, frame: Vec<Rgb888> },
    PushAnimation { layer_id: usize, animation: Animation }
}

#[derive(Debug)]
pub struct DlclDrawService
{
    layer_manager: &LayerManager
}

#[tonic::async_trait]
impl DlclDraw for DlclDrawService {
    async fn get_animated_layers(&self, request: Request<EmptyRequest>) -> Result<Response<AnimatedLayersResponse>, Status> {
        let layers = layer::layers_of_type::<AnimatedLayer>();
        todo!()
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

impl DlclDrawService {
    // todo
}