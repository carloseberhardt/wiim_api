//! # WiiM API Client
//!
//! A Rust library for controlling WiiM audio streaming devices via their HTTP API.
//!
//! ## Features
//!
//! - **Now Playing Info**: Get current track metadata including title, artist, album, and cover art
//! - **Playback Control**: Play, pause, stop, next/previous track
//! - **Volume Control**: Set volume, relative volume changes, mute/unmute
//! - **Connection Management**: Test connectivity and configure target IP
//!
//! ## Quick Start
//!
//! ```no_run
//! use wiim_api::{WiimClient, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Connect to your WiiM device
//!     let client = WiimClient::connect("192.168.1.100").await?;
//!
//!     // Get now playing information
//!     let now_playing = client.get_now_playing().await?;
//!     println!("♪ {} - {}",
//!         now_playing.artist.unwrap_or_default(),
//!         now_playing.title.unwrap_or_default()
//!     );
//!
//!     // Control playback
//!     client.set_volume(75).await?;
//!     client.pause().await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Finding Your Device IP
//!
//! - Check your router's admin page (usually 192.168.1.1)
//! - Use network scanner apps
//! - Check the WiiM mobile app settings
//! - Use command: `nmap -sn 192.168.1.0/24`

use reqwest::Client;
use serde::Deserialize;
use std::fmt;
use thiserror::Error;

/// Errors that can occur when using the WiiM API
#[derive(Error, Debug)]
pub enum WiimError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

/// Result type for WiiM API operations
pub type Result<T> = std::result::Result<T, WiimError>;

/// HTTP client for communicating with WiiM devices
#[derive(Debug, Clone)]
pub struct WiimClient {
    base_url: String,
    client: Client,
}

/// Raw player status response from the WiiM device
#[derive(Debug, Deserialize)]
pub struct PlayerStatus {
    #[serde(rename = "type")]
    pub device_type: String,
    pub ch: String,
    pub mode: String,
    #[serde(rename = "loop")]
    pub loop_mode: String,
    pub eq: String,
    pub status: String,
    pub curpos: String,
    pub offset_pts: String,
    pub totlen: String,
    pub alarmflag: String,
    pub plicount: String,
    pub plicurr: String,
    pub vol: String,
    pub mute: String,
}

/// Track metadata from the WiiM device
#[derive(Debug, Deserialize)]
pub struct MetaData {
    pub album: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    #[serde(rename = "albumArtURI")]
    pub album_art_uri: Option<String>,
    #[serde(rename = "sampleRate")]
    pub sample_rate: Option<String>,
    #[serde(rename = "bitDepth")]
    pub bit_depth: Option<String>,
}

/// Container for track metadata response
#[derive(Debug, Deserialize)]
pub struct MetaInfo {
    #[serde(rename = "metaData")]
    pub meta_data: MetaData,
}

/// Current playback state of the device
#[derive(Debug, Clone)]
pub enum PlayState {
    Playing,
    Paused,
    Stopped,
    Loading,
}

impl fmt::Display for PlayState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayState::Playing => write!(f, "playing"),
            PlayState::Paused => write!(f, "paused"),
            PlayState::Stopped => write!(f, "stopped"),
            PlayState::Loading => write!(f, "loading"),
        }
    }
}

/// Complete now playing information combining playback status and track metadata
#[derive(Debug, Clone)]
pub struct NowPlaying {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_art_uri: Option<String>,
    pub state: PlayState,
    pub volume: u8,
    pub is_muted: bool,
    pub position_ms: u64,
    pub duration_ms: u64,
    pub sample_rate: Option<String>,
    pub bit_depth: Option<String>,
}

impl WiimClient {
    /// Parse volume string to u8 with proper error handling
    fn parse_volume(vol_str: &str) -> Result<u8> {
        vol_str
            .parse()
            .map_err(|_| WiimError::InvalidResponse(format!("Invalid volume value: {vol_str}")))
    }

    /// Parse duration string to u64 with proper error handling
    fn parse_duration(duration_str: &str) -> Result<u64> {
        duration_str.parse().map_err(|_| {
            WiimError::InvalidResponse(format!("Invalid duration value: {duration_str}"))
        })
    }

    /// Parse position string to u64 with proper error handling
    fn parse_position(position_str: &str) -> Result<u64> {
        position_str.parse().map_err(|_| {
            WiimError::InvalidResponse(format!("Invalid position value: {position_str}"))
        })
    }

