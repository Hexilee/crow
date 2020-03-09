use super::Channel;
use super::curvature_splines::{CurvatureSplines, PointVector};
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::lock::Mutex;
use futures::sink::SinkExt;
use futures_timer::Delay;
use proto::{Curve, Point};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tonic::Status;

pub fn mock_channel() -> Channel {
    let channel = Arc::new(Mutex::new(
        Vec::<UnboundedSender<Result<Curve, Status>>>::new(),
    ));
    let channel_cpy = channel.clone();
    tokio::spawn(async move {
        loop {
            let points = vec![
                (4.66, 0.21, 0.),
                (9.36, 0.27, 0.),
                (14.82, 0.086, 0.),
                (19.72, -0.0093, 0.),
                (24.74, -0.091, 0.),
                (29.95, -0.079, 0.),
            ]
                .interpolate(0.2)
                .to_curve().unwrap();
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            for sender in channel.lock().await.iter_mut() {
                sender
                    .send(Ok(Curve {
                        timestamp,
                        points: points.clone(),
                    }))
                    .await
                    .unwrap()
            }
        }
    });
    channel_cpy
}
