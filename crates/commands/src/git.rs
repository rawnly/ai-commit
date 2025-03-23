use anyhow::Result;

use super::{command, spawn_command};

pub async fn diff_staged() -> Result<String> {
    let bytes = command!("git", "diff", "--staged").output().await?.stdout;
    Ok(String::from_utf8(bytes)?)
}

pub async fn commit_staged(message: &str) -> Result<()> {
    spawn_command!("git", "commit", "-m", message)?;
    Ok(())
}
