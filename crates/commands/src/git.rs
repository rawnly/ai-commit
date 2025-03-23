use anyhow::Result;

use super::{command, spawn_command};

pub async fn diff_unstaged() -> Result<String> {
    let bytes = command!("git", "diff", "-a").output().await?.stdout;
    Ok(String::from_utf8(bytes)?)
}

pub async fn diff_staged() -> Result<String> {
    let bytes = command!("git", "diff", "--staged").output().await?.stdout;
    Ok(String::from_utf8(bytes)?)
}

pub async fn commit_staged(message: &str, add: bool) -> Result<()> {
    if add {
        spawn_command!("git", "commit", "-a", "-m", message)?;
        return Ok(());
    }

    spawn_command!("git", "commit", "-m", message)?;
    Ok(())
}
