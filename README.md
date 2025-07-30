# iOS Fonts Configurator

A command-line tool written in Rust that generates `.mobileconfig` files for iOS font installation. Provides a programmatic alternative to Apple's Configurator app.

## Features

- Generate iOS configuration profiles for font installation
- Support for individual font files and directories
- Automatic font detection (TTF, OTF, WOFF, WOFF2)
- Base64 encoding and proper XML structure generation
- UUID management for profile payloads

## Installation

### Prerequisites

- Rust 1.70+ and Cargo

### Build from source

```bash
git clone https://github.com/erning/ios-fonts-configurator.git
cd ios-fonts-configurator
cargo build --release
```

The binary will be available at `target/release/ios-fonts-configurator`.

## Usage

### Basic Usage

```bash
ios-fonts-configurator \
  --output output.mobileconfig \
  --name "Custom Font Profile" \
  --identifier com.example.fonts \
  --fonts font1.ttf font2.ttf
```

### Directory Support

You can specify directories containing fonts:

```bash
ios-fonts-configurator \
  --output output.mobileconfig \
  --name "My Font Collection" \
  --identifier com.company.fonts \
  --fonts /path/to/fonts/
```

### Mixed Usage

Combine individual files and directories:

```bash
ios-fonts-configurator \
  --output output.mobileconfig \
  --name "Mixed Fonts" \
  --identifier com.example.mixed \
  --fonts font1.ttf /path/to/fonts/ font3.otf
```

## Command Line Options

| Option | Short | Description | Required |
|--------|-------|-------------|----------|
| `--output` | `-o` | Output `.mobileconfig` file path | Yes |
| `--name` | `-n` | Display name for the font profile | Yes |
| `--identifier` | `-i` | Unique identifier (e.g., com.example.fonts) | Yes |
| `--fonts` | `-f` | Font files or directories containing fonts | Yes |

## Supported Font Formats

- **TTF** (TrueType Font)
- **OTF** (OpenType Font)
- **WOFF** (Web Open Font Format)
- **WOFF2** (Web Open Font Format 2.0)

## Installation on iOS

1. Transfer the generated `.mobileconfig` file to your iOS device
2. Open the file in Safari or Mail
3. Follow the on-screen prompts to install the configuration profile
4. Go to **Settings** -> **General** -> **VPN & Device Management**
5. Find the profile and install it
6. The fonts will be available system-wide

## Development

### Build

```bash
cargo build
```

### Run in development mode

```bash
cargo run -- --output test.mobileconfig --name "Test" --identifier com.test --fonts fonts/
```

### Run tests

```bash
cargo test
```

### Lint and format

```bash
cargo clippy
cargo fmt
```

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Technical Details

The tool generates Apple Property List (`.mobileconfig`) XML files with:
- Profile metadata (display name, identifier, UUID, version)
- Font payloads with Base64-encoded data
- Proper XML structure for iOS configuration profile installation

For more information about iOS font installation methods, see the [documentation](ios-font-installation-guide.md).
