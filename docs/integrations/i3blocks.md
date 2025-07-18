# i3blocks Integration Guide

This guide shows how to integrate WiiM API with i3blocks, a flexible status line for the i3 window manager.

## Quick Setup

### 1. Install WiiM Control CLI

```bash
# Install from source
cargo install --path . --bin wiim-control

# Or download from releases
# [Installation instructions will be added when releases are available]
```

### 2. Basic i3blocks Configuration

Add this to your `~/.config/i3blocks/config`:

```ini
[music]
command=wiim-control --profile i3blocks status
interval=1
color=#a3be8c
signal=10
```

### 3. Template Configuration

Create `~/.config/wiim-control/config.toml`:

```toml
device_ip = "192.168.1.100"

[profiles.i3blocks]
format = "text"
text_template = "{track_info} | {volume}%"
```

### 4. Add to i3 Configuration

Add this to your `~/.config/i3/config`:

```
bar {
    status_command i3blocks -c ~/.config/i3blocks/config
    colors {
        statusline #ffffff
        background #323232
    }
}
```

## Advanced Configuration

### Custom Templates

#### Minimal Display
```toml
[profiles.i3blocks-minimal]
format = "text"
text_template = "{track_info}"
```

#### Detailed Display
```toml
[profiles.i3blocks-detailed]
format = "text"
text_template = "♪ {artist} - {title} • {quality_info} • {volume}%"
```

#### Quality-Focused
```toml
[profiles.i3blocks-quality]
format = "text"
text_template = "{track_info} | {quality_info}"
```

### Advanced i3blocks Block

```ini
[music]
command=wiim-control --profile i3blocks status
interval=1
label=♪
color=#a3be8c
min_width=200
align=center
signal=10
```

## Block Styling

### Basic Styling

```ini
[music]
command=wiim-control --profile i3blocks status
interval=1
label=♪
color=#a3be8c
background=#2e3440
border=#88c0d0
border_top=1
border_bottom=1
border_left=1
border_right=1
min_width=180
align=center
```

### State-Based Styling

Create `~/.config/i3blocks/scripts/wiim-music.sh`:

```bash
#!/bin/bash

# Get current status and state
OUTPUT=$(wiim-control --profile i3blocks status)
STATE=$(wiim-control status --profile custom --template "{state}")

# Set colors based on state
case "$STATE" in
    "playing")
        echo "$OUTPUT"
        echo "♪ $OUTPUT"
        echo "#a3be8c"
        ;;
    "paused")
        echo "$OUTPUT"
        echo "⏸ $OUTPUT"
        echo "#ebcb8b"
        ;;
    "stopped")
        echo "No music"
        echo "⏹ No music"
        echo "#d08770"
        ;;
    "loading")
        echo "Loading..."
        echo "⏳ Loading..."
        echo "#88c0d0"
        ;;
    *)
        echo "$OUTPUT"
        echo "$OUTPUT"
        echo "#e5e9f0"
        ;;
esac
```

### i3blocks Configuration for State Script

```ini
[music]
command=~/.config/i3blocks/scripts/wiim-music.sh
interval=1
signal=10
min_width=180
align=center
```

## Interactive Features

### Click Actions

```ini
[music]
command=wiim-control --profile i3blocks status
interval=1
signal=10
# Left click: toggle playback
# Right click: next track
# Middle click: previous track
# Scroll up: volume up
# Scroll down: volume down
```

### Click Handler Script

Create `~/.config/i3blocks/scripts/wiim-click.sh`:

```bash
#!/bin/bash

case $BLOCK_BUTTON in
    1) # Left click
        wiim-control toggle
        ;;
    2) # Middle click
        wiim-control prev
        ;;
    3) # Right click
        wiim-control next
        ;;
    4) # Scroll up
        wiim-control volume-up 5
        ;;
    5) # Scroll down
        wiim-control volume-down 5
        ;;
esac

# Refresh the block
pkill -RTMIN+10 i3blocks
```

### Configuration with Click Handler

```ini
[music]
command=~/.config/i3blocks/scripts/wiim-click.sh
interval=1
signal=10
min_width=180
align=center
```

## Status Updates

### Signal-Based Updates

```bash
#!/bin/bash
# ~/.config/i3blocks/scripts/wiim-status.sh

# Get current status
OUTPUT=$(wiim-control --profile i3blocks status)
STATE=$(wiim-control status --profile custom --template "{state}")

# Output for i3blocks
echo "$OUTPUT"
echo "$OUTPUT"

# Set color based on state
case "$STATE" in
    "playing")
        echo "#a3be8c"
        ;;
    "paused")
        echo "#ebcb8b"
        ;;
    "stopped")
        echo "#d08770"
        ;;
    *)
        echo "#e5e9f0"
        ;;
esac
```

