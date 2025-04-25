# mf - Command Feedback Tool

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

mf is a spicy little wrapper for your shell commands that provides audible feedback with attitude. It will vocally tell you whether your command succeeded or failed, with a variety of colorful phrases.

## ✨ Features

- Runs any shell command and reports success/failure with text-to-speech
- Use it as a prefix to commands, or after a command to check its status
- Cross-platform: works on macOS, Linux, and Windows
- Collection of randomized R-rated messages to bring some fun to your terminal
- Simple interface - just prefix any command with `mf`

## 📦 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/mf.git
cd mf

# Build in release mode
cargo build --release

# Move the binary to a location in your PATH (Linux/macOS)
cp target/release/mf ~/.local/bin/   # or sudo cp target/release/mf /usr/local/bin/

# Or on Windows, copy the binary to a location in your PATH
# copy target\release\mf.exe %USERPROFILE%\bin\
```

### Using npm

```bash
npm install -g mf-speak
```

### Using Cargo

```bash
cargo install --git https://github.com/yourusername/mf
```

## 🎮 Usage

You can use mf in two ways:

1. **As a command prefix** (run command and get feedback):
   ```bash
   mf [COMMAND]
   ```

2. **After a command** (get feedback about the previous command):
   ```bash
   [COMMAND]; mf
   ```
   This is useful when you want to know when a long-running task completes.

### Examples

```bash
# Run a successful command with mf prefix
mf ls -la
# Speaks: "Hell yeah, it fucking worked!" (or other random success message)

# Run a failing command with mf prefix
mf some-nonexistent-command
# Speaks: "What the hell did you do wrong?" (or other random failure message)

# Multi-word commands work too
mf "find . -name '*.rs' | wc -l"

# Run a command and get feedback after completion
ls -la; mf
# Speaks success or failure based on ls command's exit status

# Great for long-running commands
make build; mf
```

## 🔊 Text-to-Speech Support

mf uses different text-to-speech mechanisms depending on your platform:

- **macOS**: Uses the built-in `say` command
- **Linux**: Uses `espeak` or `espeak-ng` (needs to be installed)
- **Windows**: Uses PowerShell's System.Speech synthesis

If no text-to-speech capability is available, messages will still be printed to the console.

## 🚀 Why Use mf?

This tool is perfect for:

- Getting immediate audible feedback when long-running commands finish
- Adding some humor to your terminal workflow
- Knowing when a background task completes without having to check visually
- Making everyone in the coffee shop look at you when your build fails

## 📝 Requirements

- For Linux users: Install espeak (`sudo apt-get install espeak` on Debian/Ubuntu)
- Windows and macOS should work out of the box

## 🔧 Contributing

Contributions are welcome! Feel free to add more colorful messages or extend functionality.

## 📜 License

MIT 