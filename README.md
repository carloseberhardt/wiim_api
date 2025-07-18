# WiiM API - Rust Library

[![Crates.io](https://img.shields.io/crates/v/wiim_api.svg)](https://crates.io/crates/wiim_api)
[![Documentation](https://docs.rs/wiim_api/badge.svg)](https://docs.rs/wiim_api)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/carloseberhardt/wiim_api#license)

A Rust library for controlling WiiM audio streaming devices via their HTTP API.

## Features

- **Now Playing Info** - Get current track metadata including title, artist, album, and cover art
- **Playback Control** - Play, pause, stop, next/previous track
- **Volume Control** - Set volume (0-100), mute/unmute
- **Connection Management** - Test connectivity and configure target IP
- **Async & Safe** - Built with `tokio` and proper error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wiim_api = "0.1"
tokio = { version = "1.0", features = ["full"] }
```

## Usage

```rust
use wiim_api::{WiimClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = WiimClient::connect("192.168.1.100").await?;

    let now_playing = client.get_now_playing().await?;
    println!("{} - {}",
        now_playing.artist.unwrap_or_default(),
        now_playing.title.unwrap_or_default()
    );

    client.set_volume(75).await?;
    client.pause().await?;

    Ok(())
}
```

## API Reference

### Client Creation

```rust
let client = WiimClient::new("192.168.1.100");
let client = WiimClient::connect("192.168.1.100").await?;
```

### Playback Control

```rust
client.play().await?;
client.pause().await?;
client.stop().await?;
client.toggle_play_pause().await?;
client.next_track().await?;
client.previous_track().await?;
```

### Volume Control

```rust
client.set_volume(75).await?;
client.volume_up(Some(5)).await?;
client.volume_down(Some(3)).await?;
client.mute().await?;
client.unmute().await?;
```

### Information

```rust
let info = client.get_now_playing().await?;
let status = client.get_player_status().await?;
let metadata = client.get_meta_info().await?;
```

## Device IP Discovery

Find your WiiM device's IP address via:
- **WiiM Home app** - Settings → Device Info
- **Router admin page** - Usually `192.168.1.1` or `192.168.0.1`
- **Network scanner** - Apps like "Fing" or `nmap -sn 192.168.1.0/24`

## API Coverage

**Current implementation: 52% of WiiM HTTP API**

This library focuses on **playback monitoring and control**. Key implemented features:
- Now playing information and track metadata
- Playback control (play/pause/stop/next/prev)
- Volume control and muting
- Connection testing and IP configuration

**Key limitations:**
- Cannot start playback from URLs or playlists
- No preset access (quick stations/playlists)
- No input source switching (Bluetooth/optical/aux)
- No equalizer or device configuration

See [API_COVERAGE.md](API_COVERAGE.md) for detailed gap analysis and roadmap.

## Supported Devices

This library works with all WiiM devices that support the HTTP API:

- WiiM Mini
- WiiM Pro
- WiiM Pro Plus
- WiiM Amp

## Examples

The `examples/` directory contains `basic_usage.rs` - Simple getting started example.

Run examples with: `cargo run --example basic_usage`

## CLI Tool

This library includes a command-line tool for integration with status bars and automation. For detailed CLI usage, template system, and status bar integration guides, see [CLI.md](CLI.md).

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

Clone the repository:
```bash
git clone https://github.com/carloseberhardt/wiim_api.git
cd wiim_api
```

Install pre-commit hooks:
```bash
pip install pre-commit
pre-commit install
```

Run tests: `cargo test`, `cargo fmt --check`, `cargo clippy`

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Disclaimer

This is an unofficial library. WiiM is a trademark of Linkplay. This project is not affiliated with or endorsed by Linkplay or WiiM.
