use crate::voice::gateway::opcode::VoiceOpCode;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VoiceHeartbeat {
    pub d: u64,
    pub op: VoiceOpCode,
}

impl VoiceHeartbeat {
    pub fn new(nonce: u64) -> Self {
        Self {
            d: nonce,
            op: VoiceOpCode::Heartbeat,
        }
    }
}
