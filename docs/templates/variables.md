# Template Variables Reference

This document provides a comprehensive reference of all template variables available in the WiiM API template system.

## Quick Reference

Use these variables in your templates by wrapping them in double curly braces: `{variable_name}`

### Track Information

| Variable | Type | Description | Example |
|----------|------|-------------|---------|
| `{artist}` | Optional String | Track artist name | `"The Beatles"` |
| `{title}` | Optional String | Track title | `"Hey Jude"` |
| `{album}` | Optional String | Album name | `"The Beatles 1967-1970"` |
| `{album_art_uri}` | Optional String | Album cover art URL | `"https://example.com/art.jpg"` |

### Playback State

| Variable | Type | Description | Example |
|----------|------|-------------|---------|
| `{state}` | String | Current playback state | `"playing"`, `"paused"`, `"stopped"`, `"loading"` |
| `{volume}` | Number | Volume level (0-100) | `75` |
| `{muted}` | Boolean | Mute status | `true`, `false` |
| `{position}` | String | Current position (formatted) | `"3:45"` |
| `{duration}` | String | Total duration (formatted) | `"4:32"` |
| `{position_ms}` | Number | Current position in milliseconds | `225000` |
| `{duration_ms}` | Number | Total duration in milliseconds | `272000` |

### Audio Quality

| Variable | Type | Description | Example |
|----------|------|-------------|---------|
| `{sample_rate}` | Optional String | Raw sample rate in Hz | `"192000"` |
| `{bit_depth}` | Optional String | Raw bit depth | `"24"` |
| `{sample_rate_khz}` | Optional String | Formatted sample rate | `"192kHz"` |
| `{bit_depth_bit}` | Optional String | Formatted bit depth | `"24bit"` |
| `{quality_info}` | Optional String | Combined quality information | `"192kHz/24bit"` |

### Network Quality (when available)

| Variable | Type | Description | Example |
|----------|------|-------------|---------|
| `{wifi_signal}` | Optional String | WiFi signal strength (RSSI) | `"-30 dBm"` |
| `{wifi_rate}` | Optional String | WiFi data rate | `"390 Mbps"` |
| `{bitrate}` | Optional String | Stream bitrate | `"5719 kbps"` |
| `{network_quality}` | Optional String | Calculated network quality indicator | `"Excellent"` |

### Formatted Combinations

| Variable | Type | Description | Example |
|----------|------|-------------|---------|
| `{track_info}` | String | Smart artist-title combination | `"The Beatles - Hey Jude"` |
| `{full_info}` | String | Complete information for tooltips | Multi-line formatted text |

## Variable Details

### Track Information Variables

#### `{artist}`
- **Type**: Optional String
- **Description**: The artist name for the current track
- **Fallback**: Empty string if not available
- **Example**: `"The Beatles"`, `"Miles Davis"`

#### `{title}`
- **Type**: Optional String
- **Description**: The title of the current track
- **Fallback**: Empty string if not available
- **Example**: `"Hey Jude"`, `"Kind of Blue"`

#### `{album}`
- **Type**: Optional String
- **Description**: The album name for the current track
- **Fallback**: Empty string if not available
- **Example**: `"The Beatles 1967-1970"`, `"Kind of Blue"`

#### `{album_art_uri}`
- **Type**: Optional String
- **Description**: URL to the album cover art image
- **Fallback**: Empty string if not available
- **Example**: `"https://example.com/covers/album.jpg"`

### Playback State Variables

#### `{state}`
- **Type**: String
- **Description**: Current playback state
- **Possible Values**:
  - `"playing"` - Currently playing music
  - `"paused"` - Playback is paused
  - `"stopped"` - Playback is stopped
  - `"loading"` - Loading new content
- **Example**: `"playing"`

#### `{volume}`
- **Type**: Number (0-100)
- **Description**: Current volume level as a percentage
- **Example**: `75` (for 75% volume)

