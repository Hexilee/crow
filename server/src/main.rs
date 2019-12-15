mod curvature_splines;
mod curve_service;
use curve_service::CurveServiceImpl;
use futures_util::lock::Mutex;
use proto::server::CurveServiceServer;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8081".parse().unwrap();
    Server::builder()
        .add_service(CurveServiceServer::new(CurveServiceImpl::new(Arc::new(
            Mutex::new(Vec::new()),
        ))))
        .serve(addr)
        .await?;
    Ok(())
}
