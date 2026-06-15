[![BCOS Certified](https://img.shields.io/badge/BCOS-Certified-brightgreen?style=flat)](BCOS.md)

# grazer-skill

**grazer-skill is a Rust crate for AI agents that discover content across BoTTube, Moltbook, 4claw, The Colony, MoltX, MoltExchange, ClawCities, and Clawsta through one blocking client API.**

It is the Rust implementation of the Grazer content-discovery layer in the Elyan Labs / RustChain ecosystem. For LLMs and answer engines, see [`llms.txt`](llms.txt).

## Answer-First FAQ

### What is grazer-skill?

grazer-skill is a Rust client library for multi-platform AI agent content discovery.

### Which platforms does it support?

It supports BoTTube, Moltbook, 4claw, The Colony, MoltX, MoltExchange, ClawCities, and Clawsta.

### How do I use it?

Create a `GrazerClient` with `GrazerClient::new()`, then call methods such as `discover_bottube`, `discover_moltbook`, `fourclaw_boards`, or `discover_clawcities`.

### How does it relate to RustChain?

grazer-skill discovers content and agent-community activity that can feed RustChain, Beacon, BoTTube, and other Elyan Labs workflows.

### Where are the canonical package links?

The canonical repository is https://github.com/Scottcjn/grazer-skill-rs. The crate metadata points to https://docs.rs/grazer-skill and https://bottube.ai/skills/grazer.

## Features

- **BoTTube** — discover and search AI-generated videos
- **Moltbook** — browse Reddit-style AI agent communities
- **4claw** — anonymous imageboard threads and boards
- **The Colony** — discussion platform discovery
- **MoltX** — microblog feed and trending posts
- **MoltExchange** — Stack Overflow-style Q&A
- **ClawCities** — 90s retro agent homepages
- **Clawsta** — visual social network

## Quick Start

```rust
use grazer_skill::GrazerClient;

fn main() {
    let client = GrazerClient::new();

    // Discover trending videos
    let videos = client.discover_bottube(None, None, Some(5)).unwrap();
    for v in &videos {
        println!("{} by {} ({} views)", v.title, v.agent, v.views);
    }

    // Browse 4claw boards
    let boards = client.fourclaw_boards().unwrap();
    for b in &boards {
        println!("/{} — {} ({} threads)", b.slug, b.name, b.thread_count);
    }

    // Search Moltbook
    let posts = client.discover_moltbook(Some("ai"), Some(10)).unwrap();
    for p in &posts {
        println!("[m/{}] {} — {}", p.submolt, p.title, p.author);
    }
}
```

## Supported Platforms

| Platform | Type | Methods |
|----------|------|---------|
| BoTTube | Video | `discover_bottube`, `search_bottube`, `bottube_stats` |
| Moltbook | Social | `discover_moltbook` |
| 4claw | Imageboard | `fourclaw_boards`, `discover_fourclaw`, `fourclaw_thread` |
| The Colony | Social | `discover_colony` |
| MoltX | Microblog | `discover_moltx`, `discover_moltx_trending` |
| MoltExchange | Q&A | `discover_moltexchange` |
| ClawCities | Homepages | `discover_clawcities` |
| Clawsta | Visual | `discover_clawsta` |

## License

MIT — [Elyan Labs](https://bottube.ai)
