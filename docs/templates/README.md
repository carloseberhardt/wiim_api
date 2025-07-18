# Template System Overview

The WiiM API template system provides powerful, flexible output formatting for the `wiim-control` CLI tool. This system allows you to customize how track information, playback status, and audio quality data are displayed in status bars, terminals, and other applications.

## Features

- **Handlebars templating engine** for powerful formatting
- **Rich template variables** including track info, playback state, and audio quality
- **Multiple output formats** (text and JSON)
- **Profile-based configuration** for different status bar tools
- **Smart fallbacks** for missing data
- **Pre-formatted combinations** for common use cases

## Quick Start

### 1. Basic Usage

By default, `wiim-control` uses simple text output:

```bash
wiim-control status
```
**Output**: `"▶️ The Beatles - Hey Jude"`

### 2. JSON Output for Status Bars

For status bar integration, use JSON format:

```bash
wiim-control status --format json
```
**Output**:
```json
{
  "text": "The Beatles - Hey Jude",
  "alt": "playing",
  "tooltip": "Title: Hey Jude\nArtist: The Beatles\nVolume: 75%",
  "class": "playing",
  "percentage": 75
}
```

### 3. Custom Templates

Create custom templates using template variables:

```bash
wiim-control status --profile custom --template "{artist} - {title} {quality_info}"
```
**Output**: `"The Beatles - Hey Jude 192kHz/24bit"`

## Configuration

### Configuration File Location

Templates are configured in `~/.config/wiim-control/config.toml`. The file is automatically created with defaults on first run.

### Basic Configuration Structure

```toml
device_ip = "192.168.1.100"

[output.text]
playing = "▶️ {artist} - {title} {quality_info}"
paused = "⏸️ {artist} - {title}"
stopped = "⏹️ No music"
loading = "⏳ Loading..."

[output.json]
text = "{artist} - {title}"
alt = "{state}"
tooltip = "{full_info}"
class = "{state}"

[profiles.waybar]
format = "json"
# Uses default json template

[profiles.polybar]
format = "text"
text_template = "{artist} - {title} [{quality_info}]"
```

## Template Variables

The template system provides extensive variables for customization:

### Core Variables
- **Track Info**: `{artist}`, `{title}`, `{album}`, `{album_art_uri}`
- **Playback State**: `{state}`, `{volume}`, `{muted}`, `{position}`, `{duration}`
- **Audio Quality**: `{sample_rate}`, `{bit_depth}`, `{quality_info}`
- **Formatted Combinations**: `{track_info}`, `{full_info}`

For complete details, see the [Template Variables Reference](variables.md).

## Output Formats

### Text Format

Text format produces simple string output suitable for:
- Terminal display
- Simple status bars
- Scripts and automation

**Example Configuration**:
```toml
[output.text]
playing = "▶️ {artist} - {title} {quality_info}"
paused = "⏸️ {artist} - {title}"
stopped = "⏹️ No music"
loading = "⏳ Loading..."
```

### JSON Format

JSON format produces structured output compatible with:
- Waybar
- i3blocks
- Custom status bar implementations

**Example Configuration**:
```toml
[output.json]
text = "{artist} - {title}"
alt = "{state}"
tooltip = "{full_info}"
class = "{state}"
percentage = "{volume}"
```

**Output Structure**:
```json
{
  "text": "Main display text",
  "alt": "Alternative text",
  "tooltip": "Detailed tooltip text",
  "class": "CSS class name",
  "percentage": 75
}
```

## Profiles

Profiles allow you to define different configurations for different tools and use cases.

### Using Profiles

```bash
# Use a specific profile
wiim-control status --profile waybar

# Override template for any profile
wiim-control status --profile polybar --template "{track_info} | {volume}%"
```

### Profile Configuration

```toml
[profiles.waybar]
format = "json"
# Uses default json templates

[profiles.polybar]
format = "text"
text_template = "{artist} - {title} [{quality_info}]"

[profiles.i3blocks]
format = "text"
text_template = "{track_info} | {volume}%"
```

## Template Syntax

The template system uses Handlebars syntax:

### Basic Variable Substitution
```
{variable_name}
```

### Common Patterns

#### Artist and Title
```
{artist} - {title}
```

#### With Quality Information
```
{artist} - {title} {quality_info}
```

#### Status Icons
```
▶️ {track_info}  # Playing
⏸️ {track_info}  # Paused
⏹️ No music      # Stopped
⏳ Loading...     # Loading
```

#### Multi-line Tooltips
```
{title}
Artist: {artist}
Volume: {volume}%
Quality: {quality_info}
```

## Error Handling

### Template Validation
Invalid template syntax is caught at runtime with helpful error messages:

```bash
$ wiim-control status --template "{{invalid}"
Error: Invalid template syntax: Unclosed expression
```

### Missing Data
The system gracefully handles missing data:
- Optional variables return empty strings when unavailable
- Smart fallbacks provide reasonable defaults
- Templates continue to work with partial information

## Advanced Features

### Smart Fallbacks

The `{track_info}` variable provides intelligent fallbacks:
- If both artist and title available: `"Artist - Title"`
- If only artist available: `"Artist"`
- If only title available: `"Title"`
- If only album available: `"Album"`
- If none available: `"No track info"`

### Pre-formatted Combinations

Several variables provide pre-formatted combinations:
- `{quality_info}`: `"192kHz/24bit"`
- `{track_info}`: Smart artist-title combination
- `{full_info}`: Complete multi-line information

### Performance Optimization

- Template variables are pre-computed for efficiency
- Complex formatting is done once per status update
- Template compilation is cached for repeated use

## Migration from Legacy Format

If you're upgrading from a version without template support:

### Before (Legacy)
```bash
wiim-control status --format json
```

### After (Template System)
```bash
wiim-control status --profile waybar
```

The template system maintains backward compatibility while providing much more flexibility.

## Examples and Use Cases

### Status Bar Integration
See our integration guides:
- [Waybar Integration](../integrations/waybar.md)
- [Polybar Integration](../integrations/polybar.md)
- [i3blocks Integration](../integrations/i3blocks.md)

### Custom Automation
```bash
# Get just the artist name
wiim-control status --profile custom --template "{artist}"

# Get quality information
wiim-control status --profile custom --template "{quality_info}"

# Custom format for notifications
wiim-control status --profile custom --template "♪ {artist} - {title}"
```

## Troubleshooting

### Common Issues

1. **Template syntax errors**: Check for proper `{variable}` format
2. **Missing variables**: Verify variable names against the [reference](variables.md)
3. **Profile not found**: Check configuration file syntax and profile names
4. **Network data unavailable**: Some variables depend on network quality data

### Debug Information

To see all available template variables:
```bash
wiim-control status --format json
```

This shows the current data structure with all available fields.

## See Also

- [Template Variables Reference](variables.md) - Complete variable documentation
- [Configuration Examples](examples.md) - Sample configurations
- [Audio Quality Guide](audio-quality.md) - Understanding quality indicators
- [Integration Guides](../integrations/) - Status bar setup guides

## Getting Help

For support with the template system:
1. Check the [troubleshooting section](#troubleshooting)
2. Review the [examples](examples.md)
3. Test with simple templates first
4. Check the [GitHub issues](https://github.com/carloseberhardt/wiim_api/issues) for similar problems
