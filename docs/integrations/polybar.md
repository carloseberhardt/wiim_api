# Polybar Integration Guide

This guide shows how to integrate WiiM API with Polybar, a fast and easy-to-use status bar for Linux.

## Quick Setup

### 1. Install WiiM Control CLI

```bash
# Install from source
cargo install --path . --bin wiim-control

# Or download from releases
# [Installation instructions will be added when releases are available]
```

### 2. Basic Polybar Configuration

Add this to your `~/.config/polybar/config`:

```ini
[module/music]
type = custom/script
exec = wiim-control --profile polybar status
interval = 1
format = <label>
label = %output%
label-maxlen = 50
click-left = wiim-control toggle
click-right = wiim-control next
click-middle = wiim-control prev
scroll-up = wiim-control volume-up
scroll-down = wiim-control volume-down
```

### 3. Add to Bar

```ini
[bar/main]
modules-left = bspwm i3
modules-center = music
modules-right = network pulseaudio date
```

### 4. Basic Template Configuration

Create `~/.config/wiim-control/config.toml`:

```toml
device_ip = "192.168.1.100"

[profiles.polybar]
format = "text"
text_template = "{artist} - {title} [{quality_info}]"
```

## Advanced Configuration

### Custom Templates

#### Minimal Display
```toml
[profiles.polybar-minimal]
format = "text"
text_template = "{track_info}"
```

#### Detailed Display
```toml
[profiles.polybar-detailed]
format = "text"
text_template = "♪ {artist} - {title} • {quality_info} • {volume}%"
```

#### Status Icons
```toml
[profiles.polybar-icons]
format = "text"
text_template = "%{F#a3be8c}♪%{F-} {artist} - {title} %{F#88c0d0}[{quality_info}]%{F-}"
```

### Advanced Polybar Module

```ini
[module/music]
type = custom/script
exec = wiim-control --profile polybar status
interval = 1
format = <label>
format-prefix = "♪ "
format-prefix-foreground = #a3be8c
label = %output%
label-maxlen = 60
label-ellipsis = true
click-left = wiim-control toggle
click-right = wiim-control next
click-middle = wiim-control prev
scroll-up = wiim-control volume-up 5
scroll-down = wiim-control volume-down 5
```

## Styling Options

### Color Schemes

#### Nord Theme
```ini
[module/music]
type = custom/script
exec = wiim-control --profile polybar status
interval = 1
format = <label>
format-prefix = "♪ "
format-prefix-foreground = #a3be8c
label = %output%
label-foreground = #d8dee9
label-maxlen = 50
```

#### Gruvbox Theme
```ini
[module/music]
type = custom/script
exec = wiim-control --profile polybar status
interval = 1
format = <label>
format-prefix = "♪ "
format-prefix-foreground = #b8bb26
label = %output%
label-foreground = #ebdbb2
label-maxlen = 50
```

#### Dracula Theme
```ini
[module/music]
type = custom/script
exec = wiim-control --profile polybar status
interval = 1
format = <label>
format-prefix = "♪ "
format-prefix-foreground = #50fa7b
label = %output%
label-foreground = #f8f8f2
label-maxlen = 50
```

### Custom Formatting

#### With Polybar Formatting
```toml
[profiles.polybar-styled]
format = "text"
text_template = "%{F#a3be8c}♪%{F-} {artist} %{F#88c0d0}-%{F-} {title} %{F#d08770}[{quality_info}]%{F-}"
```

#### With Background Colors
```toml
[profiles.polybar-bg]
format = "text"
text_template = "%{B#3b4252}%{F#d8dee9} {artist} - {title} %{B-}%{F-}"
```

#### With Clickable Areas
```toml
[profiles.polybar-clickable]
format = "text"
text_template = "%{A1:wiim-control toggle:}%{A3:wiim-control next:}♪ {artist} - {title}%{A}%{A}"
```

## State-Based Styling

### Script-Based State Handling

Create `~/.config/polybar/scripts/wiim-music.sh`:

```bash
#!/bin/bash

# Get current status
OUTPUT=$(wiim-control --profile polybar status)
STATE=$(wiim-control status --profile custom --template "{state}")

# Apply styling based on state
case "$STATE" in
    "playing")
        echo "%{F#a3be8c}▶%{F-} $OUTPUT"
        ;;
    "paused")
        echo "%{F#ebcb8b}⏸%{F-} $OUTPUT"
        ;;
    "stopped")
        echo "%{F#d08770}⏹%{F-} No music"
        ;;
    "loading")
        echo "%{F#88c0d0}⏳%{F-} Loading..."
        ;;
    *)
        echo "$OUTPUT"
        ;;
esac
```

