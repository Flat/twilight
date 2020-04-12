use super::{
    error::{Error, Result},
    heartbeat::{Heartbeater, Heartbeats},
};
use futures::{
    channel::mpsc::UnboundedSender,
    future::{self, AbortHandle},
    lock::Mutex,
};
use serde::ser::Serialize;
use std::{
    convert::TryFrom,
    sync::{
        atomic::{AtomicU64, AtomicU8, Ordering},
        Arc,
    },
};
use twilight_model::voice::gateway::VoiceHeartbeat;



use tokio_tungstenite::tungstenite::{protocol::CloseFrame, Message as TungsteniteMessage};

#[derive(Debug)]
pub struct Session {
    // Needs to be Arc so it can be cloned in the `Drop` impl when spawned on
    // the runtime.
    pub heartbeater_handle: Arc<Mutex<Option<AbortHandle>>>,
    pub heartbeats: Arc<Heartbeats>,
    pub heartbeat_interval: AtomicU64,
    pub tx: UnboundedSender<TungsteniteMessage>,
}

impl Session {
    pub fn new(tx: UnboundedSender<TungsteniteMessage>) -> Self {
        Self {
            heartbeater_handle: Arc::new(Mutex::new(None)),
            heartbeats: Arc::new(Heartbeats::default()),
            heartbeat_interval: AtomicU64::new(0),
            tx,
        }
    }

    /// Sends a payload as a message over the socket.
    ///
    /// # Errors
    ///
    /// Returns [`Error::PayloadSerialization`] when there is an error
    /// serializing the payload into an acceptable format.
    ///
    /// Returns [`Error::SendingMessage`] when the receiving channel has hung
    /// up. This will only happen when the shard has either not started or has
    /// already shutdown.
    ///
    /// [`Error::PayloadSerialization`]: ../enum.Error.html#variant.PayloadSerialization
    /// [`Error::SendingMessage`]: ../enum.Error.html#variant.SendingMessage
    pub fn send(&self, payload: impl Serialize) -> Result<()> {
        let bytes = serde_json::to_vec(&payload)?;

        self.tx
            .unbounded_send(TungsteniteMessage::Binary(bytes))?;

        Ok(())
    }

    pub fn close(&self, close_frame: Option<CloseFrame<'static>>) -> Result<()> {
        self.tx
            .unbounded_send(TungsteniteMessage::Close(close_frame))?;

        Ok(())
    }

    pub fn heartbeat_interval(&self) -> u64 {
        self.heartbeat_interval.load(Ordering::Relaxed)
    }

    pub fn set_heartbeat_interval(&self, new_heartbeat_interval: u64) {
        self.heartbeat_interval
            .store(new_heartbeat_interval, Ordering::Release);
    }

    pub async fn heartbeat(&self) -> Result<()> {
        use rand::RngCore;
        use rand::SeedableRng;
        let mut rng = rand::rngs::SmallRng::from_entropy();
        self.send(VoiceHeartbeat::new(rng.next_u64()))
    }

    pub async fn stop_heartbeater(&self) {
        if let Some(handle) = self.heartbeater_handle.lock().await.take() {
            handle.abort();
        }
    }

    pub async fn start_heartbeater(&self) {
        let interval = self.heartbeat_interval();
        let heartbeats = Arc::clone(&self.heartbeats);

        let heartbeater = Heartbeater::new(heartbeats, interval, self.tx.clone()).run();
        let (fut, handle) = future::abortable(heartbeater);

        tokio::spawn(async {
            let _ = fut.await;
        });

        if let Some(old) = self.heartbeater_handle.lock().await.replace(handle) {
            old.abort();
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let handle = Arc::clone(&self.heartbeater_handle);

        let _ = tokio::spawn(async move {
            if let Some(handle) = handle.lock().await.take() {
                handle.abort();
            }
        });
    }
}
