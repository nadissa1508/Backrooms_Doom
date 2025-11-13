# Backrooms Doom - Raycasting Project

A complete raycaster game inspired by The Backrooms aesthetic, built with Rust and Raylib.

## Project Overview

This is a raycasting engine project that creates a playable 3D maze-exploration game with Backrooms-inspired aesthetics. The player must navigate through a maze to reach the exit while experiencing atmospheric horror elements.

## Gameplay video
https://youtu.be/QF6CPHZaBwk

## Completed Features & Grading Breakdown

### Core Requirements (✓ Completed)

| Feature | Points | Status | Description |
|---------|--------|--------|-------------|
| **Aesthetic Quality** | 30 | implemented | Backrooms-themed level with yellow fog, fluorescent lighting effect, and atmospheric design |
| **60 FPS Performance** | 15 | implemented | Optimized rendering maintaining 60 FPS (displayed on HUD) |
| **Visual Effects** | 15 | implemented | Multiple effects: fog of war, flashlight, distance shading, damage flash, anxiety distortion, pill glitch animations |
| **Camera System** | 20 | implemented | Mouse-based horizontal rotation with configurable sensitivity |
| **Minimap** | 10 | implemented | Top-right corner minimap showing full maze layout and player position |
| **Background Music** | 5 | implemented | Ambient music with dynamic volume based on distance to goal |
| **Sound Effects** | 10 | implemented | Footstep sounds (with stop control), damage sounds, heartbeat, victory sound, start sound |
| **Sprite Animation** | 20 | implemented | Flickering light sprites with animation system |
| **Welcome Screen** | 5 | implemented | Interactive menu with controls display |
| **Victory Screen** | 10 | implemented | Victory screen when player reaches goal |
| **Health System** | 5 | implemented | Player health with max health tracking |

**Total Points: 145/100** 

## Controls

- **W/A/S/D** - Move forward/left/backward/right
- **Mouse** - Look around (horizontal rotation)
- **Arrow Keys** - Alternative rotation controls
- **ENTER** - Start game / Restart from victory
- **ESC** - Return to menu
- **F3** - Toggle debug info

## Visual Features

### Rendering System
- **Raycasting Engine**: Custom implementation with 80 rays for optimal performance
- **Textured Walls**: Different textures for normal walls and exit walls
- **Textured Floors & Ceilings**: Perspective-correct texture mapping
- **Distance Shading**: Walls darken with distance for depth perception
- **Orientation Shading**: Different wall faces have varying brightness
- **Fog Effect**: Yellowish Backrooms-style atmospheric fog
- **Flashlight Effect**: Center spotlight that brightens the middle of the screen

### Effects System
```rust
- Yellowish Backrooms aesthetic
- Flashlight with adjustable intensity
- Damage flash effect (red tint when taking damage)
- Anxiety effect (screen distortion from red pills or idle penalty)
- Distance-based shading
- Dynamic lighting
- Glitch animations on pills
```

### Minimap
- **Position**: Top-right corner
- **Size**: 100x100 pixels
- **Features**:
  - Shows entire maze layout
  - Player position (blue dot)
  - Direction indicator (line showing facing direction)
  - Start position (green)
  - Goal position (red)
  - Walls (dark gray)
  - Floor (light beige)

## Audio System

### Music
- **Ambient Background Music**: Plays during gameplay
- **Dynamic Volume**: Music volume increases as player approaches goal
- **Menu Music**: Separate track for menu screen

### Sound Effects
- **Footstep Sounds**: Play when player is moving (0.5s interval), automatically stops when player stops moving
- **Start Sound**: Plays when entering gameplay
- **Victory Sound**: Plays upon reaching the goal
- **Damage Sound**: Plays when taking damage
- **Heartbeat Sound**: Plays alongside damage for tension and during idle penalty

### Audio Files Required
Place in `assets/audio/`:
- `ambiental.wav` - Background music
- `start.wav` - Game start sound
- `footstep.wav` - Walking sound
- `victory.wav` - Win sound
- `damage.wav` - Damage sound
- `heartbeat.wav` - Heartbeat sound

## Technical Architecture