### Polybar Configuration for Script

```ini
[module/music]
type = custom/script
exec = ~/.config/polybar/scripts/wiim-music.sh
interval = 1
format = <label>
label = %output%
label-maxlen = 50
click-left = wiim-control toggle
click-right = wiim-control next
click-middle = wiim-control prev
scroll-up = wiim-control volume-up
scroll-down = wiim-control volume-down
```

## Interactive Features

### Click Actions

```ini
[module/music]
type = custom/script
exec = wiim-control --profile polybar status
interval = 1
format = <label>
label = %output%
click-left = wiim-control toggle
click-right = wiim-control next
click-middle = wiim-control prev
scroll-up = wiim-control volume-up 5
scroll-down = wiim-control volume-down 5
double-click-left = wiim-control stop
```

### Advanced Click Handling

Create `~/.config/polybar/scripts/wiim-control.sh`:

```bash
#!/bin/bash

case "$1" in
    "toggle")
        wiim-control toggle
        notify-send "WiiM" "Toggled playback"
        ;;
    "next")
        wiim-control next
        TRACK=$(wiim-control status --profile custom --template "{track_info}")
        notify-send "WiiM" "Next: $TRACK"
        ;;
    "prev")
        wiim-control prev
        TRACK=$(wiim-control status --profile custom --template "{track_info}")
        notify-send "WiiM" "Previous: $TRACK"
        ;;
    "volume-up")
        NEW_VOL=$(wiim-control volume-up 5)
        notify-send "WiiM" "Volume: $NEW_VOL%"
        ;;
    "volume-down")
        NEW_VOL=$(wiim-control volume-down 5)
        notify-send "WiiM" "Volume: $NEW_VOL%"
        ;;
esac
```

### Polybar Configuration for Advanced Script

```ini
[module/music]
type = custom/script
exec = wiim-control --profile polybar status
interval = 1
format = <label>
label = %output%
click-left = ~/.config/polybar/scripts/wiim-control.sh toggle
click-right = ~/.config/polybar/scripts/wiim-control.sh next
click-middle = ~/.config/polybar/scripts/wiim-control.sh prev
scroll-up = ~/.config/polybar/scripts/wiim-control.sh volume-up
scroll-down = ~/.config/polybar/scripts/wiim-control.sh volume-down
```

## Multiple Devices

### Device-Specific Modules

```ini
[module/music-main]
type = custom/script
exec = wiim-control --profile polybar status
interval = 1
format = <label>
format-prefix = "🏠 "
label = %output%
label-maxlen = 40
click-left = wiim-control toggle

[module/music-kitchen]
type = custom/script
exec = wiim-control --profile polybar-kitchen status
interval = 1
format = <label>
format-prefix = "🍳 "
label = %output%
label-maxlen = 40
click-left = wiim-control --profile polybar-kitchen toggle
```

### Configuration for Multiple Devices

```toml
device_ip = "192.168.1.100"

[profiles.polybar]
format = "text"
text_template = "{artist} - {title} [{quality_info}]"

[profiles.polybar-kitchen]
device_ip = "192.168.1.101"
format = "text"
text_template = "{artist} - {title}"

[profiles.polybar-bedroom]
device_ip = "192.168.1.102"
format = "text"
text_template = "{track_info}"
```

## Animation and Effects

### Scrolling Text Effect

```bash
#!/bin/bash
# ~/.config/polybar/scripts/wiim-scroll.sh

MAX_LENGTH=30
TEXT=$(wiim-control --profile polybar status)
TEXT_LENGTH=${#TEXT}

if [ $TEXT_LENGTH -gt $MAX_LENGTH ]; then
    # Create scrolling effect
    SCROLL_POS=$(( $(date +%s) % ($TEXT_LENGTH + 10) ))
    if [ $SCROLL_POS -lt $MAX_LENGTH ]; then
        echo "${TEXT:0:$MAX_LENGTH}"
    else
        START_POS=$(( $SCROLL_POS - $MAX_LENGTH ))
        echo "${TEXT:$START_POS:$MAX_LENGTH}"
    fi
else
    echo "$TEXT"
fi
```

