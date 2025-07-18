# WiiM API - Rust Library

[![Crates.io](https://img.shields.io/crates/v/wiim_api.svg)](https://crates.io/crates/wiim_api)
[![Documentation](https://docs.rs/wiim_api/badge.svg)](https://docs.rs/wiim_api)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/your-username/wiim_api#license)

A Rust library for controlling WiiM audio streaming devices via their HTTP API.

## Features

- 🎵 **Now Playing Info** - Get current track metadata including title, artist, album, and cover art
- ⏯️ **Playback Control** - Play, pause, stop, next/previous track
- 🔊 **Volume Control** - Set volume (0-100), mute/unmute
- 🔗 **Connection Management** - Test connectivity and configure target IP
- 🛡️ **Async & Safe** - Built with `tokio` and proper error handling

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
wiim_api = "0.1"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Usage

```rust
use wiim_api::{WiimClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to your WiiM device
    let client = WiimClient::connect("192.168.1.100").await?;

    // Get now playing information
    let now_playing = client.get_now_playing().await?;
    println!("♪ {} - {}",
        now_playing.artist.unwrap_or_default(),
        now_playing.title.unwrap_or_default()
    );

    // Control playback
    client.set_volume(75).await?;
    client.pause().await?;

    Ok(())
}
```

### Advanced Usage

```rust
use wiim_api::{WiimClient, PlayState};

#[tokio::main]
async fn main() -> wiim_api::Result<()> {
    let client = WiimClient::new("192.168.1.100");

    // Test if device is reachable
    if client.test_connection().await.is_ok() {
        println!("Device is online!");

        // Get detailed track info
        let info = client.get_now_playing().await?;

        match info.state {
            PlayState::Playing => {
                println!("🎵 Now playing: {} - {}",
                    info.artist.unwrap_or("Unknown".to_string()),
                    info.title.unwrap_or("Unknown".to_string())
                );

                if let Some(cover_url) = info.album_art_uri {
                    println!("🎨 Album art: {}", cover_url);
                }

                println!("📊 {}kHz/{}bit • Volume: {}%",
                    info.sample_rate.unwrap_or_default(),
                    info.bit_depth.unwrap_or_default(),
                    info.volume
                );
            }
            PlayState::Paused => println!("⏸️ Playback paused"),
            PlayState::Stopped => println!("⏹️ Playback stopped"),
            PlayState::Loading => println!("⏳ Loading..."),
        }
    }

    Ok(())
}
```

## API Reference

### Client Creation

```rust
// Basic client (provide IP address)
let client = WiimClient::new("192.168.1.100");

// Connect with validation (tests connection)
let client = WiimClient::connect("192.168.1.100").await?;

// Change IP address later
client.set_ip_address("192.168.1.101");
```

### Playback Control

```rust
// Basic controls
client.play().await?;
client.pause().await?;
client.stop().await?;
client.toggle_play_pause().await?;

// Navigation
client.next_track().await?;
client.previous_track().await?;
```

### Volume Control

```rust
// Set volume (0-100)
client.set_volume(75).await?;

// Relative volume changes
let new_volume = client.volume_up(Some(5)).await?;    // +5, returns new level
let new_volume = client.volume_down(Some(3)).await?;  // -3, returns new level
let new_volume = client.volume_up(None).await?;       // +5 default step

// Mute/unmute
client.mute().await?;
client.unmute().await?;
```

### Information

```rust
// Complete now playing info (recommended)
let info = client.get_now_playing().await?;

// Raw API responses (advanced)
let status = client.get_player_status().await?;
let metadata = client.get_meta_info().await?;
```

## Finding Your Device IP

Your WiiM device's IP address can be found in several ways:

- **Router admin page** - Usually `192.168.1.1` or `192.168.0.1`
- **WiiM Home app** - Settings → Device Info
- **Network scanner** - Apps like "Fing" or "Advanced IP Scanner"
- **Command line** - `nmap -sn 192.168.1.0/24` (scan your network)

## API Coverage

**Current implementation: 52% of WiiM HTTP API**

This library focuses on **playback monitoring and control**. Key implemented features:
- ✅ Now playing information and track metadata
- ✅ Playback control (play/pause/stop/next/prev)
- ✅ Volume control and muting
- ✅ Connection testing and IP configuration

**Key limitations:**
- ❌ Cannot start playback from URLs or playlists
- ❌ No preset access (quick stations/playlists)
- ❌ No input source switching (Bluetooth/optical/aux)
- ❌ No equalizer or device configuration

See [API_COVERAGE.md](API_COVERAGE.md) for detailed gap analysis and roadmap.

## Supported Devices

This library should work with all WiiM devices that support the HTTP API:

- WiiM Mini
- WiiM Pro
- WiiM Pro Plus
- WiiM Amp

## Examples

The `examples/` directory contains:

- `basic_usage.rs` - Simple getting started example
- `test_device.rs` - Test connection and get device info
- `test_controls.rs` - Test all playback and volume controls
- `test_metadata.rs` - Explore all available metadata fields
- `waybar_config.json` - Example waybar configuration

Run examples with:
```bash
cargo run --example basic_usage
```

## CLI Tool

The library includes a `wiim-control` CLI tool for integration with status bars and automation:

```bash
# Install the CLI tool
cargo install --path . --bin wiim-control

# Basic commands
wiim-control status                    # Show current track (text format)
wiim-control --format json status     # JSON output for status bars
wiim-control toggle                    # Play/pause
wiim-control next                      # Next track
wiim-control volume 75                 # Set volume
wiim-control volume-up                 # Increase volume by 5
wiim-control volume-down 10            # Decrease volume by 10

# Configuration
~/.config/wiim-control/config.toml     # Auto-created config file
```

## Error Handling

The library uses a custom `Result<T>` type with `WiimError`:

```rust
match client.get_now_playing().await {
    Ok(info) => println!("Playing: {:?}", info.title),
    Err(wiim_api::WiimError::Request(_)) => println!("Network error"),
    Err(wiim_api::WiimError::Json(_)) => println!("Invalid response"),
    Err(e) => println!("Other error: {}", e),
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/wiim_api.git
   cd wiim_api
   ```

2. **Install pre-commit hooks:**
   ```bash
   # Install pre-commit (if not already installed)
   pip install pre-commit
   # or on Arch Linux: pacman -S python-pre-commit

   # Install hooks for this repository
   pre-commit install
   ```

3. **Make your changes and commit:**
   ```bash
   # Pre-commit hooks will automatically run:
   # - cargo fmt (code formatting)
   # - cargo clippy (linting)
   # - Basic file checks
   git commit -m "Your changes"
   ```

### Testing Guidelines

- **Prefer unit tests** that don't require actual WiiM devices
- **Use integration tests sparingly** - name them `test_*_integration`
- **Run tests:** `cargo test`
- **Check formatting:** `cargo fmt --check`
- **Run linter:** `cargo clippy`

The CI will automatically run these checks on your PR.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Disclaimer

This is an unofficial library. WiiM is a trademark of Linkplay. This project is not affiliated with or endorsed by Linkplay or WiiM.
