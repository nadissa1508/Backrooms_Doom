use raylib::prelude::*;

pub struct Effects {
    pub fog_enabled: bool,
    pub fog_distance: f32,
    pub fog_color: Color,
    pub flashlight_enabled: bool,
    pub flashlight_intensity: f32,
    pub damage_flash_timer: f32,
    // Anxiety effect fields
    pub anxiety_intensity: f32,  // 0.0 to 1.0
    pub anxiety_timer: f32,       // Duration of anxiety effect
    pub screen_shake_offset: (f32, f32), // Random offset for screen shake
}

impl Effects {
    pub fn new() -> Self {
        Self {
            fog_enabled: false, // Fog disabled
            fog_distance: 15.0,
            fog_color: Color::new(80, 75, 50, 255), // Yellowish Backrooms fog (fluorescent lighting feel)
            flashlight_enabled: false, // Flashlight disabled
            flashlight_intensity: 1.0,
            damage_flash_timer: 0.0,
            anxiety_intensity: 0.0,
            anxiety_timer: 0.0,
            screen_shake_offset: (0.0, 0.0),
        }
    }

    /// Apply fog effect to a color based on distance
    pub fn apply_fog(&self, color: Color, distance: f32) -> Color {
        if !self.fog_enabled {
            return color;
        }

        let fog_factor = (distance / self.fog_distance).clamp(0.0, 1.0);

        Color::new(
            ((1.0 - fog_factor) * color.r as f32 + fog_factor * self.fog_color.r as f32) as u8,
            ((1.0 - fog_factor) * color.g as f32 + fog_factor * self.fog_color.g as f32) as u8,
            ((1.0 - fog_factor) * color.b as f32 + fog_factor * self.fog_color.b as f32) as u8,
            255,
        )
    }

    /// Apply flashlight effect (brightens center, darkens edges)
    pub fn apply_flashlight(&self, color: Color, screen_x: usize, screen_width: usize) -> Color {
        if !self.flashlight_enabled {
            return color;
        }

        let center = screen_width as f32 / 2.0;
        let distance_from_center = ((screen_x as f32 - center).abs() / center).clamp(0.0, 1.0);
        let brightness = (1.0 - distance_from_center * 0.5) * self.flashlight_intensity;

        Color::new(
            (color.r as f32 * brightness) as u8,
            (color.g as f32 * brightness) as u8,
            (color.b as f32 * brightness) as u8,
            255,
        )
    }

    /// Apply damage flash effect
    pub fn apply_damage_flash(&self, color: Color) -> Color {
        if self.damage_flash_timer <= 0.0 {
            return color;
        }

        let flash_intensity = (self.damage_flash_timer * 255.0) as u8;

        Color::new(
            color.r.saturating_add(flash_intensity),
            color.g.saturating_sub(flash_intensity / 2),
            color.b.saturating_sub(flash_intensity / 2),
            255,
        )
    }

    /// Update damage flash timer
    pub fn update(&mut self, delta_time: f32) {
        if self.damage_flash_timer > 0.0 {
            self.damage_flash_timer -= delta_time * 2.0;
            self.damage_flash_timer = self.damage_flash_timer.max(0.0);
        }

        // Update anxiety effect
        if self.anxiety_timer > 0.0 {
            self.anxiety_timer -= delta_time;
            self.anxiety_timer = self.anxiety_timer.max(0.0);
            
            // Fade out anxiety intensity as timer decreases
            self.anxiety_intensity = (self.anxiety_timer / 2.0).min(1.0);
            
            // Update screen shake with random offset
            if self.anxiety_intensity > 0.0 {
                use std::f32::consts::PI;
                let time_factor = self.anxiety_timer * 10.0;
                self.screen_shake_offset = (
                    (time_factor.sin() * 2.0 + (time_factor * 2.3).cos() * 1.5) * self.anxiety_intensity,
                    (time_factor.cos() * 2.0 + (time_factor * 1.7).sin() * 1.5) * self.anxiety_intensity,
                );
            } else {
                self.screen_shake_offset = (0.0, 0.0);
            }
        } else {
            self.anxiety_intensity = 0.0;
            self.screen_shake_offset = (0.0, 0.0);
        }
    }

    /// Trigger damage flash
    pub fn trigger_damage_flash(&mut self) {
        self.damage_flash_timer = 0.3;
    }

    /// Trigger anxiety effect (idle penalty)
    pub fn trigger_anxiety_effect(&mut self) {
        self.anxiety_timer = 2.0; // 2 seconds of anxiety effect
        self.anxiety_intensity = 1.0;
    }

    /// Apply vignette effect (darkened edges) for anxiety
    pub fn apply_anxiety_vignette(&self, color: Color, screen_x: usize, screen_y: usize, screen_width: usize, screen_height: usize) -> Color {
        if self.anxiety_intensity <= 0.0 {
            return color;
        }

        let center_x = screen_width as f32 / 2.0;
        let center_y = screen_height as f32 / 2.0;
        
        let dx = (screen_x as f32 - center_x) / center_x;
        let dy = (screen_y as f32 - center_y) / center_y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        // Stronger vignette effect based on anxiety
        let vignette_strength = (distance * self.anxiety_intensity * 0.7).min(0.8);
        
        Color::new(
            (color.r as f32 * (1.0 - vignette_strength)) as u8,
            (color.g as f32 * (1.0 - vignette_strength)) as u8,
            (color.b as f32 * (1.0 - vignette_strength)) as u8,
            255,
        )
    }

    /// Calculate shading based on wall orientation
    pub fn calculate_shading(&self, hit_vertical: bool) -> f32 {
        if hit_vertical {
            0.95 // Much brighter for Backrooms fluorescent feel
        } else {
            1.0 // Full brightness for horizontal walls
        }
    }

    /// Apply distance-based shading
    pub fn calculate_distance_shading(&self, distance: f32, max_distance: f32) -> f32 {
        let normalized = (distance / max_distance).clamp(0.0, 1.0);
        1.0 - normalized * 0.25 // Very minimal darkening for Backrooms bright lighting
    }
}