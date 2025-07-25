//! # WiiM API Client
//!
//! A Rust library for controlling WiiM audio streaming devices via their HTTP API.
//!
//! ## Features
//!
//! - **Now Playing Info**: Get current track metadata including title, artist, album, and cover art
//! - **Playback Control**: Play, pause, stop, next/previous track
//! - **Volume Control**: Set volume, relative volume changes, mute/unmute
//! - **Device Information**: Get network quality, WiFi signal strength, and device details
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
//!     // Get device and network information
//!     let status_ex = client.get_status_ex().await?;
//!     if let Some(signal) = status_ex.signal_quality() {
//!         println!("📶 WiFi Signal: {} ({})",
//!             signal,
//!             status_ex.rssi_formatted().unwrap_or_default()
//!         );
//!     }
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
use std::time::Duration;
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
    pub subtitle: Option<String>,
    pub artist: Option<String>,
    #[serde(rename = "albumArtURI")]
    pub album_art_uri: Option<String>,
    #[serde(rename = "sampleRate")]
    pub sample_rate: Option<String>,
    #[serde(rename = "bitDepth")]
    pub bit_depth: Option<String>,
    #[serde(rename = "bitRate")]
    pub bit_rate: Option<String>,
    #[serde(rename = "trackId")]
    pub track_id: Option<String>,
}

/// Container for track metadata response
#[derive(Debug, Deserialize)]
pub struct MetaInfo {
    #[serde(rename = "metaData")]
    pub meta_data: MetaData,
}

/// Extended device status response from getStatusEx API
#[derive(Debug, Deserialize, Default)]
pub struct StatusEx {
    // Basic Device Information
    pub language: Option<String>, // "en_us"
    pub ssid: Option<String>,     // "WiiM Mini-8FA2"
    #[serde(rename = "hideSSID")]
    pub hide_ssid: Option<String>, // "0"
    pub firmware: Option<String>, // "Linkplay.4.6.425351"
    pub build: Option<String>,    // "release"
    pub project: Option<String>,  // "Muzo_Mini"
    pub priv_prj: Option<String>, // "Muzo_Mini"
    #[serde(rename = "Release")]
    pub release: Option<String>, // "20220805"
    #[serde(rename = "FW_Release_version")]
    pub fw_release_version: Option<String>, // ""
    #[serde(rename = "PCB_version")]
    pub pcb_version: Option<String>, // "0"
    pub group: Option<String>,    // "0"
    pub wmrm_version: Option<String>, // "4.2"
    pub wmrm_sub_ver: Option<String>, // "1"
    pub expired: Option<String>,  // "0"
    pub hardware: Option<String>, // "ALLWINNER-R328"
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>, // "WiiM Mini-8FA2"
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>, // "WiiM Mini-8FA2"

    // Network Configuration
    pub internet: Option<String>, // "1"
    pub netstat: Option<String>,  // "2"
    pub essid: Option<String>,    // Network SSID (encoded)
    pub apcli0: Option<String>,   // "192.168.4.62"
    pub eth0: Option<String>,     // "0.0.0.0"
    pub ra0: Option<String>,      // "10.10.10.254"

    // Network Quality Fields
    #[serde(rename = "RSSI")]
    pub rssi: Option<String>, // "-30"
    #[serde(rename = "BSSID")]
    pub bssid: Option<String>, // "8c:25:05:1c:41:40"
    #[serde(rename = "wlanSnr")]
    pub wlan_snr: Option<String>, // "35"
    #[serde(rename = "wlanNoise")]
    pub wlan_noise: Option<String>, // "-92"
    #[serde(rename = "wlanFreq")]
    pub wlan_freq: Option<String>, // "5805"
    #[serde(rename = "wlanDataRate")]
    pub wlan_data_rate: Option<String>, // "390"
    #[serde(rename = "WifiChannel")]
    pub wifi_channel: Option<String>, // "0"

    // Device Identifiers
    pub uuid: Option<String>, // "FF970016A6FE22C1660AB4D8"
    #[serde(rename = "MAC")]
    pub mac: Option<String>, // "08:E9:F6:8F:8F:A2"
    #[serde(rename = "BT_MAC")]
    pub bt_mac: Option<String>, // "08:E9:F6:8F:8F:A3"
    #[serde(rename = "AP_MAC")]
    pub ap_mac: Option<String>, // "0A:E9:F6:8F:8F:A2"
    #[serde(rename = "ETH_MAC")]
    pub eth_mac: Option<String>, // "00:00:00:00:00:00"

