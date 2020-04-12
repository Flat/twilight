use crate::id::GuildId;
use super::opcode::VoiceOpCode;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoiceResume {
    pub d: VoiceResumeInfo,
    pub op: VoiceOpCode,
}

impl VoiceResume {
    pub fn new(server_id: GuildId, session_id: &str, token: &str) -> Self {
        let info = VoiceResumeInfo::new(server_id, session_id, token);
        Self::new_from_info(info)
    }

    pub fn new_from_info(info: VoiceResumeInfo) -> Self {
        Self {
            d: info,
            op: VoiceOpCode::Resume,
        }
    }
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoiceResumeInfo {
    pub server_id: GuildId,
    pub session_id: String,
    pub token: String,
}

impl VoiceResumeInfo {
    pub fn new(server_id: GuildId, session_id: &str, token: &str) -> Self {
        let token = if token.starts_with("Bot ") {
            let (_, token_part) = token.split_at(4);
            String::from(token_part)
        } else {
            String::from(token)
        };
        Self {
            server_id,
            session_id: String::from(session_id),
            token,
        }
    }
}