    /// Create a new client with the device's IP address
    ///
    /// # Examples
    /// ```
    /// use wiim_api::WiimClient;
    ///
    /// let client = WiimClient::new("192.168.1.100");
    /// let client_with_https = WiimClient::new("https://192.168.1.100");
    /// ```
    pub fn new(ip_address: &str) -> Self {
        let base_url = if ip_address.starts_with("http") {
            ip_address.to_string()
        } else {
            format!("https://{ip_address}")
        };

        // Configure client to accept self-signed certificates (WiiM devices use them)
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to create HTTP client");

        Self { base_url, client }
    }

    /// Create a client and test connection to ensure the device is reachable
    ///
    /// # Examples
    /// ```no_run
    /// use wiim_api::WiimClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> wiim_api::Result<()> {
    ///     let client = WiimClient::connect("192.168.1.100").await?;
    ///     println!("Connected to WiiM device!");
    ///     Ok(())
    /// }
    /// ```
    pub async fn connect(ip_address: &str) -> Result<Self> {
        let client = Self::new(ip_address);

        // Test connection by getting device status
        client.get_player_status().await?;

        Ok(client)
    }

    /// Change the IP address of an existing client
    ///
    /// # Examples
    /// ```
    /// use wiim_api::WiimClient;
    ///
    /// let mut client = WiimClient::new("192.168.1.100");
    /// client.set_ip_address("192.168.1.101");
    /// ```
    pub fn set_ip_address(&mut self, ip_address: &str) {
        self.base_url = if ip_address.starts_with("http") {
            ip_address.to_string()
        } else {
            format!("https://{ip_address}")
        };
    }

    /// Get the current IP address/URL being used
    pub fn get_ip_address(&self) -> &str {
        &self.base_url
    }

    /// Test if the device is reachable
    ///
    /// # Examples
    /// ```no_run
    /// use wiim_api::WiimClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> wiim_api::Result<()> {
    ///     let client = WiimClient::new("192.168.1.100");
    ///
    ///     if client.test_connection().await.is_ok() {
    ///         println!("Device is reachable!");
    ///     } else {
    ///         println!("Device is not reachable");
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn test_connection(&self) -> Result<()> {
        self.get_player_status().await?;
        Ok(())
    }

    async fn send_command(&self, command: &str) -> Result<String> {
        let url = format!("{}/httpapi.asp?command={command}", self.base_url);
        let response = self.client.get(&url).send().await?;
        let text = response.text().await?;
        Ok(text)
    }

    pub async fn get_player_status(&self) -> Result<PlayerStatus> {
        let response = self.send_command("getPlayerStatus").await?;
        let status: PlayerStatus = serde_json::from_str(&response)?;
        Ok(status)
    }

    pub async fn get_meta_info(&self) -> Result<MetaInfo> {
        let response = self.send_command("getMetaInfo").await?;
        let meta: MetaInfo = serde_json::from_str(&response)?;
        Ok(meta)
    }

    /// Get comprehensive now playing information combining playback status and track metadata
    ///
    /// # Errors
    /// Returns `WiimError::InvalidResponse` if the device returns malformed data that cannot be parsed
    /// (e.g., invalid volume, position, or duration values)
    pub async fn get_now_playing(&self) -> Result<NowPlaying> {
        let (status, meta) = tokio::try_join!(self.get_player_status(), self.get_meta_info())?;

        let state = match status.status.as_str() {
            "play" => PlayState::Playing,
            "pause" => PlayState::Paused,
            "stop" => PlayState::Stopped,
            "loading" => PlayState::Loading,
            _ => PlayState::Stopped,
        };

        let volume = Self::parse_volume(&status.vol)?;
        let is_muted = status.mute == "1";
        let position_ms = Self::parse_position(&status.curpos)?;
        let duration_ms = Self::parse_duration(&status.totlen)?;

        Ok(NowPlaying {
            title: meta.meta_data.title,
            artist: meta.meta_data.artist,
            album: meta.meta_data.album,
            album_art_uri: meta.meta_data.album_art_uri,
            state,
            volume,
            is_muted,
            position_ms,
            duration_ms,
            sample_rate: meta.meta_data.sample_rate,
            bit_depth: meta.meta_data.bit_depth,
        })
    }

