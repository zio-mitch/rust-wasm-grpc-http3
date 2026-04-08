use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;

pub mod ping {
    tonic::include_proto!("ping");
}
use ping::greeter_server::{Greeter, GreeterServer};
use ping::{PongReply, PingRequest};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_ping(&self, request: Request<PingRequest>) -> Result<Response<PongReply>, Status> {
        let reply = PongReply {
            message: format!("Ping {}! (from Rust + gRPC)", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env!("API_PORT");
    let addr_str = format!("0.0.0.0:{}", port);
    let addr = addr_str.parse()?;

    let greeter = MyGreeter::default();
    let service = GreeterServer::new(greeter);

    println!("Server gRPC pronto su {}", addr);

    Server::builder()
        // >[!HINT] FIXME: when webTransport is ready to work in wasm we can pass to http3 usage
        .accept_http1(true)
        // We use Layer: since it is more compatible with generated types and tonic-web's expectations for gRPC-Web,
        .layer(GrpcWebLayer::new()) 
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}