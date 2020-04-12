use crate::error::{Error, Result};
use log::debug;
use std::str::FromStr;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use url::Url;

use crate::ShardStream;

pub async fn connect(url: &str) -> Result<ShardStream> {
    let url = Url::from_str(url)?;

    let request = url
        .into_client_request()?;
    let (stream, _) = tokio_tungstenite::connect_async(request).await?;

    debug!("Shook hands with remote");

    Ok(stream)
}
