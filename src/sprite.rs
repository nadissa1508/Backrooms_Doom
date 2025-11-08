use crate::player::Vector2;

pub struct Sprite {
    pub pos: Vector2,
    pub texture_index: usize,
    pub animation_speed: f32,
    pub animation_timer: f32,
    pub num_frames: usize,
    pub current_frame: usize,
    pub scale: f32,
}

impl Sprite {
    pub fn new(x: f32, y: f32, texture_index: usize, num_frames: usize, animation_speed: f32) -> Self {
        Self {
            pos: Vector2::new(x, y),
            texture_index,
            animation_speed,
            animation_timer: 0.0,
            num_frames,
            current_frame: 0,
            scale: 1.0,
        }
    }

    /// Update sprite animation
    pub fn update(&mut self, delta_time: f32) {
        self.animation_timer += delta_time;

        if self.animation_timer >= self.animation_speed {
            self.animation_timer = 0.0;
            self.current_frame = (self.current_frame + 1) % self.num_frames;
        }
    }

    /// Create a flickering light sprite (for Backrooms atmosphere)
    pub fn new_flickering_light(x: f32, y: f32) -> Self {
        Self::new(x, y, 0, 4, 0.1) // Fast flicker
    }
}

pub struct SpriteRenderer {
    pub sprite_distance_threshold: f32,
}

impl SpriteRenderer {
    pub fn new() -> Self {
        Self {
            sprite_distance_threshold: 20.0,
        }
    }

    /// Calculate sprite distance from player for sorting
    pub fn calculate_distance(&self, sprite: &Sprite, player_x: f32, player_y: f32) -> f32 {
        let dx = sprite.pos.x - player_x;
        let dy = sprite.pos.y - player_y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Check if sprite is visible to player (basic frustum culling)
    pub fn is_visible(&self, sprite: &Sprite, player_x: f32, player_y: f32, player_angle: f32, fov: f32) -> bool {
        let dx = sprite.pos.x - player_x;
        let dy = sprite.pos.y - player_y;
        let angle_to_sprite = dy.atan2(dx);
        let angle_diff = (angle_to_sprite - player_angle).abs();

        angle_diff < fov / 2.0 + 0.5 // Add small margin
    }
}