# Waybar Integration Guide

This guide shows how to integrate WiiM API with Waybar, a highly customizable status bar for Wayland compositors.

## Quick Setup

### 1. Install WiiM Control CLI

```bash
# Install from source
cargo install --path . --bin wiim-control

# Or download from releases
# [Installation instructions will be added when releases are available]
```

### 2. Basic Waybar Configuration

Add this to your `~/.config/waybar/config`:

```json
{
    "modules-left": ["sway/workspaces"],
    "modules-center": ["custom/music"],
    "modules-right": ["network", "pulseaudio", "clock"],

    "custom/music": {
        "exec": "wiim-control --profile waybar status",
        "return-type": "json",
        "interval": 1,
        "max-length": 50,
        "on-click": "wiim-control toggle",
        "on-click-right": "wiim-control next",
        "on-click-middle": "wiim-control prev",
        "on-scroll-up": "wiim-control volume-up",
        "on-scroll-down": "wiim-control volume-down",
        "format": "{icon} {text}",
        "format-icons": {
            "playing": "♪",
            "paused": "⏸",
            "stopped": "⏹"
        },
        "tooltip": true
    }
}
```

### 3. Basic CSS Styling

Add this to your `~/.config/waybar/style.css`:

```css
#custom-music {
    background-color: #2d3748;
    color: #e2e8f0;
    padding: 0 12px;
    border-radius: 8px;
    margin: 0 4px;
    font-weight: 500;
}

#custom-music.playing {
    background-color: #38a169;
    color: #f7fafc;
}

#custom-music.paused {
    background-color: #ecc94b;
    color: #2d3748;
}

#custom-music.stopped {
    background-color: #4a5568;
    color: #a0aec0;
}
```

## Advanced Configuration

### Template Customization

Create `~/.config/wiim-control/config.toml`:

```toml
device_ip = "192.168.1.100"

[profiles.waybar]
format = "json"

[output.json]
text = "{artist} - {title}"
alt = "{state}"
tooltip = "{full_info}"
class = "{state}"
percentage = "{volume}"
```

### Advanced Waybar Module

```json
{
    "custom/music": {
        "exec": "wiim-control --profile waybar status",
        "return-type": "json",
        "interval": 1,
        "max-length": 60,
        "min-length": 20,
        "align": 0,
        "justify": "center",
        "on-click": "wiim-control toggle",
        "on-click-right": "wiim-control next",
        "on-click-middle": "wiim-control prev",
        "on-scroll-up": "wiim-control volume-up 5",
        "on-scroll-down": "wiim-control volume-down 5",
        "format": "{icon} {text}",
        "format-icons": {
            "playing": "♪",
            "paused": "⏸",
            "stopped": "⏹",
            "loading": "⏳"
        },
        "tooltip": true,
        "tooltip-format": "{tooltip}",
        "signal": 8
    }
}
```

### Custom Template Options

#### Minimal Display
```toml
[output.json]
text = "{track_info}"
alt = "{state}"
tooltip = "{artist} - {title}\nVolume: {volume}%"
class = "{state}"
percentage = "{volume}"
```

#### Quality-Focused Display
```toml
[output.json]
text = "{artist} - {title} {quality_info}"
alt = "{state}"
tooltip = "{full_info}"
class = "music-{state}"
percentage = "{volume}"
```

#### Compact Display
```toml
[output.json]
text = "{track_info}"
alt = "{volume}%"
tooltip = "{full_info}"
class = "{state}"
percentage = "{volume}"
```

## Styling Options

### Basic Theming

```css
/* Basic module styling */
#custom-music {
    background-color: #2d3748;
    color: #e2e8f0;
    padding: 0 12px;
    border-radius: 8px;
    margin: 0 4px;
    font-weight: 500;
    min-width: 200px;
}

/* State-based styling */
#custom-music.playing {
    background-color: #38a169;
    color: #f7fafc;
}

#custom-music.paused {
    background-color: #ecc94b;
    color: #2d3748;
}

#custom-music.stopped {
    background-color: #4a5568;
    color: #a0aec0;
}

#custom-music.loading {
    background-color: #4299e1;
    color: #f7fafc;
}
```

### Advanced Theming

```css
/* Module with gradient background */
#custom-music {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: #ffffff;
    padding: 0 16px;
    border-radius: 12px;
    margin: 0 6px;
    font-weight: 600;
    font-size: 14px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    transition: all 0.3s ease;
}

/* Hover effects */
#custom-music:hover {
    background: linear-gradient(135deg, #5a67d8 0%, #6b46c1 100%);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

/* Playing state with animation */
#custom-music.playing {
    background: linear-gradient(135deg, #38a169 0%, #2f855a 100%);
    animation: pulse 2s infinite;
}

@keyframes pulse {
    0% { box-shadow: 0 0 0 0 rgba(56, 161, 105, 0.4); }
    70% { box-shadow: 0 0 0 6px rgba(56, 161, 105, 0); }
    100% { box-shadow: 0 0 0 0 rgba(56, 161, 105, 0); }
}

/* Paused state */
#custom-music.paused {
    background: linear-gradient(135deg, #ecc94b 0%, #d69e2e 100%);
    color: #2d3748;
}

/* Stopped state */
#custom-music.stopped {
    background: linear-gradient(135deg, #4a5568 0%, #2d3748 100%);
    color: #a0aec0;
}
```

### Icon-Based Styling

```css
/* Icon-focused design */
#custom-music {
    background-color: transparent;
    color: #e2e8f0;
    padding: 0 8px;
    font-size: 16px;
    min-width: 150px;
}

/* State-specific icons via CSS content */
#custom-music::before {
    content: "♪ ";
    font-size: 18px;
    margin-right: 4px;
}

#custom-music.playing::before {
    content: "▶️ ";
    color: #38a169;
}

#custom-music.paused::before {
    content: "⏸️ ";
    color: #ecc94b;
}

#custom-music.stopped::before {
    content: "⏹️ ";
    color: #4a5568;
}
```

