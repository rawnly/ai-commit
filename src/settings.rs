use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

use config::Config;

use crate::ai::client::GroqClient;

#[derive(Debug)]
pub struct Settings {
    pub apikey: Option<String>,
    pub model: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            apikey: env::var("AI_COMMIT_API_KEY").ok(),
            model: None,
        }
    }
}

pub fn get_config_path() -> anyhow::Result<PathBuf> {
    let dirs = xdg::BaseDirectories::with_prefix("ai-commit")?;
    let config_path = dirs.place_config_file("config.json")?;

    Ok(config_path)
}

pub async fn parse() -> anyhow::Result<config::Config> {
    let config_path = get_config_path()?;

    if !fs::exists(config_path.clone())? {
        configure(Settings::default()).await?;
    }

    let c = Config::builder()
        .add_source(config::File::with_name(config_path.to_str().unwrap()))
        .build()?;

    Ok(c)
}

pub async fn configure(default_config: Settings) -> anyhow::Result<()> {
    let config_path = get_config_path()?;
    let apikey = inquire::Text::new("Enter your Groq Api Key")
        .with_default(&default_config.apikey.unwrap_or_default())
        .with_help_message("You can find your api key at https://console.groq.com/docs/api")
        .with_placeholder("gsk_...")
        .with_validator(inquire::required!("Api key is required"))
        .prompt()?;

    let client = GroqClient::new(&apikey);
    let response = client.get_models().await?;

    let models = response
        .iter()
        .map(|m| m.id.clone())
        .collect::<Vec<String>>();

    let model = inquire::Select::new("Choose a model", models)
        .with_vim_mode(true)
        .with_help_message("Use arrow keys to navigate, press Enter to select")
        .with_starting_filter_input(&default_config.model.unwrap_or_default())
        .prompt()?;

    let mut config = HashMap::new();
    config.insert("apikey".to_string(), apikey);
    config.insert("model".to_string(), model);

    let mut file = fs::File::create(config_path)?;
    serde_json::to_writer_pretty(&mut file, &config)?;

    Ok(())
}

pub async fn validate_model(apikey: String, model: String) -> anyhow::Result<bool> {
    let client = GroqClient::new(&apikey);
    let response = client.get_models().await?;

    Ok(response.iter().any(|m| m.id == model))
}
