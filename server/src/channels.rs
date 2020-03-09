mod curvature_splines;
mod mock;
use async_std::sync::{Mutex, RwLock};
use futures::stream::SplitSink;
use futures::SinkExt;
use log::error;
use roa::websocket::{Message, SocketStream};
use slab::Slab;
use std::sync::Arc;

type Sender = SplitSink<SocketStream, Message>;
type Channel = Slab<Mutex<Sender>>;

#[derive(Clone)]
pub struct SyncChannel(Arc<RwLock<Channel>>);

impl SyncChannel {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(Slab::new())))
    }

    pub async fn broadcast(&self, message: Message) {
        let channel = self.0.read().await;
        for (_, sender) in channel.iter() {
            if let Err(err) = sender.lock().await.send(message.clone()).await {
                error!("broadcast error: {}", err);
            }
        }
    }

    pub async fn send(&self, index: usize, message: Message) {
        if let Err(err) = self.0.read().await[index].lock().await.send(message).await {
            error!("message send error: {}", err)
        }
    }

    pub async fn register(&self, sender: Sender) -> usize {
        self.0.write().await.insert(Mutex::new(sender))
    }

    pub async fn deregister(&self, index: usize) -> Sender {
        self.0.write().await.remove(index).into_inner()
    }
}

pub use mock::push_channel;
