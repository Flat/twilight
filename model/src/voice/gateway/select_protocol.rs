use super::VoiceOpCode;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SelectProtocol {
    pub d: SelectProtocolInfo,
    pub op: VoiceOpCode,
}

impl SelectProtocol {
    pub fn new(info: SelectProtocolInfo) -> Self {
        Self {
            d: info,
            op: VoiceOpCode::SelectProtocol,
        }
    }
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SelectProtocolInfo {
    protocol: String,
    data: SelectProtocolData,
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EncryptionMode {
    #[cfg_attr(
        feature = "serde-support",
        serde(rename = "xsalsa20_poly1305")
    )]
    Normal,
    #[cfg_attr(
        feature = "serde-support",
        serde(rename = "xsalsa20_poly1305_suffix")
    )]
    Suffix,
    #[cfg_attr(
        feature = "serde-support",
        serde(rename = "xsalsa20_poly1305_lite")
    )]
    Lite,
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SelectProtocolData {
    address: String,
    port: u16,
    mode: EncryptionMode,
}
