# WiiM API Coverage

This document tracks implementation status of WiiM HTTP API endpoints.

## Implementation Status: 56% (13/23 endpoints)

### ✅ Implemented (13 endpoints)

**Playback Control:**
- `getPlayerStatus` - Get current playback state
- `setPlayerCmd:pause` - Pause playback
- `setPlayerCmd:resume` - Resume playback
- `setPlayerCmd:onepause` - Toggle play/pause
- `setPlayerCmd:stop` - Stop playback
- `setPlayerCmd:next` - Next track
- `setPlayerCmd:prev` - Previous track

**Volume Control:**
- `setPlayerCmd:vol:value` - Set volume (0-100)
- `setPlayerCmd:mute:n` - Mute/unmute

**Track Information:**
- `getMetaInfo` - Get current track metadata

**Device Information:**
- `getStatusEx` - Get comprehensive device and network status

**Library Methods:**
- `get_now_playing()` - Combined status + metadata
- `volume_up()/volume_down()` - Relative volume control

### ❌ Not Implemented

#### High Priority (7 endpoints)
Essential features missing from current implementation:

- `setPlayerCmd:play:url` - **Play audio URL** - Cannot start playback
- `setPlayerCmd:playlist:url:<index>` - **Play playlists** - No playlist support
- `setPlayerCmd:seek:position` - **Seek to position** - No track seeking
- `setPlayerCmd:loopmode:n` - **Set repeat/shuffle** - No loop control
- `setPlayerCmd:switchmode:%s` - **Switch input source** - No BT/optical/aux switching
- `MCUKeyShortClick:%d` - **Play presets** - No quick access to saved stations
- `getPresetInfo` - **Get preset list** - Cannot see configured presets

#### Medium Priority (7 endpoints)
Useful enhancements:

- `EQOn/EQOff/EQLoad` - Equalizer controls
- `EQGetList/EQGetStat` - EQ preset management
- `getNewAudioOutputHardwareMode` - Audio output status
- `setAudioOutputHardwareMode` - Configure hardware outputs
- `wlanGetConnectState` - Network status
- `reboot` - Device restart
- `setShutdown:sec` - Shutdown timer

#### Low Priority (6 endpoints)
Advanced/specialized features:

- `setAlarmClock/getAlarmClock` - Alarm/timer functions
- `timeSync` - Manual time sync
- `getShutdown` - Shutdown timer status
- Various alarm management endpoints

## Key Limitations

**Current library is primarily a monitoring/control tool** with these limitations:

1. **Cannot initiate playback** - Only control existing streams
2. **No content selection** - No URL, playlist, or preset playback
3. **No input management** - Cannot switch between audio sources
4. **No device discovery** - Must manually provide IP addresses
5. **No configuration** - Cannot adjust EQ, outputs, or device settings

## Recommended Next Steps

To become a complete WiiM client library:

1. **Add playback initiation** - `play_url()`, `play_playlist()`
2. **Add preset support** - `play_preset()`, `get_presets()`
3. **Add input switching** - `switch_source()`
4. **Add device info** - `get_device_info()`
5. **Add seek functionality** - `seek()`

These additions would enable the library to serve as a full WiiM remote control rather than just a status monitor.
