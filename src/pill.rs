use crate::player::Vector2;
use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PillType {
    Red,   // Bad: -15 HP, anxiety effect
    Blue,  // Good: +10 HP, Bad: -20 seconds on timer
}

pub struct Pill {
    pub pos: Vector2,
    pub pill_type: PillType,
    pub collected: bool,
    pub glow_timer: f32, // For animation
}

impl Pill {
    pub fn new(x: f32, y: f32, pill_type: PillType) -> Self {
        Self {
            pos: Vector2::new(x, y),
            pill_type,
            collected: false,
            glow_timer: 0.0,
        }
    }

    /// Update pill animation
    pub fn update(&mut self, delta_time: f32) {
        if !self.collected {
            self.glow_timer += delta_time * 2.0;
        }
    }

    /// Check if player is close enough to collect the pill
    pub fn can_collect(&self, player_x: f32, player_y: f32, collect_radius: f32) -> bool {
        if self.collected {
            return false;
        }

        let dx = self.pos.x - player_x;
        let dy = self.pos.y - player_y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        distance < collect_radius
    }

    /// Get glow color based on pill type
    pub fn get_glow_color(&self) -> Color {
        let pulse = (self.glow_timer.sin() * 0.3 + 0.7) as f32;
        match self.pill_type {
            PillType::Red => Color::new(
                (255.0 * pulse) as u8,
                (50.0 * pulse) as u8,
                (50.0 * pulse) as u8,
                200
            ),
            PillType::Blue => Color::new(
                (50.0 * pulse) as u8,
                (150.0 * pulse) as u8,
                (255.0 * pulse) as u8,
                200
            ),
        }
    }

    /// Get base color for the pill
    pub fn get_color(&self) -> Color {
        match self.pill_type {
            PillType::Red => Color::new(255, 50, 50, 255),
            PillType::Blue => Color::new(50, 150, 255, 255),
        }
    }
}

/// Floating text for pill collection feedback
pub struct FloatingText {
    pub text: String,
    pub pos: Vector2,
    pub color: Color,
    pub lifetime: f32,
    pub velocity_y: f32,
    pub z: f32, // Vertical position for 3D rendering
}

impl FloatingText {
    pub fn new(text: String, x: f32, y: f32, color: Color) -> Self {
        Self {
            text,
            pos: Vector2::new(x, y),
            color,
            lifetime: 1.5, // 1.5 seconds
            velocity_y: -50.0, // Float upward
            z: 0.0, // Start at ground level
        }
    }

    /// Update floating text position and lifetime
    pub fn update(&mut self, delta_time: f32) {
        self.lifetime -= delta_time;
        self.z += 1.0 * delta_time; // Float upward in 3D space
        
        // Fade out as lifetime decreases
        let alpha = (self.lifetime / 1.5 * 255.0) as u8;
        self.color.a = alpha;
    }

    /// Check if text should be removed
    pub fn is_expired(&self) -> bool {
        self.lifetime <= 0.0
    }
}
