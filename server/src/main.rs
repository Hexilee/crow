mod channels;
mod curve_service;
use curve_service::CurveServiceImpl;
use proto::server::CurveServiceServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8081".parse().unwrap();
    let mut service = CurveServiceImpl::new();
    service.register(channels::mock_channel());
    Server::builder()
        .add_service(CurveServiceServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
