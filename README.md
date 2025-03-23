# AI Commit

AI Commit is a command-line tool that generates conventional commit messages using Groq's AI models. It analyzes your staged changes and creates meaningful, standardized commit messages following conventional commit format.

## Features

- 🤖 AI-powered commit message generation
- 🎯 Follows conventional commit message format
- ⚙️ Configurable AI model selection
- 🔑 Secure API key management
- 🔍 Dry run option to preview commit messages
- 📝 Support for custom commit message subjects

## Prerequisites

- Rust toolchain
- Git
- Groq API key (obtain from [Groq Console](https://console.groq.com/docs/api))

## Installation

### Homebrew
```bash
brew tap rawnly/tap
brew install ai-commit
```

### Build from source
```bash
git clone github.com/rawnly/ai-commit
cargo install --path .
```


## Configuration

On first run, the tool will prompt you to configure:
- Groq API key
- Preferred AI model

Configuration is stored in `~/.config/ai-commit/config.json` (XDG base directory standard).

You can also set your API key via environment variable:
```bash
export AI_COMMIT_API_KEY="your_api_key"
```

## Usage

### Generate and commit changes

```bash
ai-commit commit
```

### Preview commit message without committing (dry run)

```bash
ai-commit commit --dry-run
```

### Specify a custom commit message to be improved

```bash
ai-commit commit -s "feat: my custom subject"
```

### Use a different AI model

```bash
ai-commit commit --model "mixtral-8x7b-32768"
```

### Reconfigure settings

```bash
ai-commit configure
```

### Check version

```bash
ai-commit -v
```

## Commit Message Format

The 