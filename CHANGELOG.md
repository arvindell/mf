# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2023-05-30

### Added
- Initial release
- Basic command execution with text-to-speech feedback
- Cross-platform support (macOS, Linux, Windows)
- Random success and failure messages
- Command line interface using clap

## [0.3.2] - 2023-05-31

### Fixed
- Shell aliases now work correctly when used with the command
- Added support for detecting and using the user's actual shell (zsh, bash, fish)
- Improved handling of shell-specific configurations and initialization 