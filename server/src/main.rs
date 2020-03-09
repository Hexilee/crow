mod channels;
mod curve;
use channels::{push_channel, SyncChannel};
use futures::{stream::SplitStream, SinkExt, StreamExt};
use log::{debug, error, info, warn};
use roa::cors::Cors;
use roa::logger::logger;
use roa::preload::*;
use roa::websocket::tungstenite::protocol::frame::{coding::CloseCode, CloseFrame};
use roa::websocket::tungstenite::Error as WsError;
use roa::websocket::{Message, SocketStream, Websocket};
use roa::{App, SyncContext};
use std::borrow::Cow;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let channel = SyncChannel::new();
    push_channel(channel.clone());
    let mut app = App::new(channel);
    app.gate(logger)
        .gate(Cors::new())
        .gate(Websocket::new(handle_ws_client));
    app.listen("127.0.0.1:8000", |addr| {
        info!("Server is listening on {}", addr)
    })?
    .await?;
    Ok(())
}

async fn handle_ws_client(ctx: SyncContext<SyncChannel>, stream: SocketStream) {
    let (sender, receiver) = stream.split();
    let index = ctx.register(sender).await;
    let result = handle_message(&ctx, index, receiver).await;
    let mut sender = ctx.deregister(index).await;
    if let Err(err) = result {
        let result = sender
            .send(Message::Close(Some(CloseFrame {
                code: CloseCode::Invalid,
                reason: Cow::Owned(err.to_string()),
            })))
            .await;
        if let Err(err) = result {
            error!("send close message error: {}", err)
        }
    }
}

async fn handle_message(
    _ctx: &SyncContext<SyncChannel>,
    _index: usize,
    mut receiver: SplitStream<SocketStream>,
) -> Result<(), WsError> {
    while let Some(message) = receiver.next().await {
        let message = message?;
        match message {
            Message::Close(frame) => {
                debug!("websocket connection close: {:?}", frame);
                break;
            }
            Message::Ping(ref data) => info!("client ping: {}", String::from_utf8_lossy(data)),
            Message::Pong(ref data) => warn!("ignored pong: {}", String::from_utf8_lossy(data)),
            Message::Text(ref data) => info!("receive a message: {}", data),
            Message::Binary(ref data) => {
                info!("receive a message: {}", String::from_utf8_lossy(data))
            }
        }
    }
    Ok(())
}
