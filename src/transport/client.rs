use reqwest::Client;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use crate::schema::nav::DateRange;

#[derive(Clone)]
pub struct AsyncNavPipeHTTP {
    client: Arc<Client>,
}

impl AsyncNavPipeHTTP {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Client::new()),
        }
    }

    pub async fn get_nav<T: DeserializeOwned>(
        &self,
        scheme_code: u32,
        _date_range: Option<&DateRange>,
    ) -> Result<T, reqwest::Error> {
        let url = format!("https://api.mfapi.in/mf/{}", scheme_code);
        let resp = self.client.get(&url).send().await?;

        // consume the response after checking status
        let resp = resp.error_for_status()?;
        resp.json::<T>().await
    }
}