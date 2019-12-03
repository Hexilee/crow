use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use tokio::timer::delay_for;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use futures_util::sink::SinkExt;

pub mod crow {
    tonic::include_proto!("crow");
}

use crow::{
    server::{CurveService, CurveServiceServer},
    Curve, CurveRequest, Point,
};

#[derive(Default)]
pub struct MockCurveService {}

#[tonic::async_trait]
impl CurveService for MockCurveService {
    type GetCurveStream = mpsc::UnboundedReceiver<Result<Curve, Status>>;
    async fn get_curve(
        &self,
        request: Request<CurveRequest>,
    ) -> Result<Response<Self::GetCurveStream>, Status> {
        println!("Got a request: {:?}", request);
        let (mut tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            loop {
                delay_for(Duration::from_millis(500)).await;
                let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
                let mut points = Vec::new();
                for i in 1..20 {
                    let f = i as f32;
                    points.push(Point {x: f, y: f, z: f});
                }
                tx.send(Ok(Curve {timestamp, points})).await.unwrap();
            }
        });
        Ok(Response::new(rx))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8081".parse().unwrap();
    let curve_service = MockCurveService::default();

    Server::builder()
        .add_service(CurveServiceServer::new(curve_service))
        .serve(addr)
        .await?;
    Ok(())
}