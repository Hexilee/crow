use crate::channels::Channel;
use futures::channel::mpsc::{self, UnboundedReceiver};
use proto::curve_service_server::CurveService;
use proto::{Curve, CurveRequest, RawData, RegisterReply};
use tonic::{Request, Response, Status, Streaming};

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
    type GetCurveStream = UnboundedReceiver<Result<Curve, Status>>;
    async fn get_curve(
        &self,
        request: Request<CurveRequest>,
    ) -> Result<Response<<Self as CurveService>::GetCurveStream>, Status> {
        println!("Got a request: {:?}", request);
        let (tx, rx) = mpsc::unbounded();
        self.curve_channels[request.get_ref().index as usize]
            .lock()
            .await
            .push(tx);
        Ok(Response::new(rx))
    }

    async fn register(
        &self,
        _request: Request<Streaming<RawData>>,
    ) -> Result<Response<RegisterReply>, Status> {
        unimplemented!()
    }
}
