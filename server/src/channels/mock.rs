use super::curvature_splines::PointSlice;
use super::SyncChannel;
use crate::curve::Curve;
use num::{One, Zero};
use rand::Rng;
use roa::websocket::Message;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const MAX_FPS: u64 = 60;

pub fn random_channel(channel: SyncChannel) {
    async_std::task::spawn(async move {
        let mut curvatures = [0., 0., 0., 0., 0., 0.];
        let min_period_ms = Duration::from_millis(1000 / MAX_FPS);
        loop {
            let start = SystemTime::now();
            {
                let mut rng = rand::thread_rng();
                for i in curvatures.iter_mut() {
                    *i += rng.gen_range(-0.001, 0.001);
                }
            }
            let points = [
                (0., 0., 0.),
                (4.66, curvatures[0], 0.),
                (9.36, curvatures[1], 0.),
                (14.82, curvatures[2], 0.),
                (19.72, curvatures[3], 0.),
                (24.74, curvatures[4], 0.),
                (29.95, curvatures[5], 0.),
            ]
            .interpolate(0.1)
            .frenet_reconstruct(Zero::zero(), One::one())
            .unwrap();
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            let curve = Curve { timestamp, points };
            let data = serde_json::to_string(&curve).unwrap();
            channel.broadcast(Message::Text(data)).await;
            let cost = start.elapsed().unwrap();
            if cost < min_period_ms {
                async_std::task::sleep(min_period_ms - cost).await;
            }
        }
    });
}

pub fn static_channel(channel: SyncChannel) {
    async_std::task::spawn(async move {
        let data = [
            (0., 0., 0.),
            (4.66, 0.21, 0.),
            (9.36, 0.27, 0.),
            (14.82, 0.086, 0.),
            (19.72, -0.0093, 0.),
            (24.74, -0.091, 0.),
            (29.95, -0.079, 0.),
        ];
        loop {
            let points = data
                .interpolate(0.1)
                .frenet_reconstruct(Zero::zero(), One::one())
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
