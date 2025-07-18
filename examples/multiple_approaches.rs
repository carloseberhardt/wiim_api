use wiim_api::{Result, WiimClient};

#[tokio::main]
async fn main() -> Result<()> {
    // Approach 1: Basic constructor (user provides IP)
    let _client = WiimClient::new("192.168.1.100");

    // Approach 2: Connect and test (validates IP works)
    let _client = WiimClient::connect("192.168.1.100").await?;
    println!("Successfully connected to WiiM device");

    // Approach 3: Change IP later if device moves
    let mut client = WiimClient::new("192.168.1.100");
    // ... later, if device IP changes ...
    client.set_ip_address("192.168.1.101");

    // Approach 4: Test if device is reachable
    let client = WiimClient::new("192.168.1.100");
    match client.test_connection().await {
        Ok(_) => {
            println!("Device is online and reachable");

            // Now use the client
            let now_playing = client.get_now_playing().await?;
            println!("Playing: {:?}", now_playing.title);
        }
        Err(e) => {
            println!("Device not reachable: {}", e);
            println!("Check your IP address or network connection");
        }
    }

    Ok(())
}