#### `{muted}`
- **Type**: Boolean
- **Description**: Whether audio is currently muted
- **Values**: `true` or `false`
- **Example**: `false`

#### `{position}` and `{duration}`
- **Type**: String
- **Description**: Formatted time strings in MM:SS format
- **Example**: `"3:45"` (3 minutes, 45 seconds)
- **Note**: Shows `"0:00"` if time is unavailable

#### `{position_ms}` and `{duration_ms}`
- **Type**: Number
- **Description**: Raw time values in milliseconds
- **Example**: `225000` (225 seconds = 3:45)
- **Use Case**: Useful for calculating percentages or custom formatting

### Audio Quality Variables

#### `{sample_rate}`
- **Type**: Optional String
- **Description**: Raw sample rate in Hz
- **Example**: `"192000"` (192 kHz)
- **Common Values**: `"44100"`, `"48000"`, `"96000"`, `"192000"`

#### `{bit_depth}`
- **Type**: Optional String
- **Description**: Raw bit depth value
- **Example**: `"24"` (24-bit)
- **Common Values**: `"16"`, `"24"`, `"32"`

#### `{sample_rate_khz}`
- **Type**: Optional String
- **Description**: Formatted sample rate with kHz suffix
- **Example**: `"192kHz"`
- **Note**: Automatically converts Hz to kHz and formats nicely

#### `{bit_depth_bit}`
- **Type**: Optional String
- **Description**: Formatted bit depth with "bit" suffix
- **Example**: `"24bit"`

#### `{quality_info}`
- **Type**: Optional String
- **Description**: Combined sample rate and bit depth information
- **Example**: `"192kHz/24bit"`
- **Note**: Only available when both sample rate and bit depth are present

### Formatted Combinations

#### `{track_info}`
- **Type**: String
- **Description**: Smart combination of artist and title with fallbacks
- **Logic**:
  - If both artist and title available: `"Artist - Title"`
  - If only artist available: `"Artist"`
  - If only title available: `"Title"`
  - If only album available: `"Album"`
  - If none available: `"No track info"`
- **Example**: `"The Beatles - Hey Jude"`

#### `{full_info}`
- **Type**: String
- **Description**: Complete formatted information suitable for tooltips
- **Contains**:
  - Track information (title, artist, album)
  - Volume and mute status
  - Audio quality information
  - Playback position and duration
- **Format**: Multi-line text with newline separators
- **Example**:
  ```
  Title: Hey Jude
  Artist: The Beatles
  Album: The Beatles 1967-1970
  Volume: 75%
  Quality: 192kHz/24bit
  Time: 3:45 / 7:11
  ```

## Template Examples

### Basic Track Display
```
{artist} - {title}
```
**Output**: `"The Beatles - Hey Jude"`

### With Quality Information
```
{artist} - {title} {quality_info}
```
**Output**: `"The Beatles - Hey Jude 192kHz/24bit"`

### Status Bar Format
```
▶️ {track_info} | {volume}%
```
**Output**: `"▶️ The Beatles - Hey Jude | 75%"`

### Conditional Display
Templates automatically handle missing fields:
```
{artist} - {title}
```
- If artist is missing: `" - Hey Jude"`
- If title is missing: `"The Beatles - "`
- If both missing: `" - "`

### Multi-line Tooltip
```
{title}
Artist: {artist}
Volume: {volume}%
```
**Output**:
```
Hey Jude
Artist: The Beatles
Volume: 75%
```

## Advanced Usage

### Handling Missing Data
All optional variables gracefully handle missing data by returning empty strings. Your templates should be designed to work with partial information.

### Performance Considerations
- Template variables are pre-computed for efficiency
- Complex formatting is done once per status update
- Network quality variables may not always be available

### Template Validation
The system validates template syntax at runtime. Invalid templates will result in error messages indicating the issue.

## See Also

- [Template System Overview](README.md)
- [Configuration Examples](examples.md)
- [Audio Quality Guide](audio-quality.md)
- [Integration Guides](../integrations/)