### Project Structure
```
src/
├── main.rs          - Main game loop and window management
├── game.rs          - Game state management
├── player.rs        - Player movement and collision
├── maze.rs          - Maze loading and collision detection
├── caster.rs        - Raycasting algorithm
├── camera.rs        - Mouse-based camera controls
├── framebuffer.rs   - Custom rendering buffer
├── textures.rs      - Texture management system
├── audio.rs         - Audio manager (with footstep control)
├── sprite.rs        - Sprite rendering and animation
├── pill.rs          - Pill system (red/blue pills with effects)
├── minimap.rs       - Minimap rendering
├── ui.rs            - UI rendering (menu, HUD, victory, timer)
├── effects.rs       - Visual effects system (damage, anxiety)
└── enemy.rs         - Enemy system (future expansion)
```

### Performance Optimizations
- **Ray Count**: 80 rays (scaled to 640px screen)
- **Texture Size**: Configurable (64x64 for performance)
- **Framebuffer**: Custom CPU-based rendering for control
- **Sprite Culling**: Distance-based rendering threshold
- **Optimized Collision**: Simple radius-based collision detection

## Game Features

### Player System
- **Health**: 100 HP (max)
- **Movement Speed**: 3.0 units/second
- **Rotation Speed**: 2.5 radians/second
- **Collision Radius**: 0.3 units
- **Game Timer**: 3 minutes (180 seconds) to reach the exit

### Pill System
The game features a risk/reward pill system scattered throughout the maze:

- **Red Pill** (Bad):
  - **Effect**: -15 HP damage
  - **Penalty**: Triggers anxiety visual effect (screen distortion)
  - **Visual**: Red glow with pulsing animation

- **Blue Pill** (Mixed):
  - **Benefit**: +10 HP healing
  - **Penalty**: -20 seconds from game timer
  - **Visual**: Blue glow with pulsing animation
  - **Trade-off**: Players must decide if the health boost is worth losing time

Both pills display floating text feedback showing their effects when collected.

### Idle Penalty System
- If the player stands still for **5 seconds**, they take damage
- Triggers anxiety visual effect and heartbeat sound
- Encourages constant movement and exploration
- Resets when player moves again

### Maze System
- Loaded from `maze.txt`
- Configurable tile size
- Multiple tile types:
  - `#` - Wall
  - ` ` - Floor
  - `S` - Start position
  - `G` - Goal/Exit position
  - `p` - pill sprite


### State Management
```rust
enum State {
    Menu,     // Welcome screen
    Playing,  // Active gameplay
    Victory,  // Win screen
}
```

## Building & Running

### Prerequisites
- Rust (latest stable version)
- Cargo
- Raylib dependencies for your platform

### Build & Run
```bash
# Development build
cargo run

# Release build (optimized)
cargo build --release
cargo run --release
```

### Performance
- **Target FPS**: 60
- **Typical Performance**: Maintains 60 FPS on modern hardware
- **Screen Resolution**: 640x480 (configurable)

## Dependencies

```toml
[dependencies]
raylib = "5.5.1"

[profile.release]
opt-level = 3
lto = true
```

## Aesthetic Design

### Backrooms Theme
- **Color Palette**: Yellow/beige walls, muted lighting
- **Atmosphere**: Liminal space horror aesthetic
- **Lighting**: Fluorescent-style overhead lighting simulation
- **Fog**: Dense yellowish fog for unease
- **Flickering Lights**: Animated sprite lights throughout the maze

### UI Design
- **Menu**: Clean text-based interface with glowing title effect
- **HUD**:
  - Health bar (top-left)
  - Countdown timer (top-center, color-coded: green > 60s, yellow 30-60s, red < 30s)
  - FPS counter (top-right)
  - Floating text feedback for pill collection
  - Minimalist design to not obstruct gameplay
- **Victory Screen**: Celebratory message with replay option

## Configuration

### Adjustable Parameters
```rust
// main.rs
const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 480;
const TARGET_FPS: u32 = 60;

// game.rs
textures: 64x64 pixels
num_rays: 80
max_depth: 20.0 units

// effects.rs
fog_distance: 15.0
flashlight_intensity: 1.0

// audio.rs
music_volume: 0.6
sfx_volume: 0.7
```

