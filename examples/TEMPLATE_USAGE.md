# Template Usage Guide

The wiim-control CLI supports customizable output templates using the Handlebars templating engine.

## Configuration

Templates are configured in `~/.config/wiim-control/config.toml`. See `template_config.toml` in this directory for a complete example.

### Basic Text Templates

```toml
[output.text]
playing = "▶️ {artist} - {title} {quality_info}"
paused = "⏸️ {artist} - {title}"
stopped = "⏹️ No music"
loading = "⏳ Loading..."
```

### JSON Output Templates

```toml
[output.json]
text = "{artist} - {title}"
alt = "{state}"
tooltip = "{title}\nArtist: {artist}\nVolume: {volume}%"
class = "{state}"
```

## Discovering Available Variables

To see all available template variables, run:

```bash
wiim-control status --format json
```

This shows the current data structure with all available fields that can be used in templates.

## Common Patterns

### Artist and Title
```
{artist} - {title}
```

### Include Audio Quality
```
{artist} - {title} {quality_info}
```

### Multi-line Tooltip
```
{title}
Artist: {artist}
Volume: {volume}%
```

### Conditional Display
Templates automatically handle missing fields - if `{artist}` is empty, it won't be displayed.

## Default Behavior

Without any template configuration, wiim-control uses built-in defaults that match the original output format.
