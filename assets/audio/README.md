# Audio Files for Backrooms Doom

## Required Files

Place the following audio files in this directory (`assets/audio/`):

### 1. ambiental.wav
- **Type**: Background music (looping)
- **Duration**: 30-60 seconds
- **Description**: Eerie ambient drone or hum
- **Format**: WAV, 44100 Hz, Mono or Stereo
- **Volume will be set to 60% (0.6) and increase near the goal**

### 2. start.wav
- **Type**: Sound effect (one-shot)
- **Duration**: 1-2 seconds
- **Description**: Game start sound (door opening, chime, etc.)
- **Format**: WAV, 44100 Hz

### 3. footstep.wav
- **Type**: Sound effect (repeating)
- **Duration**: 0.2-0.5 seconds
- **Description**: Footstep on carpet
- **Format**: WAV, 44100 Hz
- **Plays every 0.5 seconds while moving**

### 4. victory.wav
- **Type**: Sound effect (one-shot)
- **Duration**: 2-5 seconds
- **Description**: Victory/escape sound
- **Format**: WAV, 44100 Hz

### 5. damage.wav (optional)
- **Type**: Sound effect (one-shot)
- **Duration**: 0.5-1 second
- **Description**: Damage/hurt sound
- **Format**: WAV, 44100 Hz

### 6. heartbeat.wav (optional)
- **Type**: Sound effect (one-shot)
- **Duration**: 1-2 seconds
- **Description**: Heartbeat sound
- **Format**: WAV, 44100 Hz

## Creating Placeholder Files

If you don't have audio files yet, you can:

1. **Download from free sources:**
   - https://freesound.org
   - https://zapsplat.com
   - https://mixkit.co/free-sound-effects/

2. **Generate using Audacity:**
   - Generate > Tone for ambient hum
   - Generate > Noise for effects
   - Export as WAV

3. **Use AI generation:**
   - Suno AI, Soundraw, or similar

## Current Status

**The game will run without audio files** - it will print messages to console instead.

To enable audio:
1. Add WAV files to this directory
2. Restart the game
3. Audio will play automatically

## Volume Settings

- **Master Volume**: 100% (1.0)
- **Background Music**: 60% (0.6) - increases to 100% near goal
- **Sound Effects**: 70% (0.7)