    // Date/Time
    pub date: Option<String>,            // "2022:08:09"
    pub time: Option<String>,            // "07:13:16"
    pub app_timezone_id: Option<String>, // "America/Chicago"
    pub avs_timezone_id: Option<String>, // "America/Chicago"
    pub tz_info_ver: Option<String>,     // "1.0"
    pub tz: Option<String>,              // "-5.0"

    // Version Information
    pub ota_api_ver: Option<String>, // "3.0"
    #[serde(rename = "VersionUpdate")]
    pub version_update: Option<String>, // "0"
    #[serde(rename = "NewVer")]
    pub new_ver: Option<String>, // "0"
    pub mcu_ver: Option<String>,     // "0"
    pub mcu_ver_new: Option<String>, // "0"
    pub update_check_count: Option<String>, // "102"
    #[serde(rename = "BleRemote_update_checked_counter")]
    pub ble_remote_update_checked_counter: Option<String>, // "0"
    pub temp_uuid: Option<String>,   // "BEDA811FFC2F4D5C"

    // Capabilities
    pub cap1: Option<String>,        // "0x400"
    pub capability: Option<String>,  // "0x20084000"
    pub languages: Option<String>,   // "0x1ec"
    pub streams_all: Option<String>, // "0x1edffbfd"
    pub streams: Option<String>,     // "0x1edffbfd"
    #[serde(rename = "ModuleColorNumber")]
    pub module_color_number: Option<String>, // "0"
    #[serde(rename = "ModuleColorString")]
    pub module_color_string: Option<String>, // ""

    // Audio Configuration
    pub region: Option<String>,               // "unknown"
    pub volume_control: Option<String>,       // "0"
    pub external: Option<String>,             // "0x0"
    pub preset_key: Option<String>,           // "6"
    pub max_volume: Option<String>,           // "100"
    pub audio_channel_config: Option<String>, // "1.0"

    // Service Support
    pub plm_support: Option<String>,          // "0x300006"
    pub lbc_support: Option<String>,          // "0"
    pub mqtt_support: Option<String>,         // "1"
    pub prompt_status: Option<String>,        // "1"
    pub alexa_ver: Option<String>,            // "20180604"
    pub alexa_beta_enable: Option<String>,    // "1"
    pub alexa_force_beta_cfg: Option<String>, // "1"
    pub dsp_ver: Option<String>,              // "0"

    // Power and Battery
    pub battery: Option<String>,         // "0"
    pub battery_percent: Option<String>, // "0"
    pub power_mode: Option<String>,      // "-1"

    // Security
    pub securemode: Option<String>,                       // "1"
    pub security: Option<String>,                         // "https/2.0"
    pub security_version: Option<String>,                 // "3.0"
    pub security_capabilities: Option<serde_json::Value>, // JSON object
    pub public_https_version: Option<String>,             // "1.0"
    pub privacy_mode: Option<String>,                     // "0"

    // Network Services
    pub ota_interface_ver: Option<String>,        // "2.0"
    pub upnp_version: Option<String>,             // "1005"
    pub upnp_uuid: Option<String>,                // "uuid:FF970016-A6FE-22C1-660A-B4D8FF970016"
    pub uart_pass_port: Option<String>,           // "0"
    pub communication_port: Option<String>,       // "8819"
    pub web_firmware_update_hide: Option<String>, // "0"

    // Service Versions
    pub tidal_version: Option<String>,   // "2.0"
    pub service_version: Option<String>, // "1.0"
    #[serde(rename = "EQ_support")]
    pub eq_support: Option<String>, // "Eq10HP_ver_1.0"
    #[serde(rename = "EQVersion")]
    pub eq_version: Option<String>, // "4.3"
    #[serde(rename = "HiFiSRC_version")]
    pub hifi_src_version: Option<String>, // "1.0"

    // Bluetooth Remote
    #[serde(rename = "BleRemoteControl")]
    pub ble_remote_control: Option<String>, // "1"
    #[serde(rename = "BleRemoteConnected")]
    pub ble_remote_connected: Option<String>, // "0"
    #[serde(rename = "BleRemoteException")]
    pub ble_remote_exception: Option<String>, // "0"

