use crate::curvature_splines::CurvatureSplines;
use futures_util::lock::Mutex;
use proto::{server::CurveService, Curve, CurveRequest};
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};

pub struct CurveServiceImpl {
    curve_senders: Arc<Vec<Mutex<Vec<mpsc::UnboundedSender<Result<Curve, Status>>>>>>,
}

impl CurveServiceImpl {
    pub fn new(curves_nums: usize) -> Self {
        let mut curve_senders = Vec::with_capacity(curves_nums);
        for _ in 0..curves_nums {
            curve_senders.push(Mutex::new(Vec::new()))
        }
        Self {
            curve_senders: Arc::new(curve_senders),
        }
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
        self.curve_senders[request.get_ref().index as usize]
            .lock()
            .await
            .push(tx);
        Ok(Response::new(rx))
    }
}
