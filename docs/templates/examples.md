# Configuration Examples

This document provides comprehensive configuration examples for the WiiM API template system, covering various use cases and status bar integrations.

## Basic Configuration

### Complete Config File Example

```toml
# ~/.config/wiim-control/config.toml
device_ip = "192.168.1.100"

# Default text output templates
[output.text]
playing = "▶️ {artist} - {title} {quality_info}"
paused = "⏸️ {artist} - {title}"
stopped = "⏹️ No music"
loading = "⏳ Loading..."

# Default JSON output templates
[output.json]
text = "{artist} - {title}"
alt = "{state}"
tooltip = "{full_info}"
class = "{state}"
percentage = "{volume}"

# Status bar profiles
[profiles.waybar]
format = "json"
# Uses default json templates

[profiles.polybar]
format = "text"
text_template = "{artist} - {title} [{quality_info}]"

[profiles.i3blocks]
format = "text"
text_template = "{track_info} | {volume}%"

[profiles.minimal]
format = "text"
text_template = "{track_info}"

[profiles.audiophile]
format = "text"
text_template = "♪ {artist} - {title} • {quality_info} • {volume}%"
```

## Text Format Examples

### Basic Text Templates

```toml
[output.text]
# Simple with icons
playing = "▶️ {track_info}"
paused = "⏸️ {track_info}"
stopped = "⏹️ No music"
loading = "⏳ Loading..."

# With quality information
playing = "▶️ {artist} - {title} {quality_info}"
paused = "⏸️ {artist} - {title}"
stopped = "⏹️ No music"
loading = "⏳ Loading..."

# Volume included
playing = "▶️ {artist} - {title} | {volume}%"
paused = "⏸️ {artist} - {title} | {volume}%"
stopped = "⏹️ No music | {volume}%"
loading = "⏳ Loading... | {volume}%"

# Minimal format
playing = "{artist} - {title}"
paused = "{artist} - {title} [PAUSED]"
stopped = "No music"
loading = "Loading..."
```

### Advanced Text Templates

```toml
[output.text]
# Audiophile format with detailed quality
playing = "♪ {artist} - {title} • {sample_rate_khz}/{bit_depth_bit} • Vol: {volume}%"
paused = "⏸ {artist} - {title} • {sample_rate_khz}/{bit_depth_bit} • Vol: {volume}%"
stopped = "⏹ Stopped • Vol: {volume}%"
loading = "⏳ Loading..."

# Compact format
playing = "{artist} - {title} ({quality_info})"
paused = "{artist} - {title} [⏸]"
stopped = "[⏹]"
loading = "[⏳]"

# Position information
playing = "▶️ {artist} - {title} • {position}/{duration}"
paused = "⏸️ {artist} - {title} • {position}/{duration}"
stopped = "⏹️ No music"
loading = "⏳ Loading..."
```

## JSON Format Examples

### Basic JSON Templates

```toml
[output.json]
# Standard format
text = "{artist} - {title}"
alt = "{state}"
tooltip = "{full_info}"
class = "{state}"
percentage = "{volume}"

# Track info focus
text = "{track_info}"
alt = "{state}"
tooltip = "{artist} - {title}\nAlbum: {album}\nVolume: {volume}%"
class = "{state}"
percentage = "{volume}"

# Quality focus
text = "{artist} - {title} {quality_info}"
alt = "{state}"
tooltip = "{full_info}"
class = "music-{state}"
percentage = "{volume}"
```

### Advanced JSON Templates

```toml
[output.json]
# Audiophile format
text = "{artist} - {title}"
alt = "{quality_info}"
tooltip = "{title}\nArtist: {artist}\nAlbum: {album}\nQuality: {quality_info}\nVolume: {volume}%\nPosition: {position}/{duration}"
class = "music-{state}"
percentage = "{volume}"

# Compact format
text = "{track_info}"
alt = "{volume}%"
tooltip = "{full_info}"
class = "{state}"
percentage = "{volume}"

# Position-aware format
text = "{artist} - {title} ({position})"
alt = "{state}"
tooltip = "{full_info}"
class = "music-{state}"
percentage = "{volume}"
```

