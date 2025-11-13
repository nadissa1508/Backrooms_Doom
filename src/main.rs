mod audio;
mod camera;
mod caster;
mod effects;
mod enemy;
mod framebuffer;
mod game;
mod maze;
mod minimap;
mod player;
mod sprite;
mod textures;
mod ui;

use audio::AudioManager;
use game::{GameState, State};
use raylib::prelude::*;

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 480;
const TARGET_FPS: u32 = 60;

fn main() {
    // Initialize raylib
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Backrooms Doom - Raycaster")
        .build();

    // Initialize audio device
    let audio = match RaylibAudio::init_audio_device() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Failed to initialize audio: {:?}", e);
            eprintln!("Continuing without audio...");
            return;
        }
    };
    audio.set_master_volume(1.0);
    println!("✓ Audio device initialized");

    // Set target FPS
    rl.set_target_fps(TARGET_FPS);

    // Hide cursor for immersive experience
    rl.hide_cursor();

    // Initialize game state with audio
    let mut game = match GameState::new(SCREEN_WIDTH, SCREEN_HEIGHT, &audio) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Failed to initialize game: {}", e);
            return;
        }
    };

    let mut last_state = game.state;

    // Play menu music on startup
    game.audio.play_menu_music();

    // Main game loop
    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        // Handle state transitions for audio
        if game.state != last_state {
            match game.state {
                State::Playing => {
                    // Start ambient music when gameplay begins
                    game.audio.play_background_music();
                }
                State::Victory => {
                    // Stop all music and play victory sound
                    game.audio.stop_music();
                    game.audio.play_victory();
                }
                State::GameOver => {
                    // Stop all music when game over
                    game.audio.stop_music();
                    // Could add a game over sound here if you have one
                }
                State::Menu => {
                    // Stop gameplay music and play menu music
                    game.audio.stop_music();
                    game.audio.play_menu_music();
                }
            }
            last_state = game.state;
        }

        // Update music stream
        game.audio.update_music();

        // Update music volume based on distance to goal (dynamic volume)
        if game.state == State::Playing {
            let dx = game.player.pos.x - game.maze.goal_pos.0;
            let dy = game.player.pos.y - game.maze.goal_pos.1;
            let distance_to_goal = (dx * dx + dy * dy).sqrt();
            game.audio.update_ambient_volume(distance_to_goal);
        }

        // Play footstep sounds only when moving
        if game.state == State::Playing {
            let is_moving = rl.is_key_down(KeyboardKey::KEY_W)
                || rl.is_key_down(KeyboardKey::KEY_S)
                || rl.is_key_down(KeyboardKey::KEY_A)
                || rl.is_key_down(KeyboardKey::KEY_D);

            if is_moving {
                game.audio.play_footstep(delta_time);
            } else {
                // Reset timer when not moving to prevent delayed footsteps
                game.audio.reset_footstep_timer();
            }
        }

        // Store previous anxiety intensity to detect triggers
        let prev_anxiety = game.effects.anxiety_intensity;

        // Update game state
        game.update(&rl, delta_time);

        // Check if anxiety effect was just triggered (idle penalty)
        if game.state == State::Playing && game.effects.anxiety_intensity > prev_anxiety && prev_anxiety == 0.0 {
            // Play heartbeat sound when anxiety effect triggers
            game.audio.play_heartbeat();
        }

        // Render
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        game.render(&mut d);

        // Debug info (optional - can be toggled with F3)
        if d.is_key_down(KeyboardKey::KEY_F3) {
            d.draw_text(
                &format!(
                    "Player Pos: ({:.2}, {:.2})",
                    game.player.pos.x, game.player.pos.y
                ),
                10,
                SCREEN_HEIGHT as i32 - 60,
                16,
                Color::YELLOW,
            );
            d.draw_text(
                &format!("Player Angle: {:.2}°", game.player.angle.to_degrees()),
                10,
                SCREEN_HEIGHT as i32 - 40,
                16,
                Color::YELLOW,
            );
            d.draw_text(
                &format!("Delta Time: {:.4}s", delta_time),
                10,
                SCREEN_HEIGHT as i32 - 20,
                16,
                Color::YELLOW,
            );
        }
    }
}
