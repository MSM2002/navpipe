use reqwest::{Client, ClientBuilder};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct AsyncNavPipeHTTP {
    client: Arc<Client>,
}

impl AsyncNavPipeHTTP {
    pub fn new() -> Self {
        let client = ClientBuilder::new()
            // CRITICAL: Forces the older, more stable protocol
            .http1_only()
            // Some free APIs block "empty" User-Agents
            .user_agent("navpipe")
            .timeout(Duration::from_secs(30))
            .tcp_keepalive(Duration::from_secs(60))
            // Prevents connection pool "stale" errors
            .pool_idle_timeout(Duration::from_secs(15))
            .build()
            .expect("Failed to build reqwest client");

        Self {
            client: Arc::new(client),
        }
    }

    pub async fn get_path<T: DeserializeOwned>(&self, path: &str) -> Result<T, reqwest::Error> {
        let url = format!("https://api.mfapi.in/{}", path.trim_start_matches('/'));

        // Use a simple retry for transient "stream" or "reset" errors
        let mut attempts = 0;
        loop {
            match self.client.get(&url).send().await {
                Ok(resp) => {
                    let resp = resp.error_for_status()?;
                    return resp.json::<T>().await;
                }
                Err(e) if attempts < 2 && (e.is_connect() || e.is_request() || e.is_timeout()) => {
                    attempts += 1;
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
