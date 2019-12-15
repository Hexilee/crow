use crate::channels::Channel;
use proto::{server::CurveService, Curve, CurveRequest};
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};

pub struct CurveServiceImpl {
    curve_channels: Vec<Channel>,
}

impl CurveServiceImpl {
    pub fn new() -> Self {
        Self {
            curve_channels: Vec::new(),
        }
    }

    pub fn register(&mut self, channel: Channel) {
        self.curve_channels.push(channel)
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
        self.curve_channels[request.get_ref().index as usize]
            .lock()
            .await
            .push(tx);
        Ok(Response::new(rx))
    }
}
