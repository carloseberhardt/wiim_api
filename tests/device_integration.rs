use wiim_api::{Result, WiimClient};

#[tokio::test]
#[ignore] // Requires actual device
async fn test_device_connection() -> Result<()> {
    println!("Testing connection to WiiM device at 192.168.86.52...");

    // Test connection first
    let client = WiimClient::connect("192.168.86.52").await?;
    println!("✅ Successfully connected to WiiM device!");

    // Get current status
    println!("\n📊 Getting player status...");
    let status = client.get_player_status().await?;
    println!("Status: {}", status.status);
    println!("Volume: {}", status.vol);
    println!("Muted: {}", if status.mute == "1" { "Yes" } else { "No" });

    // Get metadata
    println!("\n🎵 Getting now playing info...");
    let now_playing = client.get_now_playing().await?;
    println!("State: {}", now_playing.state);
    if let Some(title) = &now_playing.title {
        println!("Title: {title}");
    }
    if let Some(artist) = &now_playing.artist {
        println!("Artist: {artist}");
    }
    if let Some(album) = &now_playing.album {
        println!("Album: {album}");
    }
    println!("Volume: {}", now_playing.volume);

    println!("\n✅ Test completed!");
    Ok(())
}

#[tokio::test]
#[ignore] // Requires actual device
async fn test_volume_operations() -> Result<()> {
    let client = WiimClient::connect("192.168.86.52").await?;
    let original_volume = client.get_now_playing().await?.volume;

    // Test volume up
    let new_volume = client.volume_up(Some(5)).await?;
    assert!(new_volume >= original_volume);

    // Test volume down
    let restored_volume = client.volume_down(Some(5)).await?;
    assert!(restored_volume <= new_volume);

    // Restore original volume
    client.set_volume(original_volume).await?;

    Ok(())
}
