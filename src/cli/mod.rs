use anyhow::Context;
use std::path::PathBuf;

pub mod auth_mgmt;
pub mod clap_config;
pub mod request_executor;

pub fn auth_config_file_path() -> anyhow::Result<PathBuf> {
    let auth_config_file_path: &str = ".curlaut/auth.yaml";
    let home_dir_path = std::env::home_dir().with_context(|| "Could not get home directory")?;
    Ok(home_dir_path.join(auth_config_file_path))
}
