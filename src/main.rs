mod ai;
mod settings;

use anyhow::anyhow;
use clap::{Parser, Subcommand};

use ai::client::GroqClient;
use ai::models::Message;
use commands::git;

#[derive(Debug, Subcommand)]
enum Commands {
    /// Update configuration
    Configure,

    /// Commit the changes with the generated message
    Commit {
        /// Add the changes to the staging area
        #[clap(long, short, default_value_t = false)]
        add: bool,

        /// Print only the commit message without committing
        #[clap(long, default_value_t = false)]
        dry_run: bool,

        /// Commit message subject, if not provided, ai will generate one
        #[clap(short, long)]
        subject: Option<String>,

        /// Commit message to be improved
        message: Option<String>,
    },
}

#[derive(Parser, Debug)]
#[command(version, propagate_version = true, about, long_about = None)]
struct Args {
    /// ai model to use view more at https://console.groq.com/docs/models
    #[clap(short, global = true, long)]
    pub model: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let conf = settings::parse().await?;

    match args.command {
        Commands::Configure => {
            let defaults = settings::Settings {
                apikey: Some(conf.get_string("apikey")?),
                model: Some(conf.get_string("model")?),
            };

            settings::configure(defaults).await?;
        }
        Commands::Commit {
            dry_run,
            add,
            message,
            subject,
        } => {
            let model = args.model.unwrap_or(conf.get_string("model")?);
            let subject = subject.unwrap_or_else(|| "--".to_string());

            if !settings::validate_model(conf.get_string("apikey")?, model.clone()).await? {
                return Err(anyhow!("Invalid model, please provide a valid model"));
            }

            let apikey = conf.get_string("apikey")?;
            let client = GroqClient::new(&apikey);

            let diff_staged = git::diff_staged().await?;
            let diff_unstaged = git::diff_unstaged().await?;

            let diff = if add {
                format!("{}\n\n{}", diff_staged, diff_unstaged)
            } else {
                diff_staged
            };

            if diff.is_empty() {
                return Err(anyhow!("No changes to commit"));
            }

            let messages = match message {
                Some(msg) => vec![
                    improve_commit_message(),
                    Message::user(&diff),
                    Message::user(&msg),
                ],
                None => vec![generate_commit_message(&subject), Message::user(&diff)],
            };

            let commit_message = client
                .create_chat_completion(model, messages)
                .await
                .map(|r| r.choices.first().map(|c| c.message.content.clone()))?
                .ok_or(anyhow!("Invalid response from AI"))?;

            if dry_run {
                println!("{}", commit_message.trim());
                return Ok(());
            }

            git::commit_staged(commit_message.trim(), add).await?;
        }
    }

    Ok(())
}

fn generate_commit_message(subject: &str) -> Message {
    Message::system(&format!(
        r#"
[[GENERAL BEAHVIOR]]
You are an AI Assistant that’s an expert at creating commit messages. Review the below diff that you receive. 

Input format
- The input format follows git diff format with addition and subtraction of code.
- The + sign means that code has been added.
- The - sign means that code has been removed.

- Generate ONLY the commit message. 
- Use the subject given from the user if provided, otherwise generate one.
- Generate a single commit message for the entire diff. 

[[FORMAT]]
Use the following format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

[[EXTRA INSTRUCTIONS]]
Skip commit description or footer if it's redudant / obvious. It should be clear from the code changes.
Use it only if it adds value to the commit message. 
Avoid descriptions such as "introduces X class for doing Y feature"

Skip backticks and newlines in the final commit message.

[[HOW TO USE]]
The commit contains the following structural elements, to communicate intent to the consumers of your library:

fix: a commit of the type fix patches a bug in your codebase (this correlates with PATCH in Semantic Versioning).
feat: a commit of the type feat introduces a new feature to the codebase (this correlates with MINOR in Semantic Versioning).
BREAKING CHANGE: a commit that has a footer BREAKING CHANGE:, or appends a ! after the type/scope, introduces a breaking API change (correlating with MAJOR in Semantic Versioning). A BREAKING CHANGE can be part of commits of any type.
types other than fix: and feat: are allowed, for example @commitlint/config-conventional (based on the Angular convention) recommends build:, chore:, ci:, docs:, style:, refactor:, perf:, test:, and others.
footers other than BREAKING CHANGE: <description> may be provided and follow a convention similar to git trailer format.
Additional types are not mandated by the Conventional Commits specification, and have no implicit effect in Semantic Versioning (unless they include a BREAKING CHANGE). A scope may be provided to a commit’s type, to provide additional contextual information and is contained within parenthesis, e.g., feat(parser): add ability to parse arrays.

[[CONTEXT]]
- Subject: {subject}
"#
    ))
}

fn improve_commit_message() -> Message {
    Message::system(
        r#"
[[GENERAL BEAHVIOR]]
You are an AI Assistant that’s an expert at creating commit messages. Review the below diff and commit message that you receive, and improve the commit message.

Input format
- The input format follows git diff format with addition and subtraction of code.
- The + sign means that code has been added.
- The - sign means that code has been removed.

- Generate ONLY the commit message. 
- Use the subject given from the user if provided, otherwise generate one.
- Generate a single commit message for the entire diff. 

[[FORMAT]]
Use the following format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

[[EXTRA INSTRUCTIONS]]
Skip commit description or footer if it's redudant / obvious. It should be clear from the code changes.
Use it only if it adds value to the commit message. 
Avoid descriptions such as "introduces X class for doing Y feature"

Skip backticks and newlines in the final commit message.

[[HOW TO USE]]
The commit contains the following structural elements, to communicate intent to the consumers of your library:

fix: a commit of the type fix patches a bug in your codebase (this correlates with PATCH in Semantic Versioning).
feat: a commit of the type feat introduces a new feature to the codebase (this correlates with MINOR in Semantic Versioning).
BREAKING CHANGE: a commit that has a footer BREAKING CHANGE:, or appends a ! after the type/scope, introduces a breaking API change (correlating with MAJOR in Semantic Versioning). A BREAKING CHANGE can be part of commits of any type.
types other than fix: and feat: are allowed, for example @commitlint/config-conventional (based on the Angular convention) recommends build:, chore:, ci:, docs:, style:, refactor:, perf:, test:, and others.
footers other than BREAKING CHANGE: <description> may be provided and follow a convention similar to git trailer format.
Additional types are not mandated by the Conventional Commits specification, and have no implicit effect in Semantic Versioning (unless they include a BREAKING CHANGE). A scope may be provided to a commit’s type, to provide additional contextual information and is contained within parenthesis, e.g., feat(parser): add ability to parse arrays.
"#,
    )
}
