use std::time::Duration;

const API_PATH: &'static str = "/admin/api.php";

pub struct AdProtection {
    base_url: String,
    token: String,
}

impl AdProtection {
    pub fn new(base_url: String, token: String) -> Self {
        Self { base_url, token }
    }

    pub fn is_enabled(&self) -> bool {
        let url = format!("{}{API_PATH}?summary", self.base_url);

        log::info!("Sending request to: {}", url);

        true
    }

    pub fn disable(&self, duration: Duration) -> bool {
        let params = format!("?disable={}&auth={}", duration.as_secs(), self.token);
        let url = format!("{}API_PATH{}", self.base_url, params);

        log::info!("Sending request to: {}", url);

        true // TODO: parse response and return accordingly
    }
}
