# Contributing to grazer-skill-rs

Thank you for your interest in contributing to **grazer-skill-rs**! This Rust crate powers multi-platform AI agent content discovery across BoTTube, Moltbook, 4claw, MoltX, and more.

## Quick Start

```bash
# Clone and build
git clone https://github.com/Scottcjn/grazer-skill-rs.git
cd grazer-skill-rs
cargo build

# Run tests
cargo test

# Format and lint
cargo fmt
cargo clippy --all-targets --all-features
```

## Development Setup

### Prerequisites

- **Rust** (latest stable): Install via [rustup](https://rustup.rs/)
- **Cargo**: Included with Rust

### Fork and Clone

```bash
git clone https://github.com/YOUR_USERNAME/grazer-skill-rs.git
cd grazer-skill-rs
git remote add upstream https://github.com/Scottcjn/grazer-skill-rs.git
```

### Install Tools

```bash
rustup component add clippy rustfmt
```

## How to Contribute

### Reporting Bugs

Include:
- Clear description and steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version: `rustc --version`)
- Code example if applicable

### Suggesting Features

- Explain the use case
- Describe the proposed solution
- Consider API design implications

### Code Contributions

1. Find or create an issue
2. Fork and create a feature branch
3. Make changes following our standards
4. Test thoroughly
5. Submit a pull request

## Coding Standards

### Style Guide

Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

```rust
/// Fetches content from a specified platform.
///
/// # Arguments
///
/// * `platform` - Platform identifier (e.g., "bottube", "moltbook")
/// * `query` - Search query string
///
/// # Returns
///
/// Returns a `Result` containing content items or an error.
pub async fn fetch_content(
    platform: &str,
    query: &str,
) -> Result<Vec<Content>, GrazerError> {
    // Implementation
}
```

### Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting

```bash
# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

### Error Handling

Use `thiserror` for custom errors:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrazerError {
    #[error("Platform '{0}' not supported")]
    UnsupportedPlatform(String),
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}
```

## Testing

```bash
# Run all tests
cargo test --all-features

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch() {
        let result = fetch_content("bottube", "test");
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_async_fetch() {
        let content = fetch_content("bottube", "rust").await;
        assert!(content.is_ok());
    }
}
```

## Documentation

- All public items need doc comments (`///`)
- Include examples in docs
- Build docs: `cargo doc --open`

## Pull Request Process

1. Update documentation
2. Add tests for new code
3. Ensure all tests pass
4. Format: `cargo fmt`
5. Lint: `cargo clippy --all-targets --all-features`
6. Submit PR with clear description

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `test:` Tests
- `refactor:` Code refactoring
- `perf:` Performance
- `chore:` Maintenance

Example: `feat: Add MoltX platform support`

## Questions?

- Open an issue for discussion
- Check existing issues first

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

Thank you for contributing to grazer-skill-rs!
