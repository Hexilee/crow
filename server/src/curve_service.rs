use crate::curvature_splines::CurvatureSplines;
use futures_util::lock::Mutex;
use proto::{server::CurveService, Curve, CurveRequest};
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};

pub struct CurveServiceImpl {
    curve_sender: Arc<Mutex<Vec<mpsc::UnboundedSender<Result<Curve, Status>>>>>,
}

impl CurveServiceImpl {
    pub fn new(
        curve_sender: Arc<Mutex<Vec<mpsc::UnboundedSender<Result<Curve, Status>>>>>,
    ) -> Self {
        Self { curve_sender }
    }
}

#[tonic::async_trait]
impl CurveService for CurveServiceImpl {
    type GetCurveStream = mpsc::UnboundedReceiver<Result<Curve, Status>>;
    async fn get_curve(
        &self,
        request: Request<CurveRequest>,
    ) -> Result<Response<Self::GetCurveStream>, Status> {
        println!("Got a request: {:?}", request);
        let (tx, rx) = mpsc::unbounded_channel();
        self.curve_sender.lock().await.push(tx);
        Ok(Response::new(rx))
    }
}
