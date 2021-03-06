pub mod mock;
pub mod ws_channel;

mod curvature_splines;
use async_std::sync::{Mutex, RwLock};
use futures::stream::SplitSink;
use futures::SinkExt;
use log::error;
use roa::http::StatusCode;
use roa::websocket::{Message, SocketStream};
use roa::{status, Result};
use slab::Slab;
use std::sync::Arc;

type Sender = SplitSink<SocketStream, Message>;
type Channel = Slab<Mutex<Sender>>;

#[derive(Clone)]
pub struct SyncChannel(Arc<RwLock<Channel>>);

#[derive(Clone)]
pub struct SyncChannels(Arc<RwLock<Slab<SyncChannel>>>);

impl SyncChannel {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(Slab::new())))
    }

    pub async fn broadcast(&self, message: Message) {
        let channel = self.0.read().await;
        let mut broken_sender = Vec::new();
        for (index, sender) in channel.iter() {
            if let Err(err) = sender.lock().await.send(message.clone()).await {
                error!("broadcast error: {}", err);
                broken_sender.push(index);
            }
        }
        for index in broken_sender {
            self.0.write().await.remove(index);
        }
    }

    #[allow(dead_code)]
    pub async fn send(&self, index: usize, message: Message) {
        if let Err(err) = self.0.read().await[index].lock().await.send(message).await {
            error!("message send error: {}", err)
        }
    }

    pub async fn register(&self, sender: Sender) -> usize {
        self.0.write().await.insert(Mutex::new(sender))
    }

    pub async fn deregister(&self, index: usize) -> Option<Sender> {
        let mut channel = self.0.write().await;
        if channel.contains(index) {
            Some(channel.remove(index).into_inner())
        } else {
            None
        }
    }

    pub async fn deregister_all(&self) {
        for sender in self.0.write().await.drain() {
            if let Err(err) = sender.lock().await.close().await {
                error!("error in close websocket: {}", err)
            }
        }
    }
}

impl SyncChannels {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(Slab::new())))
    }

    pub async fn new_channel(&self) -> (usize, SyncChannel) {
        let channel = SyncChannel::new();
        (self.0.write().await.insert(channel.clone()), channel)
    }

    pub async fn get_channel(&self, index: usize) -> Result<SyncChannel> {
        match self.0.read().await.get(index) {
            Some(channel) => Ok(channel.clone()),
            None => Err(status!(
                StatusCode::NOT_FOUND,
                format!("channel {} not found", index)
            )),
        }
    }

    pub async fn remove_channel(&self, index: usize) {
        let channel = self.0.write().await.remove(index);
        channel.deregister_all().await
    }
}
