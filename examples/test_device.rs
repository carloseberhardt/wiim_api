use wiim_api::{WiimClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Testing connection to WiiM device at 192.168.86.52...");
    
    // Test connection first
    let client = match WiimClient::connect("192.168.86.52").await {
        Ok(client) => {
            println!("✅ Successfully connected to WiiM device!");
            client
        }
        Err(e) => {
            println!("❌ Failed to connect: {}", e);
            return Err(e);
        }
    };
    
    // Get current status
    println!("\n📊 Getting player status...");
    match client.get_player_status().await {
        Ok(status) => {
            println!("Status: {}", status.status);
            println!("Volume: {}", status.vol);
            println!("Muted: {}", if status.mute == "1" { "Yes" } else { "No" });
        }
        Err(e) => println!("Failed to get status: {}", e),
    }
    
    // Get metadata
    println!("\n🎵 Getting now playing info...");
    match client.get_now_playing().await {
        Ok(now_playing) => {
            println!("State: {}", now_playing.state);
            if let Some(title) = &now_playing.title {
                println!("Title: {}", title);
            }
            if let Some(artist) = &now_playing.artist {
                println!("Artist: {}", artist);
            }
            if let Some(album) = &now_playing.album {
                println!("Album: {}", album);
            }
            println!("Volume: {}", now_playing.volume);
        }
        Err(e) => println!("Failed to get now playing: {}", e),
    }
    
    println!("\n✅ Test completed!");
    Ok(())
}