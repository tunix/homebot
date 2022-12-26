use std::time::Duration;

use serde::Deserialize;

const API_PATH: &'static str = "/admin/api.php";

pub struct AdProtection {
    base_url: String,
    token: String,
}

impl AdProtection {
    pub fn new(base_url: String, token: String) -> Self {
        Self { base_url, token }
    }

    pub async fn is_enabled(&self) -> Result<bool, reqwest::Error> {
        let url = format!("{}{API_PATH}?summary", self.base_url);

        log::info!("Sending request to: {}", url);

        let response = reqwest::get(url).await?
            .json::<PiHoleSummaryResponse>()
            .await?;

        match response.status.as_str() {
            "enabled" => Ok(true),
            _ => Ok(false),
        }
    }

    pub async fn disable(&self, duration: Duration) -> Result<bool, reqwest::Error> {
        let params = format!("?disable={}&auth={}", duration.as_secs(), self.token);
        let url = format!("{}{API_PATH}{}", self.base_url, params);

        log::info!("Sending request to: {}", url);

        let response = reqwest::get(url).await?
            .json::<PiHoleSummaryResponse>()
            .await?;

        match response.status.as_str() {
            "disabled" => Ok(true),
            _ => Ok(false),
        }
    }
}

#[derive(Deserialize)]
struct PiHoleSummaryResponse {
    status: String,
}