## Profile-Based Examples

### Waybar Profile

```toml
[profiles.waybar]
format = "json"
# Uses default json templates

# Alternative waybar with custom templates
[profiles.waybar-custom]
format = "json"
json_template = "custom"

[output.json]
text = "{track_info}"
alt = "{state}"
tooltip = "{full_info}"
class = "music-{state}"
percentage = "{volume}"
```

### Polybar Profile

```toml
[profiles.polybar]
format = "text"
text_template = "{artist} - {title} [{quality_info}]"

# Alternative polybar formats
[profiles.polybar-minimal]
format = "text"
text_template = "{track_info}"

[profiles.polybar-detailed]
format = "text"
text_template = "♪ {artist} - {title} • {quality_info} • {volume}%"

[profiles.polybar-compact]
format = "text"
text_template = "{artist} - {title} ({position})"
```

### i3blocks Profile

```toml
[profiles.i3blocks]
format = "text"
text_template = "{track_info} | {volume}%"

# Alternative i3blocks formats
[profiles.i3blocks-quality]
format = "text"
text_template = "{track_info} | {quality_info}"

[profiles.i3blocks-detailed]
format = "text"
text_template = "♪ {artist} - {title} • {volume}% • {quality_info}"
```

## Specialized Use Cases

### Notification System

```toml
[profiles.notify]
format = "text"
text_template = "♪ Now Playing: {artist} - {title}"

[profiles.notify-detailed]
format = "text"
text_template = "♪ {artist} - {title}\n🎵 Album: {album}\n🔊 Volume: {volume}%\n📊 Quality: {quality_info}"
```

### Terminal Display

```toml
[profiles.terminal]
format = "text"
text_template = "🎵 {artist} - {title} | 🔊 {volume}% | 📊 {quality_info}"

[profiles.terminal-simple]
format = "text"
text_template = "{track_info} ({volume}%)"

[profiles.terminal-verbose]
format = "text"
text_template = "🎵 Now Playing: {artist} - {title}\n📀 Album: {album}\n🔊 Volume: {volume}% {muted}\n📊 Quality: {quality_info}\n⏱️ Time: {position}/{duration}"
```

### Automation Scripts

```toml
[profiles.script-artist]
format = "text"
text_template = "{artist}"

[profiles.script-title]
format = "text"
text_template = "{title}"

[profiles.script-quality]
format = "text"
text_template = "{quality_info}"

[profiles.script-status]
format = "text"
text_template = "{state}"
```

## Theme-Based Examples

### Minimalist Theme

```toml
[profiles.minimal]
format = "text"
text_template = "{track_info}"

[profiles.minimal-json]
format = "json"

[output.json]
text = "{track_info}"
alt = ""
tooltip = "{artist} - {title}"
class = ""
percentage = "{volume}"
```

### Detailed Theme

```toml
[profiles.detailed]
format = "text"
text_template = "🎵 {artist} - {title} | 📊 {quality_info} | 🔊 {volume}% | ⏱️ {position}/{duration}"

[profiles.detailed-json]
format = "json"

[output.json]
text = "{artist} - {title} [{quality_info}]"
alt = "{state} • {volume}%"
tooltip = "{full_info}"
class = "music-{state}"
percentage = "{volume}"
```

### Icon-Heavy Theme

```toml
[output.text]
playing = "▶️ {artist} - {title} 🎵 {quality_info} 🔊 {volume}%"
paused = "⏸️ {artist} - {title} 🎵 {quality_info} 🔊 {volume}%"
stopped = "⏹️ No music 🔊 {volume}%"
loading = "⏳ Loading... 🔊 {volume}%"

[profiles.icons]
format = "text"
# Uses the icon-heavy text templates above
```

## Multi-Device Setup

