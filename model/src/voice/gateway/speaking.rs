use super::VoiceOpCode;
use bitflags::bitflags;

bitflags! {
    #[cfg_attr(
        feature = "serde-support",
        derive(serde::Deserialize, serde::Serialize)
    )]
    pub struct SpeakingFlags: u64 {
        const MICROPHONE = 1;
        const SOUNDSHARE = 1 << 1;
        const PRIORITY = 1 << 2;
    }
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Speaking {
    pub d: SpeakingInfo,
    pub op: VoiceOpCode,
}

impl Speaking {
    pub fn new(speaking: SpeakingFlags, delay: u64, ssrc: u64) -> Self {
        Self {
            d: SpeakingInfo {
                speaking,
                delay,
                ssrc,
            },
            op: VoiceOpCode::Speaking,
        }
    }
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeakingInfo {
    pub speaking: SpeakingFlags,
    pub delay: u64,
    pub ssrc: u64,
}