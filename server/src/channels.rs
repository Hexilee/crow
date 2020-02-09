mod curvature_splines;
mod mock;
use futures::channel::mpsc::UnboundedSender;
use futures::lock::Mutex;
use proto::Curve;
use std::sync::Arc;
use tonic::Status;

pub type Channel = Arc<Mutex<Vec<UnboundedSender<Result<Curve, Status>>>>>;

pub use mock::mock_channel;
