// Audio Manager for Backrooms Doom
// Handles all audio playback using raylib-rs 5.5.1 API

use raylib::prelude::*;
use std::path::Path;

pub struct AudioManager<'a> {
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub volume_multiplier: f32,
    pub footstep_timer: f32,
    pub music_playing: bool,

    // Loaded audio with lifetime bound to RaylibAudio
    pub ambient: Option<Music<'a>>,
    pub start: Option<Sound<'a>>,
    pub footstep: Option<Sound<'a>>,
    pub damage: Option<Sound<'a>>,
    pub heartbeat: Option<Sound<'a>>,
    pub victory: Option<Sound<'a>>,
}

impl<'a> AudioManager<'a> {
    pub fn new(audio: &'a RaylibAudio) -> Self {
        let mut files_present = false;

        // Load music
        let ambient = if Path::new("assets/audio/ambiental.wav").exists() {
            match audio.new_music("assets/audio/ambiental.wav") {
                Ok(music) => {
                    println!("Loaded: ambiental.wav");
                    files_present = true;
                    Some(music)
                }
                Err(e) => {
                    println!("Could not load ambiental.wav: {}", e);
                    None
                }
            }
        } else {
            None
        };

        // Load sounds
        let start = if Path::new("assets/audio/start.wav").exists() {
            match audio.new_sound("assets/audio/start.wav") {
                Ok(sound) => {
                    println!("Loaded: start.wav");
                    files_present = true;
                    Some(sound)
                }
                Err(_) => {
                    println!("Could not load start.wav");
                    None
                }
            }
        } else {
            None
        };

        let footstep = if Path::new("assets/audio/footstep.wav").exists() {
            match audio.new_sound("assets/audio/footstep.wav") {
                Ok(sound) => {
                    println!("Loaded: footstep.wav");
                    files_present = true;
                    Some(sound)
                }
                Err(_) => {
                    println!("Could not load footstep.wav");
                    None
                }
            }
        } else {
            None
        };

        let damage = if Path::new("assets/audio/damage.wav").exists() {
            match audio.new_sound("assets/audio/damage.wav") {
                Ok(sound) => {
                    println!("Loaded: damage.wav");
                    files_present = true;
                    Some(sound)
                }
                Err(_) => None,
            }
        } else {
            None
        };

        let heartbeat = if Path::new("assets/audio/heartbeat.wav").exists() {
            match audio.new_sound("assets/audio/heartbeat.wav") {
                Ok(sound) => {
                    println!("Loaded: heartbeat.wav");
                    files_present = true;
                    Some(sound)
                }
                Err(_) => None,
            }
        } else {
            None
        };

        let victory = if Path::new("assets/audio/victory.wav").exists() {
            match audio.new_sound("assets/audio/victory.wav") {
                Ok(sound) => {
                    println!("Loaded: victory.wav");
                    files_present = true;
                    Some(sound)
                }
                Err(_) => {
                    println!("Could not load victory.wav");
                    None
                }
            }
        } else {
            None
        };

        if !files_present {
            println!("No audio files found in assets/audio/");
            println!("Add WAV files to enable audio (see assets/audio/README.md)");
        }

        Self {
            music_volume: 0.6,
            sfx_volume: 0.7,
            volume_multiplier: 1.0,
            footstep_timer: 0.0,
            music_playing: false,
            ambient,
            start,
            footstep,
            damage,
            heartbeat,
            victory,
        }
    }

    /// Play menu music (start.wav)
    pub fn play_menu_music(&self) {
        if let Some(ref sound) = self.start {
            sound.play();
            println!("Playing menu music (start.wav)");
        }
    }

    /// Start playing background music
    pub fn play_background_music(&mut self) {
        if let Some(ref mut music) = self.ambient {
            music.play_stream();
            music.set_volume(self.music_volume * self.volume_multiplier);
            self.music_playing = true;
            println!("Playing background music");
        }
    }

    /// Stop background music
    pub fn stop_music(&mut self) {
        if let Some(ref mut music) = self.ambient {
            music.stop_stream();
            self.music_playing = false;
            println!("Stopped background music");
        }
    }

    /// Update music stream (call every frame during gameplay)
    pub fn update_music(&mut self) {
        if self.music_playing {
            if let Some(ref mut music) = self.ambient {
                music.update_stream();
            }
        }
    }

    /// Update music volume based on distance to goal
    pub fn update_ambient_volume(&mut self, distance_to_goal: f32) {
        if self.music_playing {
            if let Some(ref mut music) = self.ambient {
                let intensity = 1.0 - (distance_to_goal / 20.0).min(1.0);
                let volume = (self.music_volume + intensity * 0.4) * self.volume_multiplier;
                music.set_volume(volume.min(1.0));
            }
        }
    }

    /// Play footstep sound (with automatic timing) - only when moving
    pub fn play_footstep(&mut self, delta_time: f32) {
        self.footstep_timer += delta_time;
        if self.footstep_timer >= 0.5 {
            self.footstep_timer = 0.0;
            if let Some(ref sound) = self.footstep {
                sound.play();
            }
        }
    }

    /// Reset footstep timer (call when player stops moving)
    pub fn reset_footstep_timer(&mut self) {
        self.footstep_timer = 0.0;
    }

    /// Stop footstep sound (call when player stops moving)
    pub fn stop_footstep(&self) {
        if let Some(ref sound) = self.footstep {
            sound.stop();
        }
    }

    /// Play damage sound with heartbeat
    pub fn play_damage(&self) {
        if let Some(ref sound) = self.damage {
            sound.play();
        }
        if let Some(ref sound) = self.heartbeat {
            sound.play();
        }
        println!("Playing damage + heartbeat");
    }

    /// Play victory sound effect
    pub fn play_victory(&self) {
        if let Some(ref sound) = self.victory {
            sound.play();
            println!("Playing victory sound");
        }
    }

    /// Play heartbeat sound (for idle penalty/anxiety)
    pub fn play_heartbeat(&self) {
        if let Some(ref sound) = self.heartbeat {
            sound.play();
            println!("Playing heartbeat (idle penalty)");
        }
    }
}