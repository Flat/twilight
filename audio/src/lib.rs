pub mod session;
pub mod session_handler;
pub mod event;
pub mod socket_forwarder;
pub mod heartbeat;
mod connect;
mod audio_connection;

use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub use session::VoiceProcessor;

pub type ShardStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

mod error {
    pub type Error = Box<dyn std::error::Error>;
    pub type Result<T> = std::result::Result<T, Error>;
}