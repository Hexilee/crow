use super::curvature_splines::PointSlice;
use super::SyncChannel;
use crate::curve::Curve;

use futures::{stream::SplitStream, StreamExt};
use libflate::deflate::Encoder;
use log::{debug, error, info, warn};
use num::{one, zero};
use roa::websocket::tungstenite::Error as WsError;
use roa::websocket::{Message, SocketStream};

use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn handle(
    channel: SyncChannel,
    mut stream: SplitStream<SocketStream>,
) -> Result<(), WsError> {
    async fn response(channel: SyncChannel, raw_data: &[u8]) {
        let data: Vec<(f64, f64, f64)> = match serde_json::from_slice(raw_data) {
            Ok(d) => d,
            Err(_) => {
                error!("wrong data from source client");
                return;
            }
        };

        let points = data
            .interpolate(0.05)
            .frenet_reconstruct(zero(), one())
            .unwrap();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let curve = Curve { timestamp, points };
        let mut encoder = Encoder::new(Vec::new());
        encoder
            .write_all(&serde_json::to_vec(&curve).unwrap())
            .unwrap();
        channel
            .broadcast(Message::Binary(encoder.finish().into_result().unwrap()))
            .await;
    };

    while let Some(message) = stream.next().await {
        let message = message?;
        match message {
            Message::Close(frame) => {
                debug!("websocket connection close: {:?}", frame);
                break;
            }
            Message::Ping(ref data) => info!("client ping: {}", String::from_utf8_lossy(data)),
            Message::Pong(ref data) => warn!("ignored pong: {}", String::from_utf8_lossy(data)),
            Message::Binary(ref data) => response(channel.clone(), data.as_slice()).await,
            Message::Text(ref data) => response(channel.clone(), data.as_bytes()).await,
        }
    }
    Ok(())
}
