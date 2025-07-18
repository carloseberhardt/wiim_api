# WiiM Control CLI Tool

Command-line interface for controlling WiiM audio streaming devices with template-based output formatting for integration with status bars and automation tools.

## Installation

### From Source

```bash
cargo install --path . --bin wiim-control
```

### Configuration

The CLI tool automatically creates a configuration file at `~/.config/wiim-control/config.toml` on first run.

Basic configuration:
```toml
device_ip = "192.168.1.100"
```

## Basic Commands

### Status Information

```bash
wiim-control status                    # Show current track and playback status
```

### Playback Control

```bash
wiim-control play                      # Resume playback
wiim-control pause                     # Pause playback
wiim-control toggle                    # Toggle play/pause
wiim-control stop                      # Stop playback
wiim-control next                      # Next track
wiim-control prev                      # Previous track
```

### Volume Control

```bash
wiim-control volume 75                 # Set volume to 75%
wiim-control volume-up                 # Increase volume by 5% (default)
wiim-control volume-up 10              # Increase volume by 10%
wiim-control volume-down               # Decrease volume by 5% (default)
wiim-control volume-down 10            # Decrease volume by 10%
wiim-control mute                      # Mute audio
wiim-control unmute                    # Unmute audio
```

### Device Configuration

```bash
wiim-control --device 192.168.1.101 status    # Override device IP
wiim-control --config /path/to/config.toml    # Use custom config file
```

## Template System

The CLI tool supports flexible output formatting through templates, enabling integration with status bars and custom automation.

### Basic Usage

```bash
# Default text output
wiim-control status

# JSON output for status bars
wiim-control status --format json

# Profile-based output
wiim-control status --profile waybar
wiim-control status --profile polybar
wiim-control status --profile i3blocks
```

### Custom Templates

```bash
# Custom template
wiim-control --profile custom --template "{{artist}} - {{title}}" status

# With audio quality information
wiim-control --profile custom --template "{{artist}} - {{title}} {{quality_info}}" status

# Volume-focused display
wiim-control --profile custom --template "{{track_info}} | {{volume}}%" status
```

### Template Variables

Common template variables:
- `{{artist}}`, `{{title}}`, `{{album}}` - Track information
- `{{state}}` - Playback state (playing/paused/stopped/loading)
- `{{volume}}` - Volume level (0-100)
- `{{quality_info}}` - Audio quality (e.g., "192kHz/24bit")
- `{{track_info}}` - Smart artist-title combination with fallbacks

For complete template documentation, see [Template System Overview](docs/templates/README.md).

## Configuration

### Configuration File

Default location: `~/.config/wiim-control/config.toml`

```toml
device_ip = "192.168.1.100"

[output.text]
playing = "{{artist}} - {{title}} {{quality_info}}"
paused = "{{artist}} - {{title}}"
stopped = "No music"
loading = "Loading..."

[output.json]
text = "{{artist}} - {{title}}"
alt = "{{state}}"
tooltip = "{{full_info}}"
class = "{{state}}"
percentage = "{{volume}}"

[profiles.waybar]
format = "json"

[profiles.polybar]
format = "text"
text_template = "{{artist}} - {{title}} [{{quality_info}}]"

[profiles.i3blocks]
format = "text"
text_template = "{{track_info}} | {{volume}}%"
```

### Profile System

Profiles allow different output configurations for different tools:

```bash
# Use specific profile
wiim-control --profile waybar status

# Override template for any profile
wiim-control --profile polybar --template "{{track_info}} | {{volume}}%" status
```

## Status Bar Integration

The CLI tool integrates with popular status bars through the template system.

### Quick Setup

**Waybar:**
```bash
wiim-control --profile waybar status
```

**Polybar:**
```bash
wiim-control --profile polybar status
```

**i3blocks:**
```bash
wiim-control --profile i3blocks status
```

### Integration Guides

For detailed setup instructions:
- [Waybar Integration](docs/integrations/waybar.md)
- [Polybar Integration](docs/integrations/polybar.md)
- [i3blocks Integration](docs/integrations/i3blocks.md)

## Examples

### Basic Usage

```bash
# Check current status
wiim-control status

# Control playback
wiim-control toggle
wiim-control next
wiim-control volume-up
```

### Status Bar Integration

```bash
# Waybar module
wiim-control --profile waybar status

# Polybar module
wiim-control --profile polybar status

# Custom format
wiim-control --profile custom --template "♪ {{artist}} - {{title}}" status
```

### Automation Scripts

```bash
# Get just the artist name
wiim-control --profile custom --template "{{artist}}" status

# Get current volume
wiim-control --profile custom --template "{{volume}}" status

# Check if playing
wiim-control --profile custom --template "{{state}}" status
```

## Troubleshooting

### Common Issues

**CLI tool not found:**
- Ensure `cargo install` completed successfully
- Check that `~/.cargo/bin` is in your PATH

**Device not responding:**
- Verify the device IP address
- Check network connectivity
- Ensure the WiiM device is powered on

**Template errors:**
- Check template syntax (use `{{variable}}` format)
- Verify variable names against the [Template Variables Reference](docs/templates/variables.md)
- Test with simple templates first

**Configuration issues:**
- Check config file syntax with `wiim-control --config /path/to/config.toml status`
- Verify file permissions on config directory
- Reset to defaults by removing config file

### Testing Commands

```bash
# Test basic connectivity
wiim-control status

# Test specific device
wiim-control --device 192.168.1.100 status

# Test configuration
wiim-control --config ~/.config/wiim-control/config.toml status

# Test template syntax
wiim-control --profile custom --template "{{artist}} - {{title}}" status
```

## See Also

- [WiiM API Library Documentation](README.md) - Main library documentation
- [Template System Overview](docs/templates/README.md) - Detailed template documentation
- [Template Variables Reference](docs/templates/variables.md) - Complete variable list
- [Configuration Examples](docs/templates/examples.md) - Sample configurations
- [Integration Guides](docs/integrations/) - Status bar setup guides
