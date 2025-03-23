use std::string::FromUtf8Error;

pub mod git;

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum CommandError {
    #[error("Failed to parse output: {0}")]
    ParseError(#[from] FromUtf8Error),

    #[error("failed to execute command: {0}")]
    IOError(String),

    #[error("failed to serialize command output: {0}")]
    SerializationError(String),
}

impl CommandError {
    pub fn from_io(e: std::io::Error) -> Self {
        CommandError::IOError(e.to_string())
    }

    pub fn from_serde(e: serde_json::Error) -> Self {
        CommandError::SerializationError(e.to_string())
    }
}

#[macro_export]
macro_rules! is_installed {
    ($command:expr) => {
        std::process::Command::new($command)
            .stderr(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .spawn()
            .is_ok()
    };
}

/// executes a shell command
#[macro_export]
macro_rules! command {
    ($command:expr) => (tokio::process::Command::new($command));

    ($command:expr, $($x:expr),+) => {{
        let mut args;

        #[allow(clippy::vec_init_then_push)]
        {
            args = Vec::new();

            $(
                args.push($x);
            )*
        }

        if cfg!(debug_assertions) {
            dbg!(&args);
        }

        tokio::process::Command::new($command)
        .args(&args.clone())
    }};
}

#[macro_export]
macro_rules! spawn_command {
    ($command:expr) => (tokio::process::Command::new($command).spawn());
    ($command:expr, $($args:expr),*) => {{
        let mut args;

        #[allow(clippy::vec_init_then_push)]
        {
            args = Vec::new();
            $( args.push($args); )*
            if cfg!(debug_assertions) {
                dbg!(&args);
            }
        }

        tokio::process::Command::new($command).args(&args.clone()).spawn()
    }}
}
