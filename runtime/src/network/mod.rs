use anyhow::Result;
use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{info, warn};

/// WebSocket client used for connecting to the Go dev server (hot-reload).
pub struct DevClient {
    url: String,
}

impl DevClient {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    /// Connect and call `handler` for every text message received.
    pub async fn listen<F>(&self, mut handler: F) -> Result<()>
    where
        F: FnMut(String),
    {
        info!("DevClient connecting to {}", self.url);
        let (mut ws, _) = connect_async(&self.url).await?;
        info!("DevClient connected");

        while let Some(msg) = ws.next().await {
            match msg? {
                Message::Text(text) => handler(text.to_string()),
                Message::Close(_)   => {
                    warn!("DevClient: server closed connection");
                    break;
                }
                _ => {}
            }
        }
        Ok(())
    }
}
