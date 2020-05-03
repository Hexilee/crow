use super::curvature_splines::PointSlice;
use super::SyncChannel;
use crate::curve::Curve;
use libflate::deflate::Encoder;
use num::{One, Zero};
use rand::Rng;
use roa::websocket::Message;
use std::io::Write;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const MAX_FPS: u64 = 60;

#[allow(dead_code)]
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

#[allow(dead_code)]
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

pub fn cos_channel(channel: SyncChannel) {
    use nalgebra::{Matrix3, Vector3};
    use std::f64::consts::PI;
    /// Get curvature of this point.
    fn cos_curvature(x: f64) -> f64 {
        x.cos() / (1. + x.sin().powi(2)).powi(3).sqrt()
    }

    /// Get arc length.
    fn cos_s(from: f64, to: f64) -> f64 {
        use gkquad::single::Integrator;
        // integral to calculate arc length.
        Integrator::new(|x: f64| (1. + x.sin().powi(2)).sqrt())
            .run(from..to)
            .estimate()
            .unwrap()
    }

    async_std::task::spawn(async move {
        const PLUS: f64 = 0.01;
        let min_period_ms = Duration::from_millis(1000 / MAX_FPS);
        let mut offset = 0.;
        let init_x = (0..9) // nine sample points.
            .map(|i| i as f64 * 2. * PI / 8.)
            .collect::<Vec<_>>();
        loop {
            let start = SystemTime::now();
            let points = init_x
                .iter()
                .map(|x| {
                    (
                        cos_s(offset, offset + x),
                        cos_curvature(offset + x) / 2f64.sqrt(),
                        cos_curvature(offset + x) / 2f64.sqrt(),
                    )
                }) // get pair (<arc length>, <curvature>, 0.)
                .collect::<Vec<_>>()
                .interpolate(0.05) // linear interpolate; ds = 0.05.
                .frenet_reconstruct(
                    Vector3::new(0., 0., 1.),                          // initialized coordinate
                    Matrix3::new(0., 0., 1., 0., 1., 0., -1., 0., 0.), // initialized rotation matrix
                )
                .unwrap();
            let timestamp = start.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
            let curve = Curve { timestamp, points };
            let mut encoder = Encoder::new(Vec::new());
            encoder
                .write_all(&serde_json::to_vec(&curve).unwrap())
                .unwrap();
            channel
                .broadcast(Message::Binary(encoder.finish().into_result().unwrap()))
                .await;
            let cost = start.elapsed().unwrap();
            if cost < min_period_ms {
                async_std::task::sleep(min_period_ms - cost).await;
            }
            offset += PLUS;
        }
    });
}