### Real-time Updates

Create `~/.config/i3blocks/scripts/wiim-update.sh`:

```bash
#!/bin/bash

# Monitor for changes and update block
while true; do
    CURRENT_TRACK=$(wiim-control status --profile custom --template "{track_info}")
    if [ "$CURRENT_TRACK" != "$LAST_TRACK" ]; then
        pkill -RTMIN+10 i3blocks
        LAST_TRACK="$CURRENT_TRACK"
    fi
    sleep 1
done
```

## Multiple Devices

### Device-Specific Blocks

```ini
[music-main]
command=wiim-control --profile i3blocks status
interval=1
label=🏠
color=#a3be8c
signal=10

[music-kitchen]
command=wiim-control --profile i3blocks-kitchen status
interval=1
label=🍳
color=#88c0d0
signal=11

[music-bedroom]
command=wiim-control --profile i3blocks-bedroom status
interval=1
label=🛏️
color=#b48ead
signal=12
```

### Configuration for Multiple Devices

```toml
device_ip = "192.168.1.100"

[profiles.i3blocks]
format = "text"
text_template = "{track_info} | {volume}%"

[profiles.i3blocks-kitchen]
device_ip = "192.168.1.101"
format = "text"
text_template = "{track_info}"

[profiles.i3blocks-bedroom]
device_ip = "192.168.1.102"
format = "text"
text_template = "{track_info} | {volume}%"
```

## Theme Integration

### Nord Theme

```ini
[music]
command=wiim-control --profile i3blocks status
interval=1
label=♪
color=#a3be8c
background=#2e3440
border=#88c0d0
border_top=1
border_bottom=1
min_width=180
align=center
signal=10
```

### Gruvbox Theme

```ini
[music]
command=wiim-control --profile i3blocks status
interval=1
label=♪
color=#b8bb26
background=#282828
border=#458588
border_top=1
border_bottom=1
min_width=180
align=center
signal=10
```

### Dracula Theme

```ini
[music]
command=wiim-control --profile i3blocks status
interval=1
label=♪
color=#50fa7b
background=#282a36
border=#6272a4
border_top=1
border_bottom=1
min_width=180
align=center
signal=10
```

## Advanced Scripts

### Notification Integration

```bash
#!/bin/bash
# ~/.config/i3blocks/scripts/wiim-notify.sh

# Get current status
OUTPUT=$(wiim-control --profile i3blocks status)
STATE=$(wiim-control status --profile custom --template "{state}")

# Handle clicks with notifications
case $BLOCK_BUTTON in
    1) # Left click - toggle
        wiim-control toggle
        if [ "$STATE" = "playing" ]; then
            notify-send "WiiM" "Paused" -i audio-volume-muted
        else
            TRACK=$(wiim-control status --profile custom --template "{track_info}")
            notify-send "WiiM" "Playing: $TRACK" -i audio-volume-high
        fi
        ;;
    2) # Middle click - previous
        wiim-control prev
        TRACK=$(wiim-control status --profile custom --template "{track_info}")
        notify-send "WiiM" "Previous: $TRACK" -i media-skip-backward
        ;;
    3) # Right click - next
        wiim-control next
        TRACK=$(wiim-control status --profile custom --template "{track_info}")
        notify-send "WiiM" "Next: $TRACK" -i media-skip-forward
        ;;
    4) # Scroll up - volume up
        NEW_VOL=$(wiim-control volume-up 5)
        notify-send "WiiM" "Volume: $NEW_VOL%" -i audio-volume-high
        ;;
    5) # Scroll down - volume down
        NEW_VOL=$(wiim-control volume-down 5)
        notify-send "WiiM" "Volume: $NEW_VOL%" -i audio-volume-low
        ;;
esac

# Output for i3blocks
echo "$OUTPUT"
echo "$OUTPUT"

# Set color based on state
case "$STATE" in
    "playing")
        echo "#a3be8c"
        ;;
    "paused")
        echo "#ebcb8b"
        ;;
    "stopped")
        echo "#d08770"
        ;;
    *)
        echo "#e5e9f0"
        ;;
esac

# Refresh the block
pkill -RTMIN+10 i3blocks
```

### Progress Bar Integration

