#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoiceReady {
    pub srrc: u64,
    pub ip: String,
    pub port: u16,
    pub modes: Vec<String>,
    pub heartbeat_interval: u64,
}
