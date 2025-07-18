use std::time::Instant;
use wiim_api::{Result, WiimClient};

#[tokio::test]
#[ignore] // Requires actual device
async fn test_volume_performance() -> Result<()> {
    let client = WiimClient::connect("192.168.86.52").await?;
    let original_volume = client.get_now_playing().await?.volume;

    println!("Testing volume operations performance...\n");

    // Test 1: Using library volume_up (single client, 2 calls)
    let start = Instant::now();
    let new_vol = client.volume_up(Some(1)).await?;
    let library_time = start.elapsed();
    println!("Library volume_up: {library_time:?} ({original_volume}% -> {new_vol}%)");

    // Restore volume
    client.set_volume(original_volume).await?;

    // Test 2: Manual get+set with same client
    let start = Instant::now();
    let status = client.get_player_status().await?;
    let current: u8 = status.vol.parse().map_err(|_| {
        wiim_api::WiimError::InvalidResponse(format!("Invalid volume: {vol}", vol = status.vol))
    })?;
    client.set_volume(current + 1).await?;
    let manual_same_client_time = start.elapsed();
    println!("Manual same client: {manual_same_client_time:?}");

    // Restore volume
    client.set_volume(original_volume).await?;

    // Test 3: Manual get+set with new clients each time
    let start = Instant::now();
    let client1 = WiimClient::new("192.168.86.52");
    let status = client1.get_player_status().await?;
    let current: u8 = status.vol.parse().map_err(|_| {
        wiim_api::WiimError::InvalidResponse(format!("Invalid volume: {vol}", vol = status.vol))
    })?;
    let client2 = WiimClient::new("192.168.86.52");
    client2.set_volume(current + 1).await?;
    let manual_new_clients_time = start.elapsed();
    println!("Manual new clients: {manual_new_clients_time:?}");

    // Restore volume
    client.set_volume(original_volume).await?;

    println!("\n=== SUMMARY ===");
    println!("Library volume_up(): {library_time:?}");
    println!("Manual same client: {manual_same_client_time:?}");
    println!("Manual new clients: {manual_new_clients_time:?}");

    // Performance assertions
    assert!(
        library_time.as_millis() < 5000,
        "Library volume_up should be reasonably fast"
    );
    assert!(
        manual_same_client_time.as_millis() < 5000,
        "Manual same client should be reasonably fast"
    );

    Ok(())
}
