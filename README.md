# Zap-rs

AppImage package manager inspired by [zap](https://github.com/srevinsaju/zap), but in Rust.

## Install

```bash
cargo install --git https://github.com/ndpm13/zap-rs
```

## Usage

```bash
# Install from URL
zap-rs install --from https://f.sed.lol/wow.AppImage wow 

# Remove
zap-rs rm neovim
```

Creates symlinks in `~/.local/bin` so you can just run the apps directly. Be sure to have it included in your `$PATH` env var.

## Development Status

This is me figuring out Rust and messing around with async stuff. Code changes a lot. If you want something stable or need major features, probably better to fork this or just use [zap](https://github.com/srevinsaju/zap).

## Acknowledgments

- [zap](https://github.com/srevinsaju/zap) - The original AppImage package manager that inspired this project
- The Rust community for excellent crates like `clap`, `tokio`, and `indicatif`

## License

MIT License - see LICENSE file for details.