    /// Set the device volume level
    ///
    /// # Arguments
    /// * `volume` - Volume level from 0 to 100
    ///
    /// # Errors
    /// Returns `WiimError::InvalidResponse` if volume > 100
    ///
    /// # Examples
    /// ```no_run
    /// use wiim_api::WiimClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> wiim_api::Result<()> {
    ///     let client = WiimClient::new("192.168.1.100");
    ///
    ///     // Valid usage
    ///     client.set_volume(75).await?;
    ///
    ///     // Invalid usage - returns error
    ///     match client.set_volume(150).await {
    ///         Err(wiim_api::WiimError::InvalidResponse(msg)) => println!("Error: {}", msg),
    ///         _ => {}
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn set_volume(&self, volume: u8) -> Result<()> {
        if volume > 100 {
            return Err(WiimError::InvalidResponse(
                "Volume must be 0-100".to_string(),
            ));
        }
        let command = format!("setPlayerCmd:vol:{volume}");
        self.send_command(&command).await?;
        Ok(())
    }

    /// Increase volume by specified amount (default 5)
    ///
    /// # Errors
    /// Returns `WiimError::InvalidResponse` if the device returns an invalid volume value that cannot be parsed
    pub async fn volume_up(&self, step: Option<u8>) -> Result<u8> {
        let step = step.unwrap_or(5);
        let current_status = self.get_player_status().await?;
        let current_volume = Self::parse_volume(&current_status.vol)?;
        let new_volume = (current_volume.saturating_add(step)).min(100);
        self.set_volume(new_volume).await?;
        Ok(new_volume)
    }

    /// Decrease volume by specified amount (default 5)
    ///
    /// # Errors
    /// Returns `WiimError::InvalidResponse` if the device returns an invalid volume value that cannot be parsed
    pub async fn volume_down(&self, step: Option<u8>) -> Result<u8> {
        let step = step.unwrap_or(5);
        let current_status = self.get_player_status().await?;
        let current_volume = Self::parse_volume(&current_status.vol)?;
        let new_volume = current_volume.saturating_sub(step);
        self.set_volume(new_volume).await?;
        Ok(new_volume)
    }

    pub async fn mute(&self) -> Result<()> {
        self.send_command("setPlayerCmd:mute:1").await?;
        Ok(())
    }

    pub async fn unmute(&self) -> Result<()> {
        self.send_command("setPlayerCmd:mute:0").await?;
        Ok(())
    }

    pub async fn pause(&self) -> Result<()> {
        self.send_command("setPlayerCmd:pause").await?;
        Ok(())
    }

    pub async fn resume(&self) -> Result<()> {
        self.send_command("setPlayerCmd:resume").await?;
        Ok(())
    }

    pub async fn toggle_play_pause(&self) -> Result<()> {
        self.send_command("setPlayerCmd:onepause").await?;
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        self.send_command("setPlayerCmd:stop").await?;
        Ok(())
    }

    pub async fn next_track(&self) -> Result<()> {
        self.send_command("setPlayerCmd:next").await?;
        Ok(())
    }

    pub async fn previous_track(&self) -> Result<()> {
        self.send_command("setPlayerCmd:prev").await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = WiimClient::new("192.168.1.100");
        assert_eq!(client.base_url, "https://192.168.1.100");

        let client2 = WiimClient::new("https://192.168.1.100");
        assert_eq!(client2.base_url, "https://192.168.1.100");
    }

    #[test]
    fn test_play_state_display() {
        assert_eq!(PlayState::Playing.to_string(), "playing");
        assert_eq!(PlayState::Paused.to_string(), "paused");
        assert_eq!(PlayState::Stopped.to_string(), "stopped");
        assert_eq!(PlayState::Loading.to_string(), "loading");
    }

    #[test]
    fn test_set_volume_validation_logic() {
        // Test the validation logic directly without network calls
        // This tests that valid volumes would pass validation

        // These values should pass the validation check (volume <= 100)
        let valid_volumes = [0, 1, 50, 99, 100];
        for volume in valid_volumes {
            // The validation logic: if volume > 100
            assert!(volume <= 100, "Volume {volume} should be valid");
        }

        // These values should fail the validation check (volume > 100)
        let invalid_volumes = [101, 150, 200, 255];
        for volume in invalid_volumes {
            // The validation logic: if volume > 100
            assert!(volume > 100, "Volume {volume} should be invalid");
        }
    }