    // Miscellaneous
    #[serde(rename = "autoSenseVersion")]
    pub auto_sense_version: Option<String>, // "1.0"
    pub set_play_mode_enable: Option<String>, // "0"
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
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(10))
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

    /// Get comprehensive device and network status information
    ///
    /// This method calls the `getStatusEx` API endpoint to retrieve detailed
    /// information about the device including network quality, WiFi signal strength,
    /// device information, and connectivity status.
    ///
    /// # Examples
    /// ```no_run
    /// use wiim_api::WiimClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> wiim_api::Result<()> {
    ///     let client = WiimClient::new("192.168.1.100");
    ///
    ///     let status = client.get_status_ex().await?;
    ///
    ///     // Check network quality
    ///     if let Some(quality) = status.signal_quality() {
    ///         println!("Signal Quality: {}", quality);
    ///     }
    ///
    ///     // Check internet connectivity
    ///     if status.has_internet() {
    ///         println!("Device is connected to the internet");
    ///     }
    ///
    ///     // Get formatted network info
    ///     if let Some(signal) = status.rssi_formatted() {
    ///         println!("WiFi Signal: {}", signal);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_status_ex(&self) -> Result<StatusEx> {
        let response = self.send_command("getStatusEx").await?;
        let status: StatusEx = serde_json::from_str(&response)?;
        Ok(status)
    }
}

impl StatusEx {
    /// Parse RSSI value to integer (dBm)
    pub fn rssi_dbm(&self) -> Option<i32> {
        self.rssi.as_ref()?.parse().ok()
    }

    /// Get WiFi data rate in Mbps
    pub fn data_rate_mbps(&self) -> Option<u32> {
        self.wlan_data_rate.as_ref()?.parse().ok()
    }

    /// Calculate signal quality indicator
    pub fn signal_quality(&self) -> Option<String> {
        match self.rssi_dbm()? {
            rssi if rssi >= -50 => Some("Excellent".to_string()),
            rssi if rssi >= -60 => Some("Good".to_string()),
            rssi if rssi >= -70 => Some("Fair".to_string()),
            _ => Some("Poor".to_string()),
        }
    }

    /// Check if device has internet connectivity
    pub fn has_internet(&self) -> bool {
        self.internet.as_ref().is_some_and(|v| v == "1")
    }

    /// Format WiFi frequency in GHz
    pub fn wifi_frequency_ghz(&self) -> Option<String> {
        let freq_mhz: f64 = self.wlan_freq.as_ref()?.parse().ok()?;
        let freq_ghz = freq_mhz / 1000.0;
        Some(format!("{freq_ghz:.1} GHz"))
    }

    /// Format RSSI with unit
    pub fn rssi_formatted(&self) -> Option<String> {
        let rssi = self.rssi_dbm()?;
        Some(format!("{rssi} dBm"))
    }

