# Audio Quality Guide

Understanding the audio quality indicators displayed in WiiM API templates can help you optimize your listening experience and make informed decisions about your audio setup.

## Quick Reference

### Quality Indicators in Templates

| Template Variable | Example | Meaning |
|------------------|---------|---------|
| `{sample_rate}` | `"192000"` | Raw sample rate in Hz |
| `{bit_depth}` | `"24"` | Raw bit depth |
| `{sample_rate_khz}` | `"192kHz"` | Formatted sample rate |
| `{bit_depth_bit}` | `"24bit"` | Formatted bit depth |
| `{quality_info}` | `"192kHz/24bit"` | Combined quality display |

## Understanding Sample Rates

### What is Sample Rate?

Sample rate measures how many times per second the audio signal is sampled during recording or playback. It's measured in Hertz (Hz) or kilohertz (kHz).

### Common Sample Rates

#### 44.1 kHz (CD Quality)
- **Raw value**: `"44100"`
- **Formatted**: `"44kHz"`
- **Quality**: Standard CD quality
- **Use cases**: Most commercial music, streaming services
- **File size**: Baseline for comparison

#### 48 kHz (Professional Standard)
- **Raw value**: `"48000"`
- **Formatted**: `"48kHz"`
- **Quality**: Professional audio/video production standard
- **Use cases**: Film soundtracks, professional recording
- **File size**: ~9% larger than 44.1kHz

#### 96 kHz (High-Resolution)
- **Raw value**: `"96000"`
- **Formatted**: `"96kHz"`
- **Quality**: High-resolution audio
- **Use cases**: Audiophile recordings, studio masters
- **File size**: ~118% larger than 44.1kHz

#### 192 kHz (Ultra High-Resolution)
- **Raw value**: `"192000"`
- **Formatted**: `"192kHz"`
- **Quality**: Ultra high-resolution audio
- **Use cases**: Audiophile recordings, archival masters
- **File size**: ~336% larger than 44.1kHz

### Sample Rate Impact

**Higher sample rates provide:**
- Better representation of high-frequency content
- More accurate waveform reproduction
- Reduced aliasing artifacts
- Larger file sizes and bandwidth requirements

**Diminishing returns:**
- Human hearing typically tops out around 20kHz
- Benefits above 96kHz are debated among audiophiles
- Network bandwidth becomes more important

## Understanding Bit Depth

### What is Bit Depth?

Bit depth determines the precision of each audio sample, affecting the dynamic range and noise floor of the audio signal.

### Common Bit Depths

#### 16-bit (CD Quality)
- **Raw value**: `"16"`
- **Formatted**: `"16bit"`
- **Dynamic range**: ~96 dB
- **Use cases**: CD audio, most streaming services
- **File size**: Baseline for comparison

#### 24-bit (Professional/Hi-Res)
- **Raw value**: `"24"`
- **Formatted**: `"24bit"`
- **Dynamic range**: ~144 dB
- **Use cases**: Professional recording, hi-res streaming
- **File size**: 50% larger than 16-bit

#### 32-bit (Professional/Mastering)
- **Raw value**: `"32"`
- **Formatted**: `"32bit"`
- **Dynamic range**: ~192 dB
- **Use cases**: Professional mastering, floating-point audio
- **File size**: 100% larger than 16-bit

### Bit Depth Impact

**Higher bit depths provide:**
- Greater dynamic range
- Lower noise floor
- Better signal-to-noise ratio
- More headroom for processing

**Practical considerations:**
- 24-bit provides excellent quality for most listeners
- 32-bit is mainly beneficial for audio production
- Larger file sizes with higher bit depths

## Quality Combinations

### Common Quality Levels

#### CD Quality: 44.1kHz/16bit
- **Template display**: `"44kHz/16bit"`
- **Use case**: Standard quality music
- **Bandwidth**: Low
- **Quality**: Good for most listeners

#### Hi-Res Audio: 96kHz/24bit
- **Template display**: `"96kHz/24bit"`
- **Use case**: Audiophile streaming
- **Bandwidth**: High
- **Quality**: Excellent for critical listening

#### Studio Master: 192kHz/24bit
- **Template display**: `"192kHz/24bit"`
- **Use case**: Audiophile recordings
- **Bandwidth**: Very high
- **Quality**: Maximum available quality

### Quality Tiers

#### Basic Quality (16-bit)
- **CD Quality**: 44.1kHz/16bit
- **Broadcast**: 48kHz/16bit
- **Suitable for**: Casual listening, portable devices

#### High Quality (24-bit)
- **Hi-Res**: 96kHz/24bit or 192kHz/24bit
- **Studio**: 48kHz/24bit
- **Suitable for**: Critical listening, high-end systems

#### Professional Quality (32-bit)
- **Mastering**: 96kHz/32bit or 192kHz/32bit
- **Production**: 48kHz/32bit
- **Suitable for**: Audio production, mastering

## Network Quality Impact

### Bandwidth Requirements

Quality affects network bandwidth needs:

#### Low Bandwidth (< 1 Mbps)
- **Quality**: 44.1kHz/16bit
- **Streaming**: Reliable on most connections
- **Use case**: Mobile networks, slower connections

#### Medium Bandwidth (1-5 Mbps)
- **Quality**: 48kHz/24bit to 96kHz/24bit
- **Streaming**: Good for most home networks
- **Use case**: Home Wi-Fi, good internet connections

