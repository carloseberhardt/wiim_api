use wiim_api::{Result, WiimClient};

#[tokio::main]
async fn main() -> Result<()> {
    let client = WiimClient::connect("192.168.86.52").await?;
    println!("✅ Connected to WiiM device!");

    // Get detailed now playing info
    println!("\n🎵 Current Track Details:");
    let now_playing = client.get_now_playing().await?;

    println!("Title: {:?}", now_playing.title);
    println!("Artist: {:?}", now_playing.artist);
    println!("Album: {:?}", now_playing.album);
    println!("Album Art URL: {:?}", now_playing.album_art_uri);
    println!("Sample Rate: {:?}", now_playing.sample_rate);
    println!("Bit Depth: {:?}", now_playing.bit_depth);
    println!("Position: {}ms", now_playing.position_ms);
    println!("Duration: {}ms", now_playing.duration_ms);
    println!("State: {}", now_playing.state);
    println!("Volume: {}", now_playing.volume);
    println!("Muted: {}", now_playing.is_muted);

    // Also test raw API responses to see all available data
    println!("\n📊 Raw Player Status:");
    let status = client.get_player_status().await?;
    println!("{:#?}", status);

    println!("\n🎨 Raw Metadata:");
    let meta = client.get_meta_info().await?;
    println!("{:#?}", meta);

    Ok(())
}
