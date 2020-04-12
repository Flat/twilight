use futures::StreamExt;
use futures::SinkExt;
use std::{env, error::Error};
use twilight_gateway::Shard;
use twilight_gateway::shard::Event;

use twilight_audio::VoiceProcessor;

use twilight_model::id::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    pretty_env_logger::init_timed();
    let token = env::var("DISCORD_TOKEN")?;
    let shard = Shard::new(&token).await?;
    println!("Created shard");

    let mut events = shard.events().await;
    
    while let Some(event) = events.next().await {
        match event {
            Event::VoiceServerUpdate(vsu) => {
                let session = shard.session().id().await;
                let vp = VoiceProcessor::new(&vsu.endpoint.unwrap(), vsu.guild_id.unwrap(), UserId(561204097865351207), &session.unwrap(), &token).await;
            }
            Event::MessageCreate(msg) => {
                if msg.content == "!join" {
                    let mut sink = shard.sink();
                    let vsu = twilight_model::gateway::payload::UpdateVoiceState::new(GuildId(381880193251409931), ChannelId(381880193700069380), false, false);
                    let ser = serde_json::to_string(&vsu).unwrap();
                    let payload = tungstenite::Message::Text(ser);
                    sink.send(payload).await.unwrap();
                }
            }
            _ => {}
        }
    }

    Ok(())
}
