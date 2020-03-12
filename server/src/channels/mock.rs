use super::curvature_splines::PointSlice;
use super::SyncChannel;
use crate::curve::Curve;
use rand::Rng;
use roa::websocket::Message;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn push_channel(channel: SyncChannel) {
    async_std::task::spawn(async move {
        let mut curvatures = [0., 0., 0., 0., 0., 0.];
        loop {
            {
                let mut rng = rand::thread_rng();
                for i in curvatures.iter_mut() {
                    *i += rng.gen_range(-0.001, 0.001);
                }
            }
            let points = vec![
                (4.66, curvatures[0], 0.),
                (9.36, curvatures[1], 0.),
                (14.82, curvatures[2], 0.),
                (19.72, curvatures[3], 0.),
                (24.74, curvatures[4], 0.),
                (29.95, curvatures[5], 0.),
            ]
            .interpolate(0.2)
            .curvature_reconstruct()
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
