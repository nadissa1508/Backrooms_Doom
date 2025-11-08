use raylib::prelude::*;

pub struct Effects {
    pub fog_enabled: bool,
    pub fog_distance: f32,
    pub fog_color: Color,
    pub flashlight_enabled: bool,
    pub flashlight_intensity: f32,
    pub damage_flash_timer: f32,
}

impl Effects {
    pub fn new() -> Self {
        Self {
            fog_enabled: true,
            fog_distance: 15.0,
            fog_color: Color::new(80, 75, 50, 255), // Yellowish Backrooms fog (fluorescent lighting feel)
            flashlight_enabled: true,
            flashlight_intensity: 1.0,
            damage_flash_timer: 0.0,
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
    }

    /// Trigger damage flash
    pub fn trigger_damage_flash(&mut self) {
        self.damage_flash_timer = 0.3;
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