```bash
#!/bin/bash
# ~/.config/i3blocks/scripts/wiim-progress.sh

# Get track information
POSITION_MS=$(wiim-control status --profile custom --template "{position_ms}")
DURATION_MS=$(wiim-control status --profile custom --template "{duration_ms}")
TRACK_INFO=$(wiim-control status --profile custom --template "{track_info}")

# Calculate progress
if [ "$DURATION_MS" -gt 0 ]; then
    PROGRESS=$((POSITION_MS * 10 / DURATION_MS))
    PROGRESS_BAR=""
    for i in $(seq 1 10); do
        if [ $i -le $PROGRESS ]; then
            PROGRESS_BAR="$PROGRESS_BAR█"
        else
            PROGRESS_BAR="$PROGRESS_BAR░"
        fi
    done
    echo "$TRACK_INFO [$PROGRESS_BAR]"
else
    echo "$TRACK_INFO"
fi
```

## Performance Optimization

### Efficient Updates

```ini
[music]
command=wiim-control --profile i3blocks status
interval=2
signal=10
# Reduce update frequency for better performance
```

### Conditional Updates

```bash
#!/bin/bash
# ~/.config/i3blocks/scripts/wiim-efficient.sh

# Cache file for last known state
CACHE_FILE="/tmp/wiim-i3blocks-cache"
CURRENT_STATE=$(wiim-control status --profile custom --template "{state}")

# Only update if state changed or if playing
if [ ! -f "$CACHE_FILE" ] || [ "$(cat $CACHE_FILE)" != "$CURRENT_STATE" ] || [ "$CURRENT_STATE" = "playing" ]; then
    echo "$CURRENT_STATE" > "$CACHE_FILE"
    OUTPUT=$(wiim-control --profile i3blocks status)
    echo "$OUTPUT"
    echo "$OUTPUT"
    echo "#a3be8c"
else
    # Return cached output for stopped/paused states
    echo "No music"
    echo "No music"
    echo "#d08770"
fi
```

## Troubleshooting

### Common Issues

#### Block Not Updating
- Check if `wiim-control` is in your PATH
- Verify the device IP address in config
- Test manually: `wiim-control --profile i3blocks status`

#### Click Actions Not Working
- Ensure click handler script is executable: `chmod +x ~/.config/i3blocks/scripts/wiim-click.sh`
- Check that `$BLOCK_BUTTON` is properly handled
- Verify signal numbers are correct

#### Performance Issues
- Increase interval if updates are too frequent
- Use efficient update scripts
- Consider caching for stopped/paused states

### Debug Commands

```bash
# Test basic functionality
wiim-control status

# Test i3blocks profile
wiim-control --profile i3blocks status

# Test click handling
BLOCK_BUTTON=1 ~/.config/i3blocks/scripts/wiim-click.sh

# Check device connectivity
wiim-control --device 192.168.1.100 status
```

## Complete Example

### Full Configuration

**`~/.config/wiim-control/config.toml`**:
```toml
device_ip = "192.168.1.100"

[profiles.i3blocks]
format = "text"
text_template = "♪ {artist} - {title} • {volume}%"
```

**`~/.config/i3blocks/config`**:
```ini
[music]
command=~/.config/i3blocks/scripts/wiim-music.sh
interval=1
label=♪
color=#a3be8c
background=#2e3440
border=#88c0d0
border_top=1
border_bottom=1
min_width=200
align=center
signal=10
```

**`~/.config/i3blocks/scripts/wiim-music.sh`**:
```bash
#!/bin/bash

# Get current status and state
OUTPUT=$(wiim-control --profile i3blocks status)
STATE=$(wiim-control status --profile custom --template "{state}")

# Handle clicks
case $BLOCK_BUTTON in
    1) wiim-control toggle ;;
    2) wiim-control prev ;;
    3) wiim-control next ;;
    4) wiim-control volume-up 5 ;;
    5) wiim-control volume-down 5 ;;
esac

# Output for i3blocks
echo "$OUTPUT"
echo "$OUTPUT"

# Set color based on state
case "$STATE" in
    "playing") echo "#a3be8c" ;;
    "paused") echo "#ebcb8b" ;;
    "stopped") echo "#d08770" ;;
    *) echo "#e5e9f0" ;;
esac

# Refresh the block if button was pressed
if [ -n "$BLOCK_BUTTON" ]; then
    pkill -RTMIN+10 i3blocks
fi
```

**`~/.config/i3/config`**:
```
bar {
    status_command i3blocks -c ~/.config/i3blocks/config
    colors {
        statusline #ffffff
        background #2e3440
        focused_workspace #88c0d0 #88c0d0 #2e3440
        active_workspace #2e3440 #2e3440 #88c0d0
        inactive_workspace #2e3440 #2e3440 #88c0d0
        urgent_workspace #bf616a #bf616a #2e3440
    }
}
```

## See Also

- [Template System Overview](../templates/README.md)
- [Template Variables Reference](../templates/variables.md)
- [Configuration Examples](../templates/examples.md)
- [Waybar Integration](waybar.md)
- [Polybar Integration](polybar.md)
