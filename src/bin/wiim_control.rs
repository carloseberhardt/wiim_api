use clap::{Parser, Subcommand};
use serde::Serialize;
use std::path::PathBuf;
use tokio::fs;
use wiim_api::{PlayState, Result as WiimResult, WiimClient};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "wiim-control")]
#[command(about = "Control and monitor WiiM audio streaming devices")]
struct Cli {
    /// WiiM device IP address (overrides config file)
    #[arg(short, long)]
    device: Option<String>,

    /// Output format for status command
    #[arg(short, long, default_value = "text")]
    format: OutputFormat,

    /// Config file path (default: ~/.config/wiim-control/config.toml)
    #[arg(short, long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, clap::ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Subcommand)]
enum Commands {
    /// Show current playback status and track info
    Status,
    /// Play/resume playback
    Play,
    /// Pause playback
    Pause,
    /// Toggle play/pause
    Toggle,
    /// Stop playback
    Stop,
    /// Next track
    Next,
    /// Previous track
    Prev,
    /// Set volume (0-100)
    Volume { level: u8 },
    /// Increase volume by step (default 5)
    VolumeUp {
        #[arg(default_value = "5")]
        step: u8,
    },
    /// Decrease volume by step (default 5)
    VolumeDown {
        #[arg(default_value = "5")]
        step: u8,
    },
    /// Mute audio
    Mute,
    /// Unmute audio
    Unmute,
}

#[derive(Serialize)]
struct StatusOutput {
    text: String,
    alt: String,
    tooltip: String,
    class: String,
    percentage: Option<u8>,
}

#[derive(serde::Deserialize)]
struct Config {
    device_ip: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            device_ip: "192.168.1.100".to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load configuration
    let config = load_config(&cli.config).await?;

    // Get device IP from CLI arg or config
    let device_ip = cli.device.as_ref().unwrap_or(&config.device_ip);

    // Create client
    let client = WiimClient::new(device_ip);

    // Execute command
    match cli.command {
        Commands::Status => {
            handle_status(&client, &cli.format).await?;
        }
        Commands::Play => {
            client.resume().await?;
            eprintln!("▶️ Playing");
        }
        Commands::Pause => {
            client.pause().await?;
            eprintln!("⏸️ Paused");
        }
        Commands::Toggle => {
            client.toggle_play_pause().await?;
            eprintln!("⏯️ Toggled");
        }
        Commands::Stop => {
            client.stop().await?;
            eprintln!("⏹️ Stopped");
        }
        Commands::Next => {
            client.next_track().await?;
            eprintln!("⏭️ Next track");
        }
        Commands::Prev => {
            client.previous_track().await?;
            eprintln!("⏮️ Previous track");
        }
        Commands::Volume { level } => {
            client.set_volume(level).await?;
            eprintln!("🔊 Volume set to {level}%");
        }
        Commands::VolumeUp { step } => {
            let new_volume = client.volume_up(Some(step)).await?;
            eprintln!("🔊 Volume up to {new_volume}%");
        }
        Commands::VolumeDown { step } => {
            let new_volume = client.volume_down(Some(step)).await?;
            eprintln!("🔊 Volume down to {new_volume}%");
        }
        Commands::Mute => {
            client.mute().await?;
            eprintln!("🔇 Muted");
        }
        Commands::Unmute => {
            client.unmute().await?;
            eprintln!("🔊 Unmuted");
        }
    }

    Ok(())
}

async fn handle_status(client: &WiimClient, format: &OutputFormat) -> WiimResult<()> {
    let now_playing = client.get_now_playing().await?;

    match format {
        OutputFormat::Text => {
            let status_icon = match now_playing.state {
                PlayState::Playing => "▶️",
                PlayState::Paused => "⏸️",
                PlayState::Stopped => "⏹️",
                PlayState::Loading => "⏳",
            };

            let track_info = format_track_info(&now_playing);
            println!("{status_icon} {track_info}");
        }
        OutputFormat::Json => {
            let status_class = match now_playing.state {
                PlayState::Playing => "playing",
                PlayState::Paused => "paused",
                PlayState::Stopped => "stopped",
                PlayState::Loading => "loading",
            };

            let track_info = format_track_info(&now_playing);
            let tooltip = format_tooltip(&now_playing);

            let output = StatusOutput {
                text: track_info.clone(),
                alt: format!("{:?}", now_playing.state).to_lowercase(),
                tooltip,
                class: status_class.to_string(),
                percentage: Some(now_playing.volume),
            };

            println!("{}", serde_json::to_string(&output)?);
        }
    }

    Ok(())
}

fn format_track_info(now_playing: &wiim_api::NowPlaying) -> String {
    match (&now_playing.artist, &now_playing.title) {
        (Some(artist), Some(title)) => format!("{artist} - {title}"),
        (Some(artist), None) => artist.clone(),
        (None, Some(title)) => title.clone(),
        (None, None) => {
            if let Some(album) = &now_playing.album {
                album.clone()
            } else {
                "No track info".to_string()
            }
        }
    }
}

fn format_tooltip(now_playing: &wiim_api::NowPlaying) -> String {
    let mut parts = Vec::new();

    if let Some(title) = &now_playing.title {
        parts.push(format!("Title: {title}"));
    }
    if let Some(artist) = &now_playing.artist {
        parts.push(format!("Artist: {artist}"));
    }
    if let Some(album) = &now_playing.album {
        parts.push(format!("Album: {album}"));
    }

    parts.push(format!("Volume: {}%", now_playing.volume));

    if now_playing.is_muted {
        parts.push("🔇 Muted".to_string());
    }

    if let (Some(sample_rate), Some(bit_depth)) = (&now_playing.sample_rate, &now_playing.bit_depth)
    {
        parts.push(format!("Quality: {sample_rate}kHz/{bit_depth}bit"));
    }

    // Format position/duration
    if now_playing.duration_ms > 0 {
        let pos_min = now_playing.position_ms / 60000;
        let pos_sec = (now_playing.position_ms % 60000) / 1000;
        let dur_min = now_playing.duration_ms / 60000;
        let dur_sec = (now_playing.duration_ms % 60000) / 1000;

        parts.push(format!(
            "Time: {pos_min}:{pos_sec:02} / {dur_min}:{dur_sec:02}"
        ));
    }

    parts.join("\n")
}

async fn load_config(config_path: &Option<PathBuf>) -> Result<Config, Box<dyn std::error::Error>> {
    let config_file = match config_path {
        Some(path) => path.clone(),
        None => {
            let config_dir = dirs::config_dir()
                .ok_or("Could not find config directory")?
                .join("wiim-control");

            // Create config directory if it doesn't exist
            if !config_dir.exists() {
                fs::create_dir_all(&config_dir).await?;

                // Create default config file
                let default_config = Config::default();
                let config_content = format!("device_ip = \"{}\"\n", default_config.device_ip);
                let config_file = config_dir.join("config.toml");
                fs::write(&config_file, config_content).await?;
                eprintln!("Created default config at: {}", config_file.display());
                return Ok(default_config);
            }

            config_dir.join("config.toml")
        }
    };

    // Try to read config file
    if config_file.exists() {
        let content = fs::read_to_string(&config_file).await?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}
