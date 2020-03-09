use super::curvature_splines::{CurvatureSplines, PointVector};
use super::SyncChannel;
use crate::curve::Curve;
use roa::websocket::Message;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn push_channel(channel: SyncChannel) {
    async_std::task::spawn(async move {
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
            .to_curve()
            .unwrap();
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            let curve = Curve { timestamp, points };
            let data = serde_json::to_string(&curve).unwrap();
            channel.broadcast(Message::Text(data)).await;
        }
    });
}
