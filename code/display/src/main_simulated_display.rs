use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use cascade::cascade;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window};

use dlcl::draw::framebuffer;
use dlcl::draw::layer::LayerManager;
use dlcl::rpc::dlcl_rpc::dlcl_draw_server::DlclDrawServer;
use dlcl::rpc::DlclDrawService;
use dlcl::tonic::transport::server::Server;
use hyper::server::conn::Http;
use tokio::net::TcpListener;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let disp_width = 64;
    let disp_height = 64;

    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(2)
        .scale(10)
        .build();
    let mut display = SimulatorDisplay::new(Size::new(disp_width, disp_height));
    let mut window = cascade! {
        Window::new("Dmx Led Controller", &output_settings);
        ..update(&display);
    };

    framebuffer::set_framebuf_size(disp_width as usize, disp_height as usize);

    let layer_mgr = LayerManager::new(true);
    let dlcl_service = DlclDrawService::new(Mutex::new(layer_mgr));

    let address: SocketAddr = "127.0.0.1:50051".parse()?;
    let server_service = Server::builder()
        .add_service(DlclDrawServer::new(dlcl_service))
        .into_service();

    let mut http = Http::new();
    http.http2_only(true);

    let listener = TcpListener::bind(address).await?;

    'running: loop {
// handle window events

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
        // get tpc connections
        let (conn, addr) = match listener.accept().await {
            Ok(incoming) => incoming,
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
                continue;
            }
        };

        // serve gRPC service http connection to connected tcp clients
        let http = http.clone();
        let service = server_service.clone();

        tokio::spawn(async move {
            let svc = tower::ServiceBuilder::new()
                .service(service);

            http.serve_connection(conn, svc).await.unwrap();
        }).await.unwrap();

        window.update(&display);
        display.clear(Rgb888::new(0, 0, 0)).unwrap();
        framebuffer::clear_framebuf();
        time::sleep(time::Duration::from_micros(16666)).await;
    }

    Ok(())
}