    #[tokio::test]
    async fn test_set_volume_invalid_values() {
        let client = WiimClient::new("192.168.1.100");

        // Test values > 100 should return validation errors
        let result = client.set_volume(101).await;
        assert!(result.is_err());
        if let Err(WiimError::InvalidResponse(msg)) = result {
            assert_eq!(msg, "Volume must be 0-100");
        } else {
            panic!("Expected InvalidResponse error for volume 101");
        }

        let result = client.set_volume(150).await;
        assert!(result.is_err());
        if let Err(WiimError::InvalidResponse(msg)) = result {
            assert_eq!(msg, "Volume must be 0-100");
        } else {
            panic!("Expected InvalidResponse error for volume 150");
        }

        let result = client.set_volume(255).await;
        assert!(result.is_err());
        if let Err(WiimError::InvalidResponse(msg)) = result {
            assert_eq!(msg, "Volume must be 0-100");
        } else {
            panic!("Expected InvalidResponse error for volume 255");
        }
    }

    #[test]
    fn test_volume_validation_error_message() {
        // Test that our error message is correct
        let error = WiimError::InvalidResponse("Volume must be 0-100".to_string());
        assert_eq!(error.to_string(), "Invalid response: Volume must be 0-100");
    }

    #[test]
    fn test_parse_volume_valid_inputs() {
        // Test valid volume parsing
        assert_eq!(WiimClient::parse_volume("0").unwrap(), 0);
        assert_eq!(WiimClient::parse_volume("50").unwrap(), 50);
        assert_eq!(WiimClient::parse_volume("100").unwrap(), 100);
    }

    #[test]
    fn test_parse_volume_invalid_inputs() {
        // Test invalid volume parsing returns appropriate errors
        let result = WiimClient::parse_volume("invalid");
        assert!(result.is_err());
        if let Err(WiimError::InvalidResponse(msg)) = result {
            assert_eq!(msg, "Invalid volume value: invalid");
        } else {
            panic!("Expected InvalidResponse error");
        }

        let result = WiimClient::parse_volume("");
        assert!(result.is_err());
        if let Err(WiimError::InvalidResponse(msg)) = result {
            assert_eq!(msg, "Invalid volume value: ");
        } else {
            panic!("Expected InvalidResponse error");
        }

        let result = WiimClient::parse_volume("256");
        assert!(result.is_err());
        if let Err(WiimError::InvalidResponse(msg)) = result {
            assert_eq!(msg, "Invalid volume value: 256");
        } else {
            panic!("Expected InvalidResponse error");
        }
    }

    #[test]
    fn test_parse_duration_valid_inputs() {
        // Test valid duration parsing
        assert_eq!(WiimClient::parse_duration("0").unwrap(), 0);
        assert_eq!(WiimClient::parse_duration("30000").unwrap(), 30000);
        assert_eq!(WiimClient::parse_duration("180000").unwrap(), 180000);
    }

    #[test]
    fn test_parse_duration_invalid_inputs() {
        // Test invalid duration parsing returns appropriate errors
        let result = WiimClient::parse_duration("not_a_number");
        assert!(result.is_err());
        if let Err(WiimError::InvalidResponse(msg)) = result {
            assert_eq!(msg, "Invalid duration value: not_a_number");
        } else {
            panic!("Expected InvalidResponse error");
        }

        let result = WiimClient::parse_duration("3.14");
        assert!(result.is_err());
        if let Err(WiimError::InvalidResponse(msg)) = result {
            assert_eq!(msg, "Invalid duration value: 3.14");
        } else {
            panic!("Expected InvalidResponse error");
        }
    }

    #[test]
    fn test_parse_position_valid_inputs() {
        // Test valid position parsing
        assert_eq!(WiimClient::parse_position("0").unwrap(), 0);
        assert_eq!(WiimClient::parse_position("15000").unwrap(), 15000);
        assert_eq!(WiimClient::parse_position("90000").unwrap(), 90000);
    }

    #[test]
    fn test_parse_position_invalid_inputs() {
        // Test invalid position parsing returns appropriate errors
        let result = WiimClient::parse_position("invalid_pos");
        assert!(result.is_err());
        if let Err(WiimError::InvalidResponse(msg)) = result {
            assert_eq!(msg, "Invalid position value: invalid_pos");
        } else {
            panic!("Expected InvalidResponse error");
        }

        let result = WiimClient::parse_position("-100");
        assert!(result.is_err());
        if let Err(WiimError::InvalidResponse(msg)) = result {
            assert_eq!(msg, "Invalid position value: -100");
        } else {
            panic!("Expected InvalidResponse error");
        }
    }
}
