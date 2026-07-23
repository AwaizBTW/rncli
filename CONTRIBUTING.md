# Contributing to rncli

Thank you for your interest in contributing to rncli! This document provides guidelines and instructions for contributing.

## Code of Conduct

Please be respectful and constructive in all interactions.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/rncli.git`
3. Create a feature branch: `git checkout -b feature/your-feature`
4. Make your changes
5. Run tests: `cargo test --all`
6. Format code: `cargo fmt --all`
7. Lint: `cargo clippy --all`
8. Commit: `git commit -am 'Add your feature'`
9. Push: `git push origin feature/your-feature`
10. Submit a Pull Request

## Development Guidelines

- Write tests for new functionality
- Update documentation for API changes
- Follow Rust conventions and idioms
- Use meaningful commit messages
- Keep commits logical and focused

## Testing

All tests must pass before submitting a PR:

```bash
cargo test --all
cargo clippy --all -- -D warnings
cargo fmt --all -- --check
```

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.
