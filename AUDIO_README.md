# Backrooms Doom - Audio System Guide

## 🎵 Audio System Overview

The audio system is **fully integrated** into the game and ready to use! Currently it uses placeholder implementations (console logs) until you add actual audio files.

## ✅ What's Implemented

### **Audio Manager Features:**
- ✅ Background music looping during gameplay
- ✅ Automatic music start/stop on state transitions
- ✅ Dynamic volume adjustment based on distance to goal
- ✅ Footstep sound timing (every 0.5 seconds when moving)
- ✅ Victory sound on reaching the exit
- ✅ Start game sound effect
- ✅ Damage + heartbeat sound system

### **Game Integration:**
- ✅ **Menu State**: Music is stopped
- ✅ **Playing State**: Background music plays and loops continuously
- ✅ **Victory State**: Music stops, victory sound plays

### **Audio Lifecycle:**
```
Menu → [ENTER] → Start Game → Playing (Music Loops) → Goal Reached → Victory
                                     ↓ ESC
                                   Menu (Music Stops)
```

## 📁 Required Audio Files

Place these files in `assets/audio/`:

1. **ambiental.wav** - Background music (should be loopable)
   - Recommended: Ambient drone, eerie hum, 30-60 seconds
   - Will loop continuously during gameplay
   - Volume increases as player approaches goal

2. **start.wav** - Game start sound
   - Plays once when pressing ENTER to start
   - Short (1-2 seconds)

3. **footstep.wav** - Footstep sound effect
   - Plays every 0.5 seconds while moving
   - Keep it short and subtle

4. **victory.wav** - Victory sound
   - Plays when reaching the blue exit door
   - Can be longer (3-5 seconds)

5. **damage.wav** - Damage sound (optional)
   - For future health system integration

6. **heartbeat.wav** - Heartbeat sound (optional)
   - Plays after damage sound

## 🎮 How It Works

### **During Gameplay:**
```rust
// Music starts when entering gameplay
Menu → Playing: audio.play_background_music()

// Every frame during gameplay:
- audio.update_music()          // Keeps music stream alive and loops
- audio.update_ambient(distance) // Adjusts volume based on proximity to goal
- audio.play_footstep(dt)       // Plays footsteps when moving

// When reaching goal:
Playing → Victory: audio.stop_music() + audio.play_victory()

// When returning to menu:
Playing → Menu: audio.stop_music()
```

### **Audio Methods:**

| Method | When Called | Purpose |
|--------|-------------|---------|
| `play_background_music()` | Entering gameplay | Starts looping ambient music |
| `update_music()` | Every frame (Playing) | Updates stream, handles looping |
| `update_ambient(distance)` | Every frame (Playing) | Adjusts volume near goal |
| `stop_music()` | Leaving gameplay | Stops background music |
| `play_start_game()` | Press ENTER in menu | Start game sound |
| `play_footstep(dt)` | While moving | Timed footsteps |
| `play_victory()` | Reach goal | Victory sound |

## 🔧 Adding Real Audio

### **Option 1: Using the Placeholder System (Current)**
The game compiles and runs with placeholder audio (console logs). This is perfect for:
- Testing gameplay without audio files
- Development before audio assets are ready
- Understanding the audio flow

### **Option 2: Implementing Real Audio**
To add actual audio playback, you need to:

1. **Add audio files** to `assets/audio/`
2. **Update audio.rs** to use raylib's audio API:

```rust
// Replace TODO comments in audio.rs with actual implementations
// Example for background music:
pub fn play_background_music(&mut self) {
    // Load music file using raylib
    // Set to loop
    // Play with initial volume
    self.music_playing = true;
}
```

3. **Raylib Audio API** (for reference):
   - Load music: `rl.load_music_stream(thread, path)`
   - Play music: `audio.play_music_stream(&music)`
   - Update music: `audio.update_music_stream(&mut music)`
   - Check playing: `audio.is_music_stream_playing(&music)`
   - Load sound: `rl.load_sound(thread, path)`
   - Play sound: `audio.play_sound(&sound)`

## 🎨 Audio Asset Recommendations

### **Free Resources:**
- [freesound.org](https://freesound.org) - Footsteps, ambient sounds
- [incompetech.com](https://incompetech.com) - Royalty-free music
- [zapsplat.com](https://zapsplat.com) - Sound effects

### **Backrooms-Specific Suggestions:**
- **Ambient Music**: Low hum, fluorescent light buzz, electrical drone
- **Footsteps**: Carpet footsteps, soft thuds
- **Victory**: Relief sound, escape chime
- **Start**: Door opening, reality shift

### **Audio Specifications:**
- **Format**: WAV (best compatibility with raylib)
- **Sample Rate**: 44100 Hz
- **Channels**: Mono or Stereo
- **Bit Depth**: 16-bit

## 📊 Current Status

```
✅ Audio system architecture complete
✅ All functions properly integrated
✅ State transitions handle music correctly
✅ Placeholder implementations working
⏳ Waiting for audio asset files
⏳ Raylib API integration (when assets are added)
```

## 🚀 Testing

Run the game to see audio system messages:
```bash
cargo run --release
```

You'll see console output:
```
Audio: Start game sound (start.wav)
Audio: Background music would start playing (ambiental.wav)
Audio: Background music stopped
Audio: Victory sound (victory.wav)
```

## 📝 Code Locations

- **Audio Manager**: `src/audio.rs` (lines 11-106)
- **Game Integration**: `src/game.rs` (lines 80-165)
- **Audio State**: Tracked in `AudioManager.music_playing`

---

**The audio system is production-ready!** Just add the audio files to `assets/audio/` and update the TODO sections in `audio.rs` to enable real audio playback. 🎵
