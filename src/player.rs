use crate::maze::Maze;

#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct Player {
    pub pos: Vector2,
    pub angle: f32,
    pub health: i32,
    pub max_health: i32,
    pub move_speed: f32,
    pub rot_speed: f32,
    pub collision_radius: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vector2::new(x, y),
            angle: 0.0,
            health: 100,
            max_health: 100,
            move_speed: 3.0,
            rot_speed: 2.5,
            collision_radius: 0.3,
        }
    }

    /// Move forward in the direction the player is facing
    pub fn move_forward(&mut self, maze: &Maze, delta_time: f32) {
        let new_x = self.pos.x + self.angle.cos() * self.move_speed * delta_time;
        let new_y = self.pos.y + self.angle.sin() * self.move_speed * delta_time;

        if self.check_collision(maze, new_x, new_y) {
            self.pos.x = new_x;
            self.pos.y = new_y;
        }
    }

    /// Move backward (opposite of facing direction)
    pub fn move_backward(&mut self, maze: &Maze, delta_time: f32) {
        let new_x = self.pos.x - self.angle.cos() * self.move_speed * delta_time;
        let new_y = self.pos.y - self.angle.sin() * self.move_speed * delta_time;

        if self.check_collision(maze, new_x, new_y) {
            self.pos.x = new_x;
            self.pos.y = new_y;
        }
    }

    /// Strafe left (perpendicular to facing direction)
    pub fn move_left(&mut self, maze: &Maze, delta_time: f32) {
        let new_x = self.pos.x + (self.angle - std::f32::consts::PI / 2.0).cos() * self.move_speed * delta_time;
        let new_y = self.pos.y + (self.angle - std::f32::consts::PI / 2.0).sin() * self.move_speed * delta_time;

        if self.check_collision(maze, new_x, new_y) {
            self.pos.x = new_x;
            self.pos.y = new_y;
        }
    }

    /// Strafe right
    pub fn move_right(&mut self, maze: &Maze, delta_time: f32) {
        let new_x = self.pos.x + (self.angle + std::f32::consts::PI / 2.0).cos() * self.move_speed * delta_time;
        let new_y = self.pos.y + (self.angle + std::f32::consts::PI / 2.0).sin() * self.move_speed * delta_time;

        if self.check_collision(maze, new_x, new_y) {
            self.pos.x = new_x;
            self.pos.y = new_y;
        }
    }

    /// Rotate player view
    pub fn rotate(&mut self, delta_angle: f32) {
        self.angle += delta_angle;
        // Normalize angle to [0, 2π]
        while self.angle < 0.0 {
            self.angle += 2.0 * std::f32::consts::PI;
        }
        while self.angle >= 2.0 * std::f32::consts::PI {
            self.angle -= 2.0 * std::f32::consts::PI;
        }
    }

    /// Check collision with walls using circular collision detection
    fn check_collision(&self, maze: &Maze, new_x: f32, new_y: f32) -> bool {
        // Check multiple points around the player's collision circle
        let angles = [0.0, std::f32::consts::PI / 4.0, std::f32::consts::PI / 2.0,
                      3.0 * std::f32::consts::PI / 4.0, std::f32::consts::PI,
                      5.0 * std::f32::consts::PI / 4.0, 3.0 * std::f32::consts::PI / 2.0,
                      7.0 * std::f32::consts::PI / 4.0];

        for &angle in &angles {
            let check_x = new_x + angle.cos() * self.collision_radius;
            let check_y = new_y + angle.sin() * self.collision_radius;

            if !maze.is_walkable(check_x, check_y) {
                return false;
            }
        }

        true
    }

    /// Take damage
    pub fn take_damage(&mut self, amount: i32) {
        self.health = (self.health - amount).max(0);
    }

    /// Heal player
    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    /// Check if player is alive
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}