## Interaction Features

### Click Actions

```json
{
    "custom/music": {
        "on-click": "wiim-control toggle",
        "on-click-right": "wiim-control next",
        "on-click-middle": "wiim-control prev",
        "on-scroll-up": "wiim-control volume-up 5",
        "on-scroll-down": "wiim-control volume-down 5"
    }
}
```

### Advanced Interactions

```json
{
    "custom/music": {
        "on-click": "wiim-control toggle",
        "on-click-right": "wiim-control next",
        "on-click-middle": "wiim-control prev",
        "on-scroll-up": "wiim-control volume-up 5",
        "on-scroll-down": "wiim-control volume-down 5",
        "signal": 8
    }
}
```

### Custom Commands

```bash
# Create custom script for complex actions
# ~/.config/waybar/scripts/wiim-control.sh
#!/bin/bash
case "$1" in
    "toggle")
        wiim-control toggle
        ;;
    "next")
        wiim-control next
        notify-send "WiiM" "Next track"
        ;;
    "prev")
        wiim-control prev
        notify-send "WiiM" "Previous track"
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

## Multiple Devices

### Device-Specific Profiles

```toml
# ~/.config/wiim-control/config.toml
device_ip = "192.168.1.100"

[profiles.waybar-kitchen]
device_ip = "192.168.1.101"
format = "json"

[profiles.waybar-bedroom]
device_ip = "192.168.1.102"
format = "json"

[output.json]
text = "{artist} - {title}"
alt = "{state}"
tooltip = "{full_info}"
class = "{state}"
percentage = "{volume}"
```

### Multiple Waybar Modules

```json
{
    "modules-center": ["custom/music-main", "custom/music-kitchen"],

    "custom/music-main": {
        "exec": "wiim-control --profile waybar status",
        "return-type": "json",
        "interval": 1,
        "format": "🏠 {text}",
        "tooltip": true
    },

    "custom/music-kitchen": {
        "exec": "wiim-control --profile waybar-kitchen status",
        "return-type": "json",
        "interval": 1,
        "format": "🍳 {text}",
        "tooltip": true
    }
}
```

## Performance Optimization

### Efficient Updates

```json
{
    "custom/music": {
        "exec": "wiim-control --profile waybar status",
        "return-type": "json",
        "interval": 2,
        "restart-interval": 30,
        "max-length": 50,
        "signal": 8
    }
}
```

### Conditional Updates

```bash
# Script to update only when playing
#!/bin/bash
STATE=$(wiim-control status --profile custom --template "{state}")
if [[ "$STATE" == "playing" ]]; then
    wiim-control --profile waybar status
else
    echo '{"text": "No music", "class": "stopped"}'
fi
```

## Troubleshooting

### Common Issues

#### Module Not Updating
- Check if `wiim-control` is in your PATH
- Verify the device IP address in config
- Test the command manually: `wiim-control --profile waybar status`

#### JSON Format Errors
- Validate JSON output: `wiim-control --profile waybar status | jq`
- Check for template syntax errors
- Ensure `return-type` is set to `"json"`

#### Styling Not Applied
- Verify CSS selectors match the module name
- Check that class names are correct
- Ensure CSS file is loaded in Waybar config

### Debug Commands

```bash
# Test basic functionality
wiim-control status

# Test JSON output
wiim-control --profile waybar status

# Validate JSON
wiim-control --profile waybar status | jq

# Check device connectivity
wiim-control --device 192.168.1.100 status
```

## Complete Example

### Full Configuration

**`~/.config/wiim-control/config.toml`**:
```toml
device_ip = "192.168.1.100"

[profiles.waybar]
format = "json"

[output.json]
text = "{artist} - {title}"
alt = "{state}"
tooltip = "{full_info}"
class = "{state}"
percentage = "{volume}"
```

**`~/.config/waybar/config`**:
```json
{
    "layer": "top",
    "position": "top",
    "height": 30,
    "modules-left": ["sway/workspaces"],
    "modules-center": ["custom/music"],
    "modules-right": ["network", "pulseaudio", "clock"],

    "custom/music": {
        "exec": "wiim-control --profile waybar status",
        "return-type": "json",
        "interval": 1,
        "max-length": 50,
        "on-click": "wiim-control toggle",
        "on-click-right": "wiim-control next",
        "on-click-middle": "wiim-control prev",
        "on-scroll-up": "wiim-control volume-up",
        "on-scroll-down": "wiim-control volume-down",
        "format": "{icon} {text}",
        "format-icons": {
            "playing": "♪",
            "paused": "⏸",
            "stopped": "⏹"
        },
        "tooltip": true
    }
}
```

**`~/.config/waybar/style.css`**:
```css
#custom-music {
    background-color: #2d3748;
    color: #e2e8f0;
    padding: 0 12px;
    border-radius: 8px;
    margin: 0 4px;
    font-weight: 500;
    min-width: 200px;
}

#custom-music.playing {
    background-color: #38a169;
    color: #f7fafc;
}

#custom-music.paused {
    background-color: #ecc94b;
    color: #2d3748;
}

#custom-music.stopped {
    background-color: #4a5568;
    color: #a0aec0;
}
```

## See Also

- [Template System Overview](../templates/README.md)
- [Template Variables Reference](../templates/variables.md)
- [Configuration Examples](../templates/examples.md)
- [Polybar Integration](polybar.md)
- [i3blocks Integration](i3blocks.md)
