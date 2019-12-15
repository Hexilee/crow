mod curvature_splines;
mod mock;
use futures_util::lock::Mutex;
use proto::Curve;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::Status;

pub type Channel = Arc<Mutex<Vec<mpsc::UnboundedSender<Result<Curve, Status>>>>>;

pub use mock::mock_channel;
