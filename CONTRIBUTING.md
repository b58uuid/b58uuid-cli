# Contributing to B58UUID CLI

Thank you for your interest in contributing to B58UUID CLI!

## Development Setup

### Prerequisites
- Rust 1.70 or later
- Cargo

### Build from Source

```bash
# Clone the repository
git clone https://github.com/b58uuid/b58uuid-cli.git
cd b58uuid-cli

# Build
cargo build --release

# Run tests
cargo test

# Run the binary
./target/release/b58uuid --version
```

## Code Quality

Before submitting a pull request, ensure:

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings

# Run tests
cargo test
```

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and quality checks
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to your fork (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Write clear, descriptive commit messages
- Add tests for new functionality
- Update documentation as needed

## Reporting Issues

- Use GitHub Issues
- Provide clear description and reproduction steps
- Include system information (OS, Rust version)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

Feel free to open an issue for any questions or discussions.

Thank you for contributing! 🚀
