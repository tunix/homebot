use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub chat: Chat,
    pub ad_protection: AdProtection
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    pub id: i64
}

#[derive(Debug, Deserialize)]
pub struct AdProtection {
    pub base_url: String,
    pub token: String
}

pub fn read_config() -> Configuration {
    let config_dir = dotenv::var("APP_CONFIG").unwrap_or(String::from("config"));
    let fp = format!("{config_dir}/application.yaml");
    let file = File::open(fp).expect("Failed to read file");
    let configuration = serde_yaml::from_reader(file).expect("Failed to deserialize configuration");

    log::debug!("Read configuration: {:?}", configuration);

    configuration
}
