/*
Example Session Description Payload

{
    "op": 4,
    "d": {
        "mode": "xsalsa20_poly1305_lite",
        "secret_key": [ ...251, 100, 11...]
    }
}
*/

use super::select_protocol::EncryptionMode;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SessionDescription {
    mode: EncryptionMode,
    secret_key: Vec<u8>,
}