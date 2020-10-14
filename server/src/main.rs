mod channels;
mod curve;

use channels::mock::cos_channel;
use channels::ws_channel;
use channels::SyncChannels;
use futures::{stream::SplitStream, SinkExt, StreamExt, TryFutureExt};
use log::{debug, error, info, warn};
use roa::cors::Cors;
use roa::http::Method;
use roa::logger::logger;
use roa::preload::*;
use roa::router::{allow, Router};
use roa::websocket::tungstenite::protocol::frame::{coding::CloseCode, CloseFrame};
use roa::websocket::tungstenite::Error as WsError;
use roa::websocket::{Message, SocketStream, Websocket};
use roa::{App, Context, Next};
use std::borrow::Cow;
use std::env;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    pretty_env_logger::init();
    let server_addr = env::var("CROW_SERVER_ADDR")?;
    let channels = SyncChannels::new();
    cos_channel(channels.new_channel().await.1);

    let downstream_router = Router::new().gate(subscribe_guard).on(
        "/",
        allow([Method::GET], Websocket::new(handle_downstream_client)),
    );
    let router = Router::new()
        .on("upstream", Websocket::new(handle_upstream_client))
        .include("/downstream/:id", downstream_router);
    App::state(channels)
        .gate(logger)
        .gate(Cors::new())
        .end(router.routes("/")?)
        .listen(server_addr, |addr| info!("Server is listening on {}", addr))?
        .await?;
    Ok(())
}

async fn subscribe_guard(ctx: &mut Context<SyncChannels>, next: Next<'_>) -> roa::Result<()> {
    let index: usize = ctx.must_param("id")?.parse()?;
    ctx.get_channel(index).await?;
    next.await
}

async fn handle_upstream_client(ctx: Context<SyncChannels>, stream: SocketStream) {
    let (index, channel) = ctx.new_channel().await;
    let (mut sender, receiver) = stream.split();
    let result = sender
        .send(Message::Text(
            serde_json::to_string(&serde_json::json!({ "id": index })).unwrap(),
        ))
        .and_then(|_| ws_channel::handle(channel, receiver))
        .await;
    if let Err(err) = result {
        error!("ws error: {}", err)
    }
    ctx.remove_channel(index).await
}

async fn handle_downstream_client(ctx: Context<SyncChannels>, stream: SocketStream) {
    let index: usize = ctx.must_param("id").and_then(|id| id.parse()).unwrap();
    let channel = ctx.get_channel(index).await.unwrap();

    let (sender, receiver) = stream.split();
    let index = channel.register(sender).await;
    let result = handle_downstream_message(receiver).await;
    if let Some(mut sender) = channel.deregister(index).await {
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
}

async fn handle_downstream_message(mut receiver: SplitStream<SocketStream>) -> Result<(), WsError> {
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