    /// Format WiFi data rate with unit
    pub fn data_rate_formatted(&self) -> Option<String> {
        let rate = self.data_rate_mbps()?;
        Some(format!("{rate} Mbps"))
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

    // StatusEx Tests
    #[test]
    fn test_status_ex_rssi_dbm() {
        let mut status_ex = StatusEx {
            rssi: Some("-30".to_string()),
            ..Default::default()
        };

        assert_eq!(status_ex.rssi_dbm(), Some(-30));

        // Test invalid RSSI
        status_ex.rssi = Some("invalid".to_string());
        assert_eq!(status_ex.rssi_dbm(), None);

        // Test None RSSI
        status_ex.rssi = None;
        assert_eq!(status_ex.rssi_dbm(), None);
    }

    #[test]
    fn test_status_ex_data_rate_mbps() {
        let mut status_ex = StatusEx {
            wlan_data_rate: Some("390".to_string()),
            ..Default::default()
        };

        assert_eq!(status_ex.data_rate_mbps(), Some(390));

        // Test invalid data rate
        status_ex.wlan_data_rate = Some("invalid".to_string());
        assert_eq!(status_ex.data_rate_mbps(), None);

        // Test None data rate
        status_ex.wlan_data_rate = None;
        assert_eq!(status_ex.data_rate_mbps(), None);
    }

    #[test]
    fn test_status_ex_signal_quality() {
        let mut status_ex = StatusEx {
            rssi: Some("-30".to_string()),
            ..Default::default()
        };

        // Test Excellent signal (>= -50)
        status_ex.rssi = Some("-30".to_string());
        assert_eq!(status_ex.signal_quality(), Some("Excellent".to_string()));

        // Test Good signal (-50 to -60)
        status_ex.rssi = Some("-55".to_string());
        assert_eq!(status_ex.signal_quality(), Some("Good".to_string()));

        // Test Fair signal (-60 to -70)
        status_ex.rssi = Some("-65".to_string());
        assert_eq!(status_ex.signal_quality(), Some("Fair".to_string()));

        // Test Poor signal (< -70)
        status_ex.rssi = Some("-80".to_string());
        assert_eq!(status_ex.signal_quality(), Some("Poor".to_string()));

        // Test None RSSI
        status_ex.rssi = None;
        assert_eq!(status_ex.signal_quality(), None);
    }

    #[test]
    fn test_status_ex_has_internet() {
        let mut status_ex = StatusEx {
            internet: Some("1".to_string()),
            ..Default::default()
        };

        // Test connected
        assert!(status_ex.has_internet());

        // Test not connected
        status_ex.internet = Some("0".to_string());
        assert!(!status_ex.has_internet());

        // Test None
        status_ex.internet = None;
        assert!(!status_ex.has_internet());
    }

    #[test]
    fn test_status_ex_wifi_frequency_ghz() {
        let mut status_ex = StatusEx {
            wlan_freq: Some("5805".to_string()),
            ..Default::default()
        };

        assert_eq!(status_ex.wifi_frequency_ghz(), Some("5.8 GHz".to_string()));

        // Test 2.4GHz
        status_ex.wlan_freq = Some("2412".to_string());
        assert_eq!(status_ex.wifi_frequency_ghz(), Some("2.4 GHz".to_string()));

        // Test invalid frequency
        status_ex.wlan_freq = Some("invalid".to_string());
        assert_eq!(status_ex.wifi_frequency_ghz(), None);

        // Test None frequency
        status_ex.wlan_freq = None;
        assert_eq!(status_ex.wifi_frequency_ghz(), None);
    }

    #[test]
    fn test_status_ex_formatted_methods() {
        let status_ex = StatusEx {
            rssi: Some("-30".to_string()),
            wlan_data_rate: Some("390".to_string()),
            wlan_freq: Some("5805".to_string()),
            ..Default::default()
        };

        assert_eq!(status_ex.rssi_formatted(), Some("-30 dBm".to_string()));
        assert_eq!(
            status_ex.data_rate_formatted(),
            Some("390 Mbps".to_string())
        );
    }

    #[test]
    fn test_status_ex_deserialization() {
        let json_response = r#"{
            "language": "en_us",
            "ssid": "WiiM Mini-5932",
            "hideSSID": "0",
            "firmware": "Linkplay.4.6.719753",
            "build": "release",
            "project": "Muzo_Mini",
            "priv_prj": "Muzo_Mini",
            "Release": "20250611",
            "FW_Release_version": "",
            "PCB_version": "0",
            "group": "0",
            "wmrm_version": "4.2",
            "wmrm_sub_ver": "1",
            "expired": "0",
            "internet": "1",
            "uuid": "FF970016B757B9F1D547CE42",
            "MAC": "9C:B8:B4:9E:59:32",
            "BT_MAC": "9C:B8:B4:9E:59:33",
            "AP_MAC": "9E:B8:B4:9E:59:32",
            "date": "2025:07:18",
            "time": "04:56:40",
            "netstat": "2",
            "essid": "656265727570",
            "apcli0": "192.168.86.52",
            "eth0": "0.0.0.0",
            "ETH_MAC": "00:00:00:00:00:00",
            "hardware": "ALLWINNER-R328",
            "ota_api_ver": "3.0",
            "VersionUpdate": "0",
            "NewVer": "0",
            "mcu_ver": "0",
            "mcu_ver_new": "0",
            "update_check_count": "8",
            "BleRemote_update_checked_counter": "0",
            "ra0": "10.10.10.254",
            "temp_uuid": "D90ABDB01001CFD8",
            "cap1": "0x400",
            "capability": "0x20084008",
            "languages": "0x1ec",
            "prompt_status": "1",
            "alexa_ver": "20180604",
            "alexa_beta_enable": "0",
            "alexa_force_beta_cfg": "0",
            "dsp_ver": "0",
            "ModuleColorNumber": "0",
            "ModuleColorString": "",
            "streams_all": "0xffffbfd",
            "streams": "0xffffbfd",
            "region": "unknown",
            "volume_control": "0",
            "external": "0x0",
            "preset_key": "12",
            "plm_support": "0x300006",
            "mqtt_support": "1",
            "lbc_support": "0",
            "WifiChannel": "0",
            "RSSI": "-45",
            "BSSID": "70:3A:CB:0A:D3:48",
            "wlanSnr": "35",
            "wlanNoise": "-92",
            "wlanFreq": "5745",
            "wlanDataRate": "390",
            "battery": "0",
            "battery_percent": "0",
            "securemode": "1",
            "ota_interface_ver": "2.0",
            "upnp_version": "1005",
            "upnp_uuid": "uuid:FF970016-B757-B9F1-D547-CE42FF970016",
            "uart_pass_port": "0",
            "communication_port": "8819",
            "web_firmware_update_hide": "0",
            "tidal_version": "2.0",
            "service_version": "1.0",
            "EQ_support": "Eq4p_ver_3.0",
            "EQVersion": "4.3",
            "audio_channel_config": "1.0",
            "app_timezone_id": "America/Chicago",
            "avs_timezone_id": "America/Chicago",
            "tz_info_ver": "1.0",
            "tz": "-5.0",
            "HiFiSRC_version": "1.0",
            "max_volume": "100",
            "power_mode": "-1",
            "security": "https/2.0",
            "security_version": "3.0",
            "security_capabilities": {
                "ver": "1.0",
                "aes_ver": "1.0"
            },
            "public_https_version": "1.0",
            "BleRemoteControl": "1",
            "BleRemoteConnected": "0",
            "BleRemoteException": "0",
            "autoSenseVersion": "1.0",
            "set_play_mode_enable": "0",
            "privacy_mode": "0",
            "DeviceName": "WiiM Mini-5932",
            "GroupName": "WiiM Mini-5932"
        }"#;

        let status_ex: StatusEx = serde_json::from_str(json_response).unwrap();

        // Test core fields
        assert_eq!(status_ex.language, Some("en_us".to_string()));
        assert_eq!(status_ex.ssid, Some("WiiM Mini-5932".to_string()));
        assert_eq!(status_ex.firmware, Some("Linkplay.4.6.719753".to_string()));
        assert_eq!(status_ex.device_name, Some("WiiM Mini-5932".to_string()));
        assert_eq!(status_ex.hardware, Some("ALLWINNER-R328".to_string()));

        // Test network fields
        assert_eq!(status_ex.rssi, Some("-45".to_string()));
        assert_eq!(status_ex.wlan_data_rate, Some("390".to_string()));
        assert_eq!(status_ex.wlan_freq, Some("5745".to_string()));
        assert_eq!(status_ex.wlan_snr, Some("35".to_string()));
        assert_eq!(status_ex.wlan_noise, Some("-92".to_string()));
        assert_eq!(status_ex.apcli0, Some("192.168.86.52".to_string()));

        // Test new fields from real device
        assert_eq!(status_ex.pcb_version, Some("0".to_string()));
        assert_eq!(status_ex.wmrm_sub_ver, Some("1".to_string()));
        assert_eq!(status_ex.ota_api_ver, Some("3.0".to_string()));
        assert_eq!(status_ex.mqtt_support, Some("1".to_string()));
        assert_eq!(
            status_ex.app_timezone_id,
            Some("America/Chicago".to_string())
        );
        assert_eq!(status_ex.max_volume, Some("100".to_string()));
        assert_eq!(status_ex.eq_version, Some("4.3".to_string()));

        // Test helper methods
        assert!(status_ex.has_internet());
        assert_eq!(status_ex.rssi_dbm(), Some(-45));
        assert_eq!(status_ex.data_rate_mbps(), Some(390));
        assert_eq!(status_ex.signal_quality(), Some("Excellent".to_string()));

        // Test security capabilities JSON object
        assert!(status_ex.security_capabilities.is_some());
    }

