// Enemy module (placeholder for future expansion)
// Can be used to add hostile entities in the Backrooms

use crate::player::Vector2;

pub struct Enemy {
    pub pos: Vector2,
    pub health: i32,
    pub speed: f32,
}

impl Enemy {
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vector2::new(x, y),
            health: 100,
            speed: 1.0,
        }
    }
}
