use tokio::time::{Duration, sleep};
use wiim_api::{Result, WiimClient};

#[tokio::main]
async fn main() -> Result<()> {
    let client = WiimClient::connect("192.168.86.52").await?;
    println!("✅ Connected to WiiM device!");

    // Get current volume
    let initial = client.get_now_playing().await?;
    println!("Current volume: {}", initial.volume);

    // Test volume control
    println!("\n🔊 Testing volume control...");
    client.set_volume(30).await?;
    println!("Set volume to 30");

    sleep(Duration::from_secs(2)).await;

    client.set_volume(60).await?;
    println!("Set volume to 60");

    sleep(Duration::from_secs(2)).await;

    // Restore original volume
    client.set_volume(initial.volume).await?;
    println!("Restored volume to {}", initial.volume);

    // Test play/pause
    println!("\n⏯️  Testing play/pause...");
    client.pause().await?;
    println!("Paused playback");

    sleep(Duration::from_secs(3)).await;

    client.resume().await?;
    println!("Resumed playback");

    // Test mute
    println!("\n🔇 Testing mute...");
    client.mute().await?;
    println!("Muted");

    sleep(Duration::from_secs(2)).await;

    client.unmute().await?;
    println!("Unmuted");

    println!("\n✅ All controls working!");
    Ok(())
}
