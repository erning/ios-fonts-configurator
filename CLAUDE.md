# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

The **iOS Fonts Configurator** is a Rust-based command-line tool that generates `.mobileconfig` files for iOS font installation. It provides a programmatic alternative to Apple's Configurator app, enabling users to create iOS configuration profiles that install custom fonts system-wide.

## Architecture

### Core Structure
- **`src/main.rs`**: CLI interface using clap for argument parsing, validates font files, and orchestrates the workflow
- **`src/mobileconfig.rs`**: Core business logic for `.mobileconfig` file generation, handles Base64 encoding, XML structure generation, and UUID management

### Key Components
- **MobileConfig struct**: Main configuration container with profile metadata and font payloads
- **FontPayload struct**: Represents individual font files with encoded data and metadata
- **CLI Arguments**: Output path, display name, identifier, and font files list

## Dependencies

### Core Dependencies
- `clap 4.4`: Command-line argument parsing with derive features
- `uuid 1.6`: UUID generation for unique payload identifiers
- `base64 0.21`: Base64 encoding of font file data
- `anyhow 1.0`: Comprehensive error handling
- `serde-xml-rs 0.6`: XML serialization (not currently used in core logic)

### Build System
- Uses Cargo with Rust edition 2021
- Nix development environment available via `flake.nix`
- Includes rust-analyzer, clippy, and rustfmt in development shell

## Generated Output Format

The tool generates Apple Property List (`.mobileconfig`) XML files with:
- **Profile Metadata**: Display name, identifier, UUID, and version
- **Font Payloads**: Base64-encoded font data with individual UUIDs
- **iOS Compliance**: Proper XML structure for iOS configuration profile installation

## Development Environment

### Build Commands
```bash
# Build the project
cargo build

# Run with release optimization
cargo build --release

# Run the CLI tool
cargo run -- --output profile.mobileconfig --name "My Fonts" --identifier com.example.fonts --fonts font1.ttf font2.otf
```

### CLI Usage
```bash
ios-fonts-configurator \
  --output output.mobileconfig \
  --name "Custom Font Profile" \
  --identifier com.company.fonts \
  --fonts font1.ttf font2.ttf font3.otf
```

## Project Context

### Documentation Reference
- `ios-font-installation-guide.md`: Contains comprehensive Chinese documentation about iOS font installation methods
- Explains both Apple Configurator and manual XML creation approaches
- Provides technical background for understanding the tool's purpose

### Current State
- Core functionality complete: XML generation, Base64 encoding, UUID generation
- CLI interface implemented with clap
- Build system configured with proper dependencies
- Ready for testing with actual font files

### Future Enhancements
- Profile signing support for trusted installation
- Batch processing capabilities
- Configuration file support for repeated use
- Validation of font file formats before processing

## Git Commit Guidelines

When creating git commits, always include the following co-authorship in the commit message:

```
Co-Authored-By: GLM-4.5 <noreply@z.ai>
```
