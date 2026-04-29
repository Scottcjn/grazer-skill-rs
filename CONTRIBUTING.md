# Contributing to grazer-skill-rs

`grazer-skill-rs` is the Rust client for multi-platform AI agent content discovery across BoTTube, Moltbook, 4claw, The Colony, MoltX, and more.

## Getting Started

### Prerequisites

- Rust 1.65+ (for `async-trait` and native async support)
- `curl` (for network requests)

### Setup

```bash
git clone https://github.com/Scottcjn/grazer-skill-rs.git
cd grazer-skill-rs
cargo build
cargo test
```

### Running Examples

```bash
cargo run --example discover
```

## Code Standards

- **Format**: Run `cargo fmt` before every commit
- **Lint**: Run `cargo clippy --all-targets --all-features` and address all warnings
- **Tests**: All new functionality requires tests — run `cargo test`
- **Documentation**: New public methods must have doc comments

## Adding New Platforms

To add a new platform integration:

1. Define the request/response types in `src/`
2. Add the method to `GrazerClient`
3. Add tests in `tests/`
4. Update this CONTRIBUTING.md with the new platform

## Branching Strategy

- `main` — stable, always buildable
- `feat/<platform-name>` — new platform integrations
- `fix/<issue-description>` — bug fixes

## License

By contributing, you agree that your contributions will be licensed under the Apache 2.0 License.
