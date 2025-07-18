use clap::{Parser, Subcommand};
use handlebars::Handlebars;
use serde::Serialize;
use std::collections::HashMap;
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

    /// Output format (legacy, use --profile instead)
    #[arg(short, long)]
    format: Option<OutputFormat>,

    /// Output profile (waybar, polybar, custom)
    #[arg(short, long)]
    profile: Option<String>,

    /// Template string override (requires --profile)
    #[arg(short, long)]
    template: Option<String>,

    /// Config file path (default: ~/.config/wiim-control/config.toml)
    #[arg(short, long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, clap::ValueEnum)]
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

#[derive(Serialize)]
struct TemplateContext {
    // Track Information
    artist: Option<String>,
    title: Option<String>,
    album: Option<String>,
    album_art_uri: Option<String>,

    // Playback State
    state: String,
    volume: u8,
    muted: bool,
    position: String,
    duration: String,
    position_ms: u64,
    duration_ms: u64,

    // Audio Quality
    sample_rate: Option<String>,
    bit_depth: Option<String>,
    sample_rate_khz: Option<String>,
    bit_depth_bit: Option<String>,
    quality_info: Option<String>,

    // Formatted Combinations
    track_info: String,
    full_info: String,
}

#[derive(serde::Deserialize)]
struct Config {
    device_ip: String,
    output: Option<OutputConfig>,
    #[allow(dead_code)]
    profiles: Option<HashMap<String, ProfileConfig>>,
}

#[derive(serde::Deserialize)]
struct OutputConfig {
    text: Option<TextTemplates>,
    json: Option<JsonTemplates>,
}

#[derive(serde::Deserialize)]
struct TextTemplates {
    playing: Option<String>,
    paused: Option<String>,
    stopped: Option<String>,
    loading: Option<String>,
}

#[derive(serde::Deserialize)]
struct JsonTemplates {
    text: Option<String>,
    alt: Option<String>,
    tooltip: Option<String>,
    class: Option<String>,
    #[allow(dead_code)]
    percentage: Option<String>,
}

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct ProfileConfig {
    format: Option<String>,
    text_template: Option<String>,
    json_template: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            device_ip: "192.168.1.100".to_string(),
            output: None,
            profiles: None,
        }
    }
}

#[derive(Debug, Clone)]
struct ResolvedProfile {
    format: OutputFormat,
    text_template: Option<String>,
    json_templates: Option<JsonTemplatesResolved>,
}

