use wiim_api::{NowPlaying, PlayState, WiimClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example demonstrates how the template system works
    // You can configure templates in ~/.config/wiim-control/config.toml

    // Example configuration:
    // [output.text]
    // playing = "▶️ {artist} - {title} {quality_info}"
    // paused = "⏸️ {artist} - {title}"
    // stopped = "⏹️ No music"
    // loading = "⏳ Loading..."

    // [output.json]
    // text = "{artist} - {title}"
    // alt = "{state}"
    // tooltip = "{title}\nArtist: {artist}\nVolume: {volume}%"
    // class = "{state}"

    // Available template variables:
    // Track Information: {artist}, {title}, {album}, {album_art_uri}
    // Playback State: {state}, {volume}, {muted}, {position}, {duration}, {position_ms}, {duration_ms}
    // Audio Quality: {sample_rate}, {bit_depth}, {sample_rate_khz}, {bit_depth_bit}, {quality_info}
    // Formatted: {track_info}, {full_info}

    println!("Template system example:");
    println!("Run: cargo run --bin wiim-control status");
    println!("With templates configured in ~/.config/wiim-control/config.toml");
    println!("See examples/template_config.toml for configuration examples");

    Ok(())
}
