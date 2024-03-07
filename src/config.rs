use clap::Parser;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use shellexpand;
use anyhow::anyhow;

#[derive(Parser)]
pub struct Args {
    #[clap(long, default_value="~/.config/jiscraper2.toml")]
    pub config: PathBuf,
    #[clap(long, short, action)]
    pub all_authors: bool
}

#[derive(Deserialize)]
pub struct Config {
    pub jira: JiraConfig
}

impl Config {
    pub fn read_from(path: &Path) -> Result<Self, anyhow::Error> {
        let str = shellexpand::tilde(path.to_str().unwrap());
        match fs_err::read_to_string(str.to_string()) {
            Ok(file) => {
                let config = toml::from_str(&file)?;
                Ok(config)
            }
            Err(e) => Err(anyhow!(r#"
Error reading config file {}.
Consider creating this file with the following content:
            
[jira]
host = "https://<your-jira-space>.atlassian.net/"
email = "<email-address>"
# Create an API Token here: https://id.atlassian.com/manage-profile/security/api-tokens
api_token = "<api-token>"

ERROR: {e}
            "#, str.to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct JiraConfig {
    pub host: String,
    pub email: String,
    pub api_token: String,
}