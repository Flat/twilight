use twilight_model::id::{GuildId, UserId};
use futures::stream::StreamExt;
use futures::channel::mpsc::UnboundedReceiver;
use crate::session_handler::Session;
use std::sync::Arc;

use tokio_tungstenite::tungstenite::Message;

use crate::event::VoiceGatewayEvent;

use crate::error::{Error, Result};

pub struct VoiceProcessor {
    pub rx: UnboundedReceiver<Message>,
    pub session: Arc<Session>,
    url: String,
    server_id: GuildId,
    user_id: UserId,
    session_id: String,
    token: String,
}

impl VoiceProcessor {
    pub async fn new(endpoint: &str, server_id: GuildId, user_id: UserId, session_id: &str, token: &str) -> Result<Self> {
        let endpoint = if endpoint.starts_with("wss://") {
           format!("{}?v=4", endpoint)
        } else {
            format!("wss://{}?v=4", endpoint)
        };

        let stream = crate::connect::connect(&endpoint).await?;

        let (forwarder, rx, tx) = crate::socket_forwarder::SocketForwarder::new(stream);

        tokio::spawn(async move {
            forwarder.run().await
        });

        let session = std::sync::Arc::new(crate::session_handler::Session::new(tx));

        Ok(Self {
            rx,
            session,
            url: endpoint,
            server_id,
            user_id,
            session_id: session_id.to_owned(),
            token: token.to_owned(),
        })
    }

    pub async fn run(mut self) {
        loop {
            let gateway_event = match self.next_event().await {
                Ok(ev) => ev,
                Err(err) => {
                    log::warn!("Error receiveing gateway event: {:?}", err.source());
                    continue;
                },
            };

            // The only reason for an error is if the sender couldn't send a
            // message or if the session didn't exist when it should, so do a
            // reconnect if this fails.
            if self.process(&gateway_event).await.is_err() {
                log::debug!("Error processing event; reconnecting");

                self.reconnect().await;

                continue;
            }
        }
    }

    async fn process(&mut self, event: &VoiceGatewayEvent) -> Result<()> {
        match event {
            VoiceGatewayEvent::Ready(_ready) => {
                todo!()
            }
            VoiceGatewayEvent::SessionDescription(_sd) => {
                todo!()
            }
            VoiceGatewayEvent::Speaking(_speak) => {
                todo!()
            }
            VoiceGatewayEvent::HeartbeatAck => {
                self.session.heartbeats.receive().await;
                Ok(())
            }
            VoiceGatewayEvent::Hello(interval) => {
                self.session.set_heartbeat_interval(*interval);
                self.session.start_heartbeater().await;
                
                self.identify().await?;
                Ok(())
            }
            VoiceGatewayEvent::Resumed => {
                todo!()
            }
            VoiceGatewayEvent::ClientDisconnect(_user) => {
                todo!()
            }
            VoiceGatewayEvent::Unknown(op, value) => {
                log::info!("Got unknown voice event: {:?}, {:?}", op, value);
                Ok(())
            }
        }
    }

    async fn identify(&mut self) -> Result<()> {
        let identify = twilight_model::voice::gateway::VoiceIdentify::new(self.server_id, self.user_id, &self.session_id, &self.token);

        self.send(identify).await?;
        Ok(())
    }

    async fn reconnect(&mut self) {
        todo!()
    }

    async fn resume(&mut self) -> Result<()> {
        todo!()
    }

    pub async fn send(&mut self, payload: impl serde::Serialize) -> Result<()> {
        self.session.send(payload)?;
        Ok(())
    }

    async fn next_event(&mut self) -> Result<VoiceGatewayEvent> {
        loop {
            // Returns None when the socket forwarder has ended, meaning the
            // connection was dropped.
            let msg = if let Some(msg) = self.rx.next().await {
                msg
            } else {
                if let Err(why) = self.resume().await {
                    log::warn!("Resume failed with {}, reconnecting", why);
                    self.reconnect().await;
                }
                continue;
            };

            match msg {
                Message::Binary(_) => {},
                Message::Close(_) => self.resume().await?,
                Message::Ping(_) | Message::Pong(_) => {},
                Message::Text(text) => {
                    break match serde_json::from_str(&text) {
                        Ok(ser) => Ok(ser),
                        Err(err) => {
                            log::debug!("Broken JSON: {:?}", &text);
                            Err(Box::new(err))
                        },
                    };
                },
            }
        }
    }
}