```toml
# Main living room device
device_ip = "192.168.1.100"

# Alternative device configurations
[profiles.kitchen]
device_ip = "192.168.1.101"
format = "text"
text_template = "🍳 {track_info}"

[profiles.bedroom]
device_ip = "192.168.1.102"
format = "text"
text_template = "🛏️ {track_info} | {volume}%"

[profiles.office]
device_ip = "192.168.1.103"
format = "json"
# Uses default json templates
```

## Error Handling Examples

### Graceful Degradation

```toml
[output.text]
# These templates work even when some data is missing
playing = "{artist} - {title} {quality_info}"  # Shows partial info if available
paused = "{track_info}"                        # Falls back to smart combination
stopped = "No music"                           # Static text when nothing is playing
loading = "Loading..."                         # Static text during loading

[output.json]
# JSON templates with fallbacks
text = "{track_info}"                          # Smart fallback for missing artist/title
alt = "{state}"                                # Always available
tooltip = "{full_info}"                        # Pre-formatted with all available info
class = "{state}"                              # Always available
percentage = "{volume}"                        # Always available
```

### Data Validation

```toml
[profiles.safe]
format = "text"
# This template is designed to always produce valid output
text_template = "{track_info}"

[profiles.safe-json]
format = "json"
# These templates ensure valid JSON output
text = "{track_info}"
alt = "{state}"
tooltip = "{title} - {artist}"
class = "music"
percentage = "{volume}"
```

## Performance Optimized Examples

### Efficient Templates

```toml
# Fast templates using pre-computed combinations
[profiles.fast]
format = "text"
text_template = "{track_info}"  # Pre-computed, no template processing needed

[profiles.fast-json]
format = "json"
text = "{track_info}"           # Pre-computed
alt = "{state}"                 # Simple variable
tooltip = "{full_info}"         # Pre-computed
class = "{state}"               # Simple variable
percentage = "{volume}"         # Simple variable
```

### Minimal Processing

```toml
# Templates that require minimal processing
[profiles.minimal-processing]
format = "text"
text_template = "{artist} - {title}"

[profiles.minimal-json]
format = "json"
text = "{artist} - {title}"
alt = "{state}"
tooltip = "{artist} - {title}"
class = "{state}"
percentage = "{volume}"
```

## Testing Examples

### Debug Configuration

```toml
[profiles.debug]
format = "text"
text_template = "A:{artist} T:{title} Q:{quality_info} V:{volume} S:{state}"

[profiles.debug-json]
format = "json"
text = "Debug: {artist} - {title}"
alt = "State: {state}, Vol: {volume}"
tooltip = "{full_info}"
class = "debug-{state}"
percentage = "{volume}"
```

### Variable Testing

```toml
[profiles.test-all]
format = "text"
text_template = "Artist:{artist}|Title:{title}|Album:{album}|State:{state}|Vol:{volume}|Quality:{quality_info}"

[profiles.test-quality]
format = "text"
text_template = "SR:{sample_rate}|BD:{bit_depth}|SRK:{sample_rate_khz}|BDB:{bit_depth_bit}|QI:{quality_info}"

[profiles.test-timing]
format = "text"
text_template = "Pos:{position}|Dur:{duration}|PosMS:{position_ms}|DurMS:{duration_ms}"
```

## Usage Examples

### Command Line Usage

```bash
# Use specific profile
wiim-control status --profile waybar

# Override template
wiim-control status --profile polybar --template "{artist} - {title}"

# Test configuration
wiim-control status --profile debug

# Quality information only
wiim-control status --profile custom --template "{quality_info}"
```

### Integration Examples

```bash
# Waybar configuration
wiim-control status --profile waybar

# Polybar configuration
wiim-control status --profile polybar

# i3blocks configuration
wiim-control status --profile i3blocks

# Custom notification
wiim-control status --profile notify
```

## See Also

- [Template Variables Reference](variables.md) - Complete variable documentation
- [Template System Overview](README.md) - System overview and concepts
- [Audio Quality Guide](audio-quality.md) - Understanding quality indicators
- [Integration Guides](../integrations/) - Status bar setup guides
