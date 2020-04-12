#![allow(clippy::wildcard_imports)]
use serde::{
    de::{Deserialize, Deserializer, Error as DeError, MapAccess, Visitor},
    Deserialize as DeserializeMacro,
};
use serde_value::Value;
use std::fmt::{Formatter, Result as FmtResult};
use twilight_model::voice::gateway::*;

#[cfg(feature = "metrics")]
use metrics::counter;

#[derive(Clone, Debug)]
pub enum VoiceGatewayEvent {
    Ready(twilight_model::voice::gateway::VoiceReady),
    SessionDescription(twilight_model::voice::gateway::SessionDescription),
    Speaking(twilight_model::voice::gateway::SpeakingInfo),
    HeartbeatAck,
    Hello(u64),
    Resumed,
    ClientDisconnect(twilight_model::id::UserId),
    Unknown(VoiceOpCode,Option<Value>)
}

#[derive(DeserializeMacro)]
#[serde(field_identifier, rename_all = "lowercase")]
enum Field {
    D,
    Op,
}

struct VoiceGatewayEventVisitor;

impl<'de> Visitor<'de> for VoiceGatewayEventVisitor {
    type Value = VoiceGatewayEvent;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.write_str("struct VoiceGatewayEvent")
    }

    fn visit_map<V>(self, mut map: V) -> Result<VoiceGatewayEvent, V::Error>
    where
        V: MapAccess<'de>,
    {
        // Have to use a serde_json::Value here because serde has no
        // abstract container type.
        let mut d = None::<Value>;
        let mut op = None::<VoiceOpCode>;

        while let Some(key) = map.next_key()? {
            match key {
                Field::D => {
                    if d.is_some() {
                        return Err(DeError::duplicate_field("d"));
                    }

                    d = Some(map.next_value()?);
                },
                Field::Op => {
                    if op.is_some() {
                        return Err(DeError::duplicate_field("op"));
                    }

                    op = Some(map.next_value()?);
                },
            }
        }

        let op = op.ok_or_else(|| DeError::missing_field("op"))?;

        Ok(match op {
            VoiceOpCode::Ready => {
                let d = d.ok_or_else(|| DeError::missing_field("d"))?;
                let ready = VoiceReady::deserialize(d).map_err(DeError::custom)?;

                VoiceGatewayEvent::Ready(ready)
            }
            VoiceOpCode::SessionDescription => {
                let d = d.ok_or_else(|| DeError::missing_field("d"))?;
                let session_description = SessionDescription::deserialize(d).map_err(DeError::custom)?;

                VoiceGatewayEvent::SessionDescription(session_description)
            }
            VoiceOpCode::Speaking => {
                let d = d.ok_or_else(|| DeError::missing_field("d"))?;
                let speaking = SpeakingInfo::deserialize(d).map_err(DeError::custom)?;

                VoiceGatewayEvent::Speaking(speaking)
            }
            VoiceOpCode::HeartbeatAck => VoiceGatewayEvent::HeartbeatAck,
            VoiceOpCode::Hello => {
                let d = d.ok_or_else(|| DeError::missing_field("d"))?;
                #[derive(DeserializeMacro)]
                struct VoiceHello {
                    heartbeat_interval: u64,
                }

                let hello = VoiceHello::deserialize(d).map_err(DeError::custom)?;

                VoiceGatewayEvent::Hello(hello.heartbeat_interval)
            },
            VoiceOpCode::Resumed => VoiceGatewayEvent::Resumed,
            VoiceOpCode::ClientDisconnect => {
                let d = d.ok_or_else(|| DeError::missing_field("d"))?;
                #[derive(DeserializeMacro)]
                struct ClientDisconnect {
                    user_id: twilight_model::id::UserId,
                }

                let dc = ClientDisconnect::deserialize(d).map_err(DeError::custom)?;

                VoiceGatewayEvent::ClientDisconnect(dc.user_id)
            },
            VoiceOpCode::Identify | VoiceOpCode::SelectProtocol | VoiceOpCode::Heartbeat | VoiceOpCode::Resume => {
                log::warn!("Got opcode that should only be sent by the client");
                Err(DeError::custom("Client only opcode received"))?
            }
            VoiceOpCode::Unknown => {
                log::warn!("Got unknown opcode: {:?}", op);
                VoiceGatewayEvent::Unknown(op, d)
            }
        })
    }
}

impl<'de> Deserialize<'de> for VoiceGatewayEvent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        const FIELDS: &[&str] = &["d", "op"];

        deserializer.deserialize_struct("VoiceGatewayEvent", FIELDS, VoiceGatewayEventVisitor)
    }
}
