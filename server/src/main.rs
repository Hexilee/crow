mod channels;
mod curve;

use async_std::path::{Path, PathBuf};
use channels::mock::cos_channel;
use channels::SyncChannel;
use futures::{stream::SplitStream, SinkExt, StreamExt};
use log::{debug, error, info, warn};
use roa::body::DispositionType::Inline;
use roa::cors::Cors;
use roa::http::{Method, StatusCode};
use roa::logger::logger;
use roa::preload::*;
use roa::router::{allow, get, Router};
use roa::websocket::tungstenite::protocol::frame::{coding::CloseCode, CloseFrame};
use roa::websocket::tungstenite::Error as WsError;
use roa::websocket::{Message, SocketStream, Websocket};
use roa::{throw, App, Context};
use std::borrow::Cow;
use std::env;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    pretty_env_logger::init();
    let server_addr = env::var("CROW_SERVER_ADDR")?;
    let channel = SyncChannel::new();
    cos_channel(channel.clone());
    let router = Router::new()
        .on("/", get(index))
        .on(
            "/ws",
            allow([Method::GET], Websocket::new(handle_ws_client)),
        )
        .on("/*{file}", get(serve_sources));
    App::state(channel)
        .gate(logger)
        .gate(Cors::new())
        .end(router.routes("/")?)
        .listen(server_addr, |addr| info!("Server is listening on {}", addr))?
        .await?;
    Ok(())
}

async fn handle_ws_client(ctx: Context<SyncChannel>, stream: SocketStream) {
    let (sender, receiver) = stream.split();
    let index = ctx.register(sender).await;
    let result = handle_message(receiver).await;
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

async fn handle_message(mut receiver: SplitStream<SocketStream>) -> Result<(), WsError> {
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

async fn serve(ctx: &mut Context<SyncChannel>, path: impl AsRef<Path>) -> roa::Result {
    let dist = env::var("DIST_DIR")?;
    let path = PathBuf::from(dist).join(path);
    if !path.exists().await {
        throw!(StatusCode::NOT_FOUND, format!("file not found"))
    }
    ctx.write_file(path, Inline).await
}

async fn index(ctx: &mut Context<SyncChannel>) -> roa::Result {
    serve(ctx, "index.html").await
}

async fn serve_sources(ctx: &mut Context<SyncChannel>) -> roa::Result {
    let file = ctx.must_param("file")?;
    if !file.contains("..") && (file.ends_with("js") || file.ends_with("css")) {
        serve(ctx, file.as_str()).await?;
    }
    throw!(
        StatusCode::NOT_FOUND,
        format!("file {} not found", file.as_str())
    )
}
