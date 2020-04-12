use crate::voice::gateway::opcode::VoiceOpCode;
use crate::id::{GuildId, UserId};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VoiceIdentify {
    pub d: VoiceIdentifyInfo,
    pub op: VoiceOpCode,
}

impl VoiceIdentify {
    pub fn new(server_id: GuildId, user_id: UserId, session_id: &str, token: &str) -> Self {
        let info = VoiceIdentifyInfo::new(server_id, user_id, session_id, token);
        Self::new_from_info(info)
    }

    pub fn new_from_info(info: VoiceIdentifyInfo) -> Self {
        Self {
            d: info,
            op: VoiceOpCode::Identify,
        }
    }
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VoiceIdentifyInfo {
    pub server_id: GuildId,
    pub user_id: UserId,
    pub session_id: String,
    pub token: String,
}

impl VoiceIdentifyInfo {
    pub fn new(server_id: GuildId, user_id: UserId, session_id: &str, token: &str) -> Self {
        let token = if token.starts_with("Bot ") {
            let (_, token_part) = token.split_at(4);
            String::from(token_part)
        } else {
            String::from(token)
        };
        Self {
            server_id,
            user_id,
            session_id: String::from(session_id),
            token,
        }
    }
}