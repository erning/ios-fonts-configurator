# iOS Fonts Configurator

A Rust CLI tool that generates `.mobileconfig` files for installing custom fonts on iOS devices. No Apple Configurator needed.

## Quick start

```bash
# Install
git clone https://github.com/erning/ios-fonts-configurator.git
cd ios-fonts-configurator
cargo build --release

# Use
./target/release/ios-fonts-configurator \
  --output myfonts.mobileconfig \
  --name "My Fonts" \
  --identifier com.example.myfonts \
  --fonts font1.ttf font2.otf

# Install on iOS: AirDrop the .mobileconfig file to your device
```

## Usage

```bash
# Single font
ios-fonts-configurator \
    -o single.mobileconfig -n "Single Font" -i com.test.single \
    -f MyFont.ttf

# Directory scan
ios-fonts-configurator \
    -o pack.mobileconfig -n "Font Pack" -i com.user.fonts \
    -f ~/Library/Fonts/

# Mixed files and directories
ios-fonts-configurator \
    -o mixed.mobileconfig -n "Mixed" -i com.mixed.fonts \
    -f custom.ttf ~/fonts/ /usr/share/fonts/
```

## Example

```sh
nix develop --command fish
cargo run -- \
    -o LibertinusFonts.mobileconfig \
    -n "Fonts - Libertinus" \
    -i "com.erning.fonts.Libertinus" \
    -f ~/Library/Fonts/HomeManager/opentype/Libertinus*
```
Airdrop `LibertinusFonts.mobileconfig` to your iOS device and install it.

----

MIT License | [Full Documentation](ios-font-installation-guide.md)