### Blinking Effect for Playing

```bash
#!/bin/bash
# ~/.config/polybar/scripts/wiim-blink.sh

STATE=$(wiim-control status --profile custom --template "{state}")
OUTPUT=$(wiim-control --profile polybar status)

if [ "$STATE" = "playing" ]; then
    SECOND=$(date +%s)
    if [ $((SECOND % 2)) -eq 0 ]; then
        echo "%{F#a3be8c}♪%{F-} $OUTPUT"
    else
        echo "%{F#88c0d0}♪%{F-} $OUTPUT"
    fi
else
    echo "$OUTPUT"
fi
```

## Performance Optimization

### Efficient Updates

```ini
[module/music]
type = custom/script
exec = wiim-control --profile polybar status
interval = 2
format = <label>
label = %output%
; Only update when content changes
exec-if = pgrep -x "wiim-control" > /dev/null
```

### Conditional Updates

```bash
#!/bin/bash
# ~/.config/polybar/scripts/wiim-efficient.sh

# Cache file for last known state
CACHE_FILE="/tmp/wiim-polybar-cache"
CURRENT_STATE=$(wiim-control status --profile custom --template "{state}")

# Only update if state changed or if playing
if [ ! -f "$CACHE_FILE" ] || [ "$(cat $CACHE_FILE)" != "$CURRENT_STATE" ] || [ "$CURRENT_STATE" = "playing" ]; then
    echo "$CURRENT_STATE" > "$CACHE_FILE"
    wiim-control --profile polybar status
else
    # Return cached output for stopped/paused states
    echo "No music"
fi
```

## Theme Integration

### Material Design

```ini
[module/music]
type = custom/script
exec = wiim-control --profile polybar status
interval = 1
format = <label>
format-background = #2196f3
format-foreground = #ffffff
format-padding = 2
format-margin = 1
label = %output%
label-maxlen = 45
```

### Minimal Theme

```ini
[module/music]
type = custom/script
exec = wiim-control --profile polybar status
interval = 1
format = <label>
format-prefix = "♪ "
format-prefix-foreground = #666
label = %output%
label-foreground = #333
label-maxlen = 50
```

### Glass Effect

```ini
[module/music]
type = custom/script
exec = wiim-control --profile polybar status
interval = 1
format = <label>
format-background = #80000000
format-foreground = #ffffff
format-padding = 2
format-margin = 1
label = %output%
label-maxlen = 45
```

## Troubleshooting

### Common Issues

#### Module Not Updating
- Check if `wiim-control` is in your PATH
- Verify the device IP address in config
- Test manually: `wiim-control --profile polybar status`

#### Text Formatting Issues
- Ensure Polybar formatting codes are properly escaped
- Test template syntax: `wiim-control --profile polybar status`
- Check for special characters in track names

#### Performance Issues
- Increase interval if updates are too frequent
- Use efficient update scripts
- Consider caching for stopped/paused states

### Debug Commands

```bash
# Test basic functionality
wiim-control status

# Test polybar profile
wiim-control --profile polybar status

# Test template rendering
wiim-control status --profile custom --template "{artist} - {title}"

# Check device connectivity
wiim-control --device 192.168.1.100 status
```

## Complete Example

### Full Configuration

**`~/.config/wiim-control/config.toml`**:
```toml
device_ip = "192.168.1.100"

[profiles.polybar]
format = "text"
text_template = "♪ {artist} - {title} • {quality_info} • {volume}%"
```

**`~/.config/polybar/config`**:
```ini
[bar/main]
width = 100%
height = 27
background = #282a36
foreground = #f8f8f2
modules-left = bspwm
modules-center = music
modules-right = network pulseaudio date

[module/music]
type = custom/script
exec = wiim-control --profile polybar status
interval = 1
format = <label>
format-prefix = "♪ "
format-prefix-foreground = #50fa7b
label = %output%
label-foreground = #f8f8f2
label-maxlen = 50
click-left = wiim-control toggle
click-right = wiim-control next
click-middle = wiim-control prev
scroll-up = wiim-control volume-up
scroll-down = wiim-control volume-down
```

## See Also

- [Template System Overview](../templates/README.md)
- [Template Variables Reference](../templates/variables.md)
- [Configuration Examples](../templates/examples.md)
- [Waybar Integration](waybar.md)
- [i3blocks Integration](i3blocks.md)