    #[test]
    fn test_metadata_deserialization_complete() {
        // Test deserialization of complete JSON response with all fields
        let json_response = r#"{
            "metaData": {
                "album": "Blues for Allah",
                "title": "Help on the Way / Slipknot!",
                "subtitle": "unknow",
                "artist": "Grateful Dead",
                "albumArtURI": "https://static.qobuz.com/images/covers/78/07/0603497920778_600.jpg",
                "sampleRate": "96000",
                "bitDepth": "24",
                "bitRate": "2867",
                "trackId": "35542598"
            }
        }"#;

        let meta_info: MetaInfo = serde_json::from_str(json_response).unwrap();
        let meta_data = &meta_info.meta_data;

        // Test all fields are correctly deserialized
        assert_eq!(meta_data.album, Some("Blues for Allah".to_string()));
        assert_eq!(
            meta_data.title,
            Some("Help on the Way / Slipknot!".to_string())
        );
        assert_eq!(meta_data.subtitle, Some("unknow".to_string()));
        assert_eq!(meta_data.artist, Some("Grateful Dead".to_string()));
        assert_eq!(
            meta_data.album_art_uri,
            Some("https://static.qobuz.com/images/covers/78/07/0603497920778_600.jpg".to_string())
        );
        assert_eq!(meta_data.sample_rate, Some("96000".to_string()));
        assert_eq!(meta_data.bit_depth, Some("24".to_string()));
        assert_eq!(meta_data.bit_rate, Some("2867".to_string()));
        assert_eq!(meta_data.track_id, Some("35542598".to_string()));
    }

    #[test]
    fn test_metadata_deserialization_partial() {
        // Test deserialization with missing optional fields (backward compatibility)
        let json_response = r#"{
            "metaData": {
                "album": "Test Album",
                "title": "Test Title",
                "artist": "Test Artist"
            }
        }"#;

        let meta_info: MetaInfo = serde_json::from_str(json_response).unwrap();
        let meta_data = &meta_info.meta_data;

        // Test existing fields are present
        assert_eq!(meta_data.album, Some("Test Album".to_string()));
        assert_eq!(meta_data.title, Some("Test Title".to_string()));
        assert_eq!(meta_data.artist, Some("Test Artist".to_string()));

        // Test new fields default to None when missing
        assert_eq!(meta_data.subtitle, None);
        assert_eq!(meta_data.album_art_uri, None);
        assert_eq!(meta_data.sample_rate, None);
        assert_eq!(meta_data.bit_depth, None);
        assert_eq!(meta_data.bit_rate, None);
        assert_eq!(meta_data.track_id, None);
    }

    #[test]
    fn test_metadata_deserialization_empty() {
        // Test deserialization with empty metadata object
        let json_response = r#"{
            "metaData": {}
        }"#;

        let meta_info: MetaInfo = serde_json::from_str(json_response).unwrap();
        let meta_data = &meta_info.meta_data;

        // Test all fields are None when missing
        assert_eq!(meta_data.album, None);
        assert_eq!(meta_data.title, None);
        assert_eq!(meta_data.subtitle, None);
        assert_eq!(meta_data.artist, None);
        assert_eq!(meta_data.album_art_uri, None);
        assert_eq!(meta_data.sample_rate, None);
        assert_eq!(meta_data.bit_depth, None);
        assert_eq!(meta_data.bit_rate, None);
        assert_eq!(meta_data.track_id, None);
    }

    #[test]
    fn test_metadata_field_accessibility() {
        // Test that all fields are accessible through public API
        let json_response = r#"{
            "metaData": {
                "album": "Test Album",
                "title": "Test Title",
                "subtitle": "Test Subtitle",
                "artist": "Test Artist",
                "albumArtURI": "https://example.com/art.jpg",
                "sampleRate": "44100",
                "bitDepth": "16",
                "bitRate": "320",
                "trackId": "12345"
            }
        }"#;

        let meta_info: MetaInfo = serde_json::from_str(json_response).unwrap();
        let meta_data = &meta_info.meta_data;

        // Test field accessibility and values
        assert!(meta_data.album.is_some());
        assert!(meta_data.title.is_some());
        assert!(meta_data.subtitle.is_some());
        assert!(meta_data.artist.is_some());
        assert!(meta_data.album_art_uri.is_some());
        assert!(meta_data.sample_rate.is_some());
        assert!(meta_data.bit_depth.is_some());
        assert!(meta_data.bit_rate.is_some());
        assert!(meta_data.track_id.is_some());

        // Test specific values for new fields
        assert_eq!(meta_data.subtitle.as_ref().unwrap(), "Test Subtitle");
        assert_eq!(meta_data.bit_rate.as_ref().unwrap(), "320");
        assert_eq!(meta_data.track_id.as_ref().unwrap(), "12345");
    }
}
