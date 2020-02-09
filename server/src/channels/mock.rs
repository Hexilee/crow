use super::Channel;
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
            Delay::new(Duration::from_millis(500)).await;
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            let mut points = Vec::new();
            for i in 1..20 {
                let f = i as f64;
                points.push(Point { x: f, y: f, z: f });
            }
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
