use serde::Deserialize;

pub const CONFIG_FILE: &str = "config.json";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub blacklist_file: String,
    pub team: String,
    pub authorization_token: String,
}

pub fn read_config_file() -> anyhow::Result<Config> {
    let config_contents = &std::fs::read_to_string(CONFIG_FILE)?;
    let config = serde_json::from_str(config_contents)?;
    Ok(config)
}
