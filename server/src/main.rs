mod curvature_splines;
mod curve_service;
use curve_service::CurveServiceImpl;
use proto::server::CurveServiceServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8081".parse().unwrap();
    Server::builder()
        .add_service(CurveServiceServer::new(CurveServiceImpl::new(2)))
        .serve(addr)
        .await?;
    Ok(())
}
