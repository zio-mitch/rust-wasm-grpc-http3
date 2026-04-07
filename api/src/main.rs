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
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter::default();
    let service = GreeterServer::new(greeter);

    println!("Server gRPC pronto su {}", addr);

    Server::builder()
        .accept_http1(true)
        // We use Layer: since it is more compatible with generated types and tonic-web's expectations for gRPC-Web,
        .layer(GrpcWebLayer::new()) 
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}