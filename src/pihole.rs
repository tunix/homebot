use std::time::Duration;

use serde::Deserialize;

pub struct Pihole {
    base_url: String,
    token: String,
}

impl Pihole {
    pub fn new(base_url: String, token: String) -> Self {
        Self { base_url, token }
    }

    fn get_url(&self, path: &str) -> String {
        format!(
            "{}/admin/api.php?auth={}&{}",
            self.base_url, self.token, path
        )
    }

    pub async fn is_enabled(&self) -> Result<bool, reqwest::Error> {
        let url = self.get_url("summary");

        log::info!("Sending request to: {}", url);

        let response = reqwest::get(url)
            .await?
            .json::<PiHoleSummaryResponse>()
            .await?;

        match response.status.as_str() {
            "enabled" => Ok(true),
            _ => Ok(false),
        }
    }

    pub async fn disable(&self, duration: Duration) -> Result<bool, reqwest::Error> {
        let url = self.get_url(format!("disable={}", duration.as_secs()).as_str());

        log::info!("Sending request to: {}", url);

        let response = reqwest::get(url)
            .await?
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