#### High Bandwidth (> 5 Mbps)
- **Quality**: 192kHz/24bit or higher
- **Streaming**: Requires excellent network
- **Use case**: Audiophile setups, wired connections

### Network Quality Variables

When available, templates show network quality information:

#### `{wifi_signal}`
- **Example**: `"-30 dBm"`
- **Meaning**: WiFi signal strength (closer to 0 is better)
- **Good**: -30 to -50 dBm
- **Fair**: -50 to -70 dBm
- **Poor**: below -70 dBm

#### `{wifi_rate}`
- **Example**: `"390 Mbps"`
- **Meaning**: WiFi connection speed
- **Impact**: Higher rates support better quality streaming

#### `{bitrate}`
- **Example**: `"5719 kbps"`
- **Meaning**: Current stream bitrate
- **Quality correlation**: Higher bitrates generally mean better quality

## Practical Considerations

### Choosing Quality Settings

#### For Casual Listening
- **Recommended**: 44.1kHz/16bit to 48kHz/16bit
- **Benefits**: Lower bandwidth, good quality
- **Trade-offs**: Slightly lower fidelity

#### For Critical Listening
- **Recommended**: 96kHz/24bit
- **Benefits**: Excellent quality, reasonable bandwidth
- **Trade-offs**: Higher bandwidth requirements

#### For Audiophile Setups
- **Recommended**: 192kHz/24bit or higher
- **Benefits**: Maximum quality
- **Trade-offs**: Very high bandwidth, large files

### System Considerations

#### Network Capacity
- Check your internet speed and Wi-Fi capabilities
- Consider wired connections for highest quality
- Monitor network quality indicators

#### Storage Space
- Higher quality means larger files
- Consider storage capacity for local files
- Streaming reduces local storage needs

#### Playback Equipment
- Your speakers/headphones may be the limiting factor
- High-quality DACs can reveal quality differences
- Room acoustics affect perceived quality

## Template Usage Examples

### Basic Quality Display

```toml
[output.text]
playing = "▶️ {artist} - {title} {quality_info}"
```
**Output**: `"▶️ The Beatles - Hey Jude 192kHz/24bit"`

### Detailed Quality Information

```toml
[output.text]
playing = "♪ {artist} - {title} • {sample_rate_khz}/{bit_depth_bit} • Vol: {volume}%"
```
**Output**: `"♪ The Beatles - Hey Jude • 192kHz/24bit • Vol: 75%"`

### Quality-Focused Templates

```toml
[profiles.audiophile]
format = "text"
text_template = "🎵 {artist} - {title}\n📊 Quality: {quality_info}\n📡 Network: {wifi_signal}"

[profiles.quality-check]
format = "text"
text_template = "{quality_info} @ {bitrate}"
```

### Conditional Quality Display

Templates automatically handle missing quality information:

```toml
[output.text]
playing = "▶️ {artist} - {title} {quality_info}"
```

- **With quality**: `"▶️ Artist - Title 192kHz/24bit"`
- **Without quality**: `"▶️ Artist - Title"` (empty quality_info)

## Quality Terminology

### Lossless vs. Lossy
- **Lossless**: No data lost during compression (FLAC, ALAC)
- **Lossy**: Some data removed for smaller files (MP3, AAC)
- **Impact**: Lossless preserves original quality

### Hi-Res Audio
- **Definition**: Audio with higher resolution than CD quality
- **Minimum**: 48kHz/24bit or 96kHz/24bit
- **Standard**: Usually 96kHz/24bit or 192kHz/24bit

### DSD (Direct Stream Digital)
- **Alternative**: To PCM audio (what sample rates measure)
- **Rates**: DSD64, DSD128, DSD256
- **Quality**: Very high quality, specialized format

### MQA (Master Quality Authenticated)
- **Technology**: Efficient high-quality audio delivery
- **Benefits**: High quality with reduced bandwidth
- **Compatibility**: Requires MQA-compatible equipment

## Troubleshooting Quality Issues

### Network-Related Issues

#### Poor Quality Despite High-Res Source
- Check network bandwidth and stability
- Monitor `{wifi_signal}` and `{wifi_rate}` values
- Consider wired connection for critical listening

#### Intermittent Quality Changes
- Network congestion can cause quality reduction
- Check other devices using bandwidth
- Monitor `{bitrate}` for changes

### Equipment-Related Issues

#### No Quality Improvement with Hi-Res
- Check DAC capabilities
- Verify playback chain supports high-resolution
- Consider room acoustics and speaker quality

#### Quality Information Not Displayed
- Some sources don't provide quality metadata
- Network quality requires compatible firmware
- Check WiiM device capabilities

## Best Practices

### Network Optimization
1. **Use wired connections** for highest quality
2. **Monitor network quality** using template variables
3. **Manage bandwidth** usage on your network
4. **Position WiiM device** for optimal Wi-Fi signal

### Quality Selection
1. **Start with reasonable quality** (48kHz/24bit)
2. **Upgrade gradually** to assess benefits
3. **Consider your equipment** capabilities
4. **Balance quality with bandwidth** needs

### Template Configuration
1. **Include quality information** in your templates
2. **Monitor network indicators** when available
3. **Create quality-specific profiles** for different use cases
4. **Test templates** with various quality sources

## See Also

- [Template Variables Reference](variables.md) - Complete variable documentation
- [Template System Overview](README.md) - System overview and concepts
- [Configuration Examples](examples.md) - Sample configurations
- [Integration Guides](../integrations/) - Status bar setup guides
