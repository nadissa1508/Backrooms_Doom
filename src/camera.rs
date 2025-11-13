use raylib::prelude::*;
use crate::player::Player;

pub struct Camera {
    pub sensitivity: f32,
    pub last_mouse_pos: Vector2,
    pub is_first_frame: bool,
}

impl Camera {
    pub fn new(sensitivity: f32) -> Self {
        Self {
            sensitivity,
            last_mouse_pos: Vector2::zero(),
            is_first_frame: true,
        }
    }

    /// Update player rotation based on mouse movement
    pub fn update(&mut self, rl: &RaylibHandle, player: &mut Player, delta_time: f32) {
        let mouse_pos = rl.get_mouse_position();

        if self.is_first_frame {
            self.last_mouse_pos = mouse_pos;
            self.is_first_frame = false;
            return;
        }

        // Calculate mouse delta
        let delta_x = mouse_pos.x - self.last_mouse_pos.x;

        // Apply rotation based on mouse movement
        if delta_x.abs() > 0.1 {
            player.rotate(delta_x * self.sensitivity * delta_time);
        }

        self.last_mouse_pos = mouse_pos;
    }

    /// Reset camera state (useful when entering/exiting menus)
    pub fn reset(&mut self) {
        self.is_first_frame = true;
    }
}