impl From<&wiim_api::NowPlaying> for TemplateContext {
    fn from(now_playing: &wiim_api::NowPlaying) -> Self {
        // Helper function to format time from milliseconds
        fn format_time(ms: u64) -> String {
            if ms == 0 {
                return "0:00".to_string();
            }
            let minutes = ms / 60000;
            let seconds = (ms % 60000) / 1000;
            format!("{minutes}:{seconds:02}")
        }

        // Helper function to format sample rate
        fn format_sample_rate_khz(sample_rate: &Option<String>) -> Option<String> {
            sample_rate.as_ref().and_then(|sr| {
                sr.parse::<f32>()
                    .ok()
                    .map(|rate| format!("{:.0}kHz", rate / 1000.0))
            })
        }

        // Helper function to format bit depth
        fn format_bit_depth_bit(bit_depth: &Option<String>) -> Option<String> {
            bit_depth.as_ref().map(|bd| format!("{bd}bit"))
        }

        // Helper function to format quality info
        fn format_quality_info(
            sample_rate: &Option<String>,
            bit_depth: &Option<String>,
        ) -> Option<String> {
            match (sample_rate, bit_depth) {
                (Some(sr), Some(bd)) => {
                    if let Ok(rate) = sr.parse::<f32>() {
                        Some(format!("{:.0}kHz/{}bit", rate / 1000.0, bd))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }

        // Helper function to format track info (same logic as original)
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

        // Helper function to format full info (same logic as original tooltip)
        fn format_full_info(now_playing: &wiim_api::NowPlaying) -> String {
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

            if let (Some(sample_rate), Some(bit_depth)) =
                (&now_playing.sample_rate, &now_playing.bit_depth)
            {
                if let Ok(rate) = sample_rate.parse::<f32>() {
                    parts.push(format!("Quality: {:.0}kHz/{}bit", rate / 1000.0, bit_depth));
                }
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

        let position = format_time(now_playing.position_ms);
        let duration = format_time(now_playing.duration_ms);
        let sample_rate_khz = format_sample_rate_khz(&now_playing.sample_rate);
        let bit_depth_bit = format_bit_depth_bit(&now_playing.bit_depth);
        let quality_info = format_quality_info(&now_playing.sample_rate, &now_playing.bit_depth);
        let track_info = format_track_info(now_playing);
        let full_info = format_full_info(now_playing);

        TemplateContext {
            // Track Information
            artist: now_playing.artist.clone(),
            title: now_playing.title.clone(),
            album: now_playing.album.clone(),
            album_art_uri: now_playing.album_art_uri.clone(),

            // Playback State
            state: now_playing.state.to_string(),
            volume: now_playing.volume,
            muted: now_playing.is_muted,
            position,
            duration,
            position_ms: now_playing.position_ms,
            duration_ms: now_playing.duration_ms,

            // Audio Quality
            sample_rate: now_playing.sample_rate.clone(),
            bit_depth: now_playing.bit_depth.clone(),
            sample_rate_khz,
            bit_depth_bit,
            quality_info,

            // Formatted Combinations
            track_info,
            full_info,
        }
    }
}

fn validate_template(template: &str) -> Result<(), String> {
    let mut handlebars = Handlebars::new();

    // Check for common syntax mistakes first
    if template.contains('{') {
        // Check if there are any single braces (not part of double braces)
        let mut chars = template.chars().peekable();
        let mut has_single_braces = false;

        while let Some(ch) = chars.next() {
            if ch == '{' {
                // Check if this is a single brace or part of double braces
                match chars.peek() {
                    Some('{') => {
                        // This is a double brace, consume the next '{'
                        chars.next();
                    }
                    _ => {
                        // This is a single brace
                        has_single_braces = true;
                        break;
                    }
                }
            }
        }

        if has_single_braces {
            return Err("Invalid template syntax: found single braces. \
                 Template variables must use double braces like {{variable}}. \
                 Example: '{{artist}} - {{title}}'"
                .to_string());
        }
    }

    handlebars
        .register_template_string("validation", template)
        .map_err(|e| {
            let error_msg = e.to_string();
            if error_msg.contains("unclosed") || error_msg.contains("unexpected") {
                format!(
                    "Invalid template syntax: {error_msg}. \
                     Make sure to use double braces like {{{{variable}}}}. \
                     Example: '{{{{artist}}}} - {{{{title}}}}'"
                )
            } else {
                format!("Invalid template syntax: {error_msg}")
            }
        })?;
    Ok(())
}

fn resolve_profile(cli: &Cli, config: &Config) -> Result<ResolvedProfile, String> {
    // 1. CLI --template argument (highest priority)
    if let Some(template) = &cli.template {
        // We already validated that --template requires --profile
        let profile_name = cli.profile.as_ref().unwrap();

        // Validate template syntax
        if let Err(e) = validate_template(template) {
            return Err(format!("Invalid template syntax: {e}"));
        }

        // For template override, we need to determine the output format
        // Check if the profile exists in config first, otherwise default to text
        let format = if let Some(profiles) = &config.profiles {
            if let Some(profile_config) = profiles.get(profile_name) {
                match profile_config.format.as_deref() {
                    Some("json") => OutputFormat::Json,
                    _ => OutputFormat::Text,
                }
            } else {
                OutputFormat::Text
            }
        } else {
            OutputFormat::Text
        };

        return Ok(ResolvedProfile {
            format,
            text_template: Some(template.clone()),
            json_templates: None,
        });
    }

    // 2. CLI --profile argument
    if let Some(profile_name) = &cli.profile {
        if let Some(profiles) = &config.profiles {
            if let Some(profile_config) = profiles.get(profile_name) {
                let format = match profile_config.format.as_deref() {
                    Some("json") => OutputFormat::Json,
                    _ => OutputFormat::Text,
                };

                return Ok(ResolvedProfile {
                    format,
                    text_template: profile_config.text_template.clone(),
                    json_templates: profile_config.json_template.as_ref().map(|_| {
                        // For now, we'll use the default JSON templates
                        // This could be enhanced later to support JSON template overrides
                        get_json_templates(config)
                    }),
                });
            } else {
                let available_profiles = profiles.keys().map(|k| k.as_str()).collect::<Vec<_>>();
                let available_list = available_profiles.join(", ");
                return Err(format!(
                    "Profile '{profile_name}' not found in configuration. Available profiles: {available_list}"
                ));
            }
        } else {
            return Err(format!(
                "Profile '{profile_name}' not found in configuration. No profiles are configured."
            ));
        }
    }

    // 3. CLI --format argument (legacy, maps to default profiles)
    if let Some(format) = &cli.format {
        return Ok(ResolvedProfile {
            format: format.clone(),
            text_template: None,
            json_templates: None,
        });
    }

    // 4. Config file default profile
    // For now, we'll skip this as the config structure doesn't have a default profile field

    // 5. Built-in default (backward compatibility)
    Ok(ResolvedProfile {
        format: OutputFormat::Text,
        text_template: None,
        json_templates: None,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Validate that --template requires --profile
    if cli.template.is_some() && cli.profile.is_none() {
        return Err("--template requires --profile to be specified".into());
    }

    // Load configuration
    let config = load_config(&cli.config).await?;

    // Resolve profile configuration
    let resolved_profile =
        resolve_profile(&cli, &config).map_err(|e| format!("Profile resolution error: {e}"))?;

    // Get device IP from CLI arg or config
    let device_ip = cli.device.as_ref().unwrap_or(&config.device_ip);

    // Create client
    let client = WiimClient::new(device_ip);

    // Execute command
    match cli.command {
        Commands::Status => {
            handle_status(&client, &resolved_profile, &config).await?;
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

async fn handle_status(
    client: &WiimClient,
    resolved_profile: &ResolvedProfile,
    config: &Config,
) -> WiimResult<()> {
    let now_playing = client.get_now_playing().await?;
    let context = TemplateContext::from(&now_playing);

    match resolved_profile.format {
        OutputFormat::Text => {
            let template = if let Some(text_template) = &resolved_profile.text_template {
                // Use the resolved template from profile or CLI override
                text_template.clone()
            } else {
                // Fall back to the existing template resolution logic
                get_text_template(config, &now_playing.state)
            };
            let output = render_template(&template, &context)?;
            println!("{output}");
        }
        OutputFormat::Json => {
            let templates = if let Some(json_templates) = &resolved_profile.json_templates {
                // Use the resolved JSON templates from profile
                json_templates.clone()
            } else {
                // Fall back to the existing template resolution logic
                get_json_templates(config)
            };
            let output = StatusOutput {
                text: render_template(&templates.text, &context)?,
                alt: render_template(&templates.alt, &context)?,
                tooltip: render_template(&templates.tooltip, &context)?,
                class: render_template(&templates.class, &context)?,
                percentage: Some(now_playing.volume),
            };
            println!("{}", serde_json::to_string(&output)?);
        }
    }

    Ok(())
}

fn get_text_template(config: &Config, state: &PlayState) -> String {
    let default_icon = match state {
        PlayState::Playing => "▶️",
        PlayState::Paused => "⏸️",
        PlayState::Stopped => "⏹️",
        PlayState::Loading => "⏳",
    };

    if let Some(output) = &config.output {
        if let Some(text) = &output.text {
            let template = match state {
                PlayState::Playing => text.playing.as_ref(),
                PlayState::Paused => text.paused.as_ref(),
                PlayState::Stopped => text.stopped.as_ref(),
                PlayState::Loading => text.loading.as_ref(),
            };

            if let Some(template) = template {
                return template.clone();
            }
        }
    }

    // Default template that matches current behavior
    format!("{default_icon} {{{{track_info}}}}")
}

#[derive(Debug, Clone)]
struct JsonTemplatesResolved {
    text: String,
    alt: String,
    tooltip: String,
    class: String,
}

fn get_json_templates(config: &Config) -> JsonTemplatesResolved {
    let defaults = JsonTemplatesResolved {
        text: "{{track_info}}".to_string(),
        alt: "{{state}}".to_string(),
        tooltip: "{{full_info}}".to_string(),
        class: "{{state}}".to_string(),
    };

    if let Some(output) = &config.output {
        if let Some(json) = &output.json {
            return JsonTemplatesResolved {
                text: json.text.clone().unwrap_or(defaults.text),
                alt: json.alt.clone().unwrap_or(defaults.alt),
                tooltip: json.tooltip.clone().unwrap_or(defaults.tooltip),
                class: json.class.clone().unwrap_or(defaults.class),
            };
        }
    }

    defaults
}

fn render_template(template: &str, context: &TemplateContext) -> WiimResult<String> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("template", template)
        .map_err(|e| wiim_api::WiimError::InvalidResponse(format!("Template error: {e}")))?;
    handlebars
        .render("template", context)
        .map_err(|e| wiim_api::WiimError::InvalidResponse(format!("Template render error: {e}")))
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

#[cfg(test)]
mod tests {
    use super::*;
    use wiim_api::{NowPlaying, PlayState};

    fn create_test_now_playing() -> NowPlaying {
        NowPlaying {
            title: Some("Test Title".to_string()),
            artist: Some("Test Artist".to_string()),
            album: Some("Test Album".to_string()),
            album_art_uri: Some("https://example.com/art.jpg".to_string()),
            state: PlayState::Playing,
            volume: 75,
            is_muted: false,
            position_ms: 60000,  // 1 minute
            duration_ms: 180000, // 3 minutes
            sample_rate: Some("44100".to_string()),
            bit_depth: Some("16".to_string()),
        }
    }

    #[test]
    fn test_template_context_creation() {
        let now_playing = create_test_now_playing();
        let context = TemplateContext::from(&now_playing);

        assert_eq!(context.artist, Some("Test Artist".to_string()));
        assert_eq!(context.title, Some("Test Title".to_string()));
        assert_eq!(context.album, Some("Test Album".to_string()));
        assert_eq!(context.state, "playing");
        assert_eq!(context.volume, 75);
        assert!(!context.muted);
        assert_eq!(context.position, "1:00");
        assert_eq!(context.duration, "3:00");
        assert_eq!(context.sample_rate_khz, Some("44kHz".to_string()));
        assert_eq!(context.bit_depth_bit, Some("16bit".to_string()));
        assert_eq!(context.quality_info, Some("44kHz/16bit".to_string()));
        assert_eq!(context.track_info, "Test Artist - Test Title");
    }

    #[test]
    fn test_template_context_with_missing_fields() {
        let now_playing = NowPlaying {
            title: None,
            artist: Some("Test Artist".to_string()),
            album: None,
            album_art_uri: None,
            state: PlayState::Stopped,
            volume: 50,
            is_muted: true,
            position_ms: 0,
            duration_ms: 0,
            sample_rate: None,
            bit_depth: None,
        };

        let context = TemplateContext::from(&now_playing);

        assert_eq!(context.artist, Some("Test Artist".to_string()));
        assert_eq!(context.title, None);
        assert_eq!(context.album, None);
        assert_eq!(context.state, "stopped");
        assert_eq!(context.volume, 50);
        assert!(context.muted);
        assert_eq!(context.position, "0:00");
        assert_eq!(context.duration, "0:00");
        assert_eq!(context.sample_rate_khz, None);
        assert_eq!(context.bit_depth_bit, None);
        assert_eq!(context.quality_info, None);
        assert_eq!(context.track_info, "Test Artist");
    }

    #[test]
    fn test_template_context_no_track_info() {
        let now_playing = NowPlaying {
            title: None,
            artist: None,
            album: None,
            album_art_uri: None,
            state: PlayState::Stopped,
            volume: 50,
            is_muted: false,
            position_ms: 0,
            duration_ms: 0,
            sample_rate: None,
            bit_depth: None,
        };

        let context = TemplateContext::from(&now_playing);
        assert_eq!(context.track_info, "No track info");
    }

    #[test]
    fn test_render_template_basic() {
        let now_playing = create_test_now_playing();
        let context = TemplateContext::from(&now_playing);

        let result = render_template("{{artist}} - {{title}}", &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test Artist - Test Title");
    }

    #[test]
    fn test_render_template_with_missing_fields() {
        let now_playing = NowPlaying {
            title: None,
            artist: Some("Test Artist".to_string()),
            album: None,
            album_art_uri: None,
            state: PlayState::Playing,
            volume: 50,
            is_muted: false,
            position_ms: 0,
            duration_ms: 0,
            sample_rate: None,
            bit_depth: None,
        };

        let context = TemplateContext::from(&now_playing);

        let result = render_template("{{artist}} - {{title}}", &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test Artist - ");
    }

    #[test]
    fn test_render_template_invalid_syntax() {
        let now_playing = create_test_now_playing();
        let context = TemplateContext::from(&now_playing);

        let result = render_template("{{artist} - {{title}}", &context);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_text_template_default() {
        let config = Config::default();
        let template = get_text_template(&config, &PlayState::Playing);
        assert_eq!(template, "▶️ {{track_info}}");

        let template = get_text_template(&config, &PlayState::Paused);
        assert_eq!(template, "⏸️ {{track_info}}");

        let template = get_text_template(&config, &PlayState::Stopped);
        assert_eq!(template, "⏹️ {{track_info}}");

        let template = get_text_template(&config, &PlayState::Loading);
        assert_eq!(template, "⏳ {{track_info}}");
    }

    #[test]
    fn test_get_json_templates_default() {
        let config = Config::default();
        let templates = get_json_templates(&config);

        assert_eq!(templates.text, "{{track_info}}");
        assert_eq!(templates.alt, "{{state}}");
        assert_eq!(templates.tooltip, "{{full_info}}");
        assert_eq!(templates.class, "{{state}}");
    }

    #[test]
    fn test_validate_template_single_braces() {
        let result = validate_template("{artist} - {title}");
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("Invalid template syntax: found single braces"));
        assert!(error_msg.contains("double braces like {{variable}}"));
    }

    #[test]
    fn test_validate_template_double_braces() {
        let result = validate_template("{{artist}} - {{title}}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_template_mixed_braces() {
        let result = validate_template("{{artist}} - {title}");
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("Invalid template syntax: found single braces"));
    }

    #[test]
    fn test_validate_template_unclosed_braces() {
        let result = validate_template("{{artist} - {{title}}");
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        println!("Actual error message: {error_msg}");
        // The unclosed brace should be caught by our single brace detection or handlebars
        assert!(error_msg.contains("Invalid template syntax"));
    }

    #[test]
    fn test_validate_template_no_braces() {
        let result = validate_template("Now Playing");
        assert!(result.is_ok());
    }

    #[test]
    fn test_template_context_formatting() {
        let now_playing = NowPlaying {
            title: Some("Test Title".to_string()),
            artist: Some("Test Artist".to_string()),
            album: Some("Test Album".to_string()),
            album_art_uri: None,
            state: PlayState::Playing,
            volume: 85,
            is_muted: true,
            position_ms: 125000, // 2:05
            duration_ms: 245000, // 4:05
            sample_rate: Some("96000".to_string()),
            bit_depth: Some("24".to_string()),
        };

        let context = TemplateContext::from(&now_playing);

        assert_eq!(context.position, "2:05");
        assert_eq!(context.duration, "4:05");
        assert_eq!(context.sample_rate_khz, Some("96kHz".to_string()));
        assert_eq!(context.bit_depth_bit, Some("24bit".to_string()));
        assert_eq!(context.quality_info, Some("96kHz/24bit".to_string()));
        assert_eq!(context.volume, 85);
        assert!(context.muted);
        assert!(context.full_info.contains("Volume: 85%"));
        assert!(context.full_info.contains("🔇 Muted"));
        assert!(context.full_info.contains("Quality: 96kHz/24bit"));
        assert!(context.full_info.contains("Time: 2:05 / 4:05"));
    }
}
