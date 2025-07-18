use std::time::Instant;
use wiim_api::{WiimClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = WiimClient::connect("192.168.86.52").await?;
    
    println!("Testing single client vs multiple clients...\n");
    
    // Test 1: Single client (connection pooling)
    let start = Instant::now();
    for i in 0..5 {
        let _ = client.get_player_status().await?;
        println!("Single client call {}: {:?}", i+1, start.elapsed());
    }
    let single_client_time = start.elapsed();
    
    println!("\n---\n");
    
    // Test 2: New client each time (no connection pooling)
    let start = Instant::now();
    for i in 0..5 {
        let new_client = WiimClient::new("192.168.86.52");
        let _ = new_client.get_player_status().await?;
        println!("New client call {}: {:?}", i+1, start.elapsed());
    }
    let new_client_time = start.elapsed();
    
    println!("\n=== RESULTS ===");
    println!("Single client (pooled): {:?}", single_client_time);
    println!("New clients each time: {:?}", new_client_time);
    println!("Speedup: {:.2}x faster", new_client_time.as_millis() as f64 / single_client_time.as_millis() as f64);
    
    Ok(())
}