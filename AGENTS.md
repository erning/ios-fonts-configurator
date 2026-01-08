# AGENTS.md

**Generated:** 2026-01-08 | **Commit:** ae38195 | **Branch:** master

## Overview

Rust CLI tool generating `.mobileconfig` files for iOS font installation. Replaces Apple Configurator for custom font profiles.

## Structure

```
ios-fonts-configurator/
├── src/
│   ├── main.rs           # CLI (clap), font collection, orchestration
│   └── mobileconfig.rs   # MobileConfig/FontPayload structs, XML generation
├── Cargo.toml            # Dependencies: clap 4.5, base64 0.21, anyhow 1.0
└── flake.nix             # Nix dev shell (rust-analyzer, clippy, rustfmt)
```

## Where to Look

| Task | Location | Notes |
|------|----------|-------|
| CLI args | `main.rs:65-109` | clap builder API (not derive) |
| Font collection | `main.rs:8-62` | Recursive dir scan, depth control |
| XML generation | `mobileconfig.rs:83-168` | Apple plist format |
| Add font types | `main.rs:24-27` | Extension matching |

## Code Map

| Symbol | Type | Location | Role |
|--------|------|----------|------|
| `MobileConfig` | struct | mobileconfig.rs:38 | Profile container |
| `FontPayload` | struct | mobileconfig.rs:30 | Single font data |
| `generate_xml` | method | mobileconfig.rs:83 | Plist XML output |
| `collect_font_files` | fn | main.rs:39 | Entry for font discovery |
| `generate_uuid` | fn | mobileconfig.rs:9 | Hash-based UUID |

## Conventions

- **Error handling**: `anyhow::Result` everywhere, no panics
- **Clap style**: Builder API (`Command`/`Arg`), not derive macros (despite feature enabled)
- **No tests**: Project has no test modules or tests/ directory

## Anti-Patterns

- No forbidden patterns documented in codebase
- Codebase is clean (no TODO/FIXME/HACK comments)

## Commands

```bash
# Dev environment (recommended)
nix develop

# Build
cargo build --release

# Run
cargo run -- -o out.mobileconfig -n "Fonts" -i com.example.fonts -f ~/fonts/

# Check
cargo clippy && cargo fmt --check
```

## Notes

- **Font formats**: TTF, OTF, WOFF, WOFF2 (see main.rs:24-27)
- **Max depth**: Default 3 for recursive dir scan
- **UUID generation**: Uses timestamp hash, not cryptographic (acceptable for profiles)
- **No CI**: Manual cargo build/clippy workflows
