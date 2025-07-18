use wiim_api::{Result, WiimClient};

#[tokio::main]
async fn main() -> Result<()> {
    let client = WiimClient::new("192.168.1.100");

    println!("Getting now playing info...");
    let now_playing = client.get_now_playing().await?;
    println!("Now Playing: {now_playing:?}");

    println!("Setting volume to 50...");
    client.set_volume(50).await?;

    println!("Pausing playback...");
    client.pause().await?;

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    println!("Resuming playback...");
    client.resume().await?;

    Ok(())
}
