use std::time::Duration;

const API_PATH: &'static str = "/admin/api.php";

pub struct AdProtection {
    host: String,
    token: String,
}

impl AdProtection {
    pub fn new(host: String, token: String) -> Self {
        Self { host, token }
    }

    pub fn is_enabled(&self) -> bool {
        let url = format!("{}{API_PATH}?summary", self.host);

        log::info!("Sending request to: {}", url);

        true
    }

    pub fn enable(&self) -> bool {
        let url = format!("{}{API_PATH}?summary", self.host);

        log::info!("Sending request to: {}", url);

        true // TODO: parse response and return accordingly
    }

    pub fn disable(&self, duration: Duration) -> bool {
        let params = format!("?disable={}&auth={}", duration.as_secs(), self.token);
        let url = format!("{}API_PATH{}", self.host, params);

        log::info!("Sending request to: {}", url);

        true // TODO: parse response and return accordingly
    }
}
