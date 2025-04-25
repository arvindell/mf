# Contributing to MF

Thanks for considering contributing to MF! This document explains how to get started.

## Development Setup

1. Clone the repository:
```bash
git clone https://github.com/yourusername/mf.git
cd mf
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Run `cargo fmt` and `cargo clippy` to ensure code quality
5. Commit your changes (`git commit -am 'Add some feature'`)
6. Push to the branch (`git push origin feature/my-feature`)
7. Open a Pull Request

## Code Style

We follow Rust's standard code style. Please run:

```bash
cargo fmt
cargo clippy
```

before submitting your PR.

## Adding New Messages

If you want to add new success or failure messages:

1. Keep them concise and entertaining
2. Make sure they're appropriate for the tone of the project (colorful language is fine)
3. Add them to the appropriate array in `src/main.rs`

## Platform Support

When adding or modifying features, please consider all three platforms (macOS, Linux, Windows) and test your changes if possible.

## License

By contributing to MF, you agree that your contributions will be licensed under the project's MIT license. 