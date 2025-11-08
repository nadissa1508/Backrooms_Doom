use crate::maze::Maze;
use crate::player::Player;

pub struct RayHit {
    pub distance: f32,
    pub wall_x: f32,      // Texture coordinate (0.0 to 1.0)
    pub hit_vertical: bool, // For texture shading
    pub map_x: usize,
    pub map_y: usize,
}

pub struct RayCaster {
    pub fov: f32,
    pub num_rays: usize,
    pub max_depth: f32,
    // Precomputed values for performance
    ray_angles: Vec<f32>,
}

impl RayCaster {
    pub fn new(fov: f32, num_rays: usize, max_depth: f32) -> Self {
        let mut ray_angles = Vec::with_capacity(num_rays);
        let angle_step = fov / num_rays as f32;
        let half_fov = fov / 2.0;

        for i in 0..num_rays {
            ray_angles.push(-half_fov + angle_step * i as f32);
        }

        Self {
            fov,
            num_rays,
            max_depth,
            ray_angles,
        }
    }

    /// Cast all rays and return hit information
    pub fn cast_rays(&self, player: &Player, maze: &Maze) -> Vec<RayHit> {
        self.ray_angles
            .iter()
            .map(|&ray_offset| {
                let ray_angle = player.angle + ray_offset;
                self.cast_single_ray(player.pos.x, player.pos.y, ray_angle, maze)
            })
            .collect()
    }

    /// Cast a single ray using DDA algorithm (optimized)
    fn cast_single_ray(&self, origin_x: f32, origin_y: f32, angle: f32, maze: &Maze) -> RayHit {
        let dir_x = angle.cos();
        let dir_y = angle.sin();

        // Starting grid position
        let mut map_x = (origin_x / maze.tile_size) as i32;
        let mut map_y = (origin_y / maze.tile_size) as i32;

        // Distance to travel from one grid line to the next
        let delta_dist_x = if dir_x == 0.0 { f32::MAX } else { (1.0 / dir_x).abs() };
        let delta_dist_y = if dir_y == 0.0 { f32::MAX } else { (1.0 / dir_y).abs() };

        // Step direction
        let step_x: i32;
        let step_y: i32;

        // Initial side distances
        let mut side_dist_x: f32;
        let mut side_dist_y: f32;

        if dir_x < 0.0 {
            step_x = -1;
            side_dist_x = (origin_x / maze.tile_size - map_x as f32) * delta_dist_x;
        } else {
            step_x = 1;
            side_dist_x = (map_x as f32 + 1.0 - origin_x / maze.tile_size) * delta_dist_x;
        }

        if dir_y < 0.0 {
            step_y = -1;
            side_dist_y = (origin_y / maze.tile_size - map_y as f32) * delta_dist_y;
        } else {
            step_y = 1;
            side_dist_y = (map_y as f32 + 1.0 - origin_y / maze.tile_size) * delta_dist_y;
        }

        // DDA algorithm
        let mut hit = false;
        let mut hit_vertical = false;

        while !hit {
            // Jump to next grid square
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                hit_vertical = true;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                hit_vertical = false;
            }

            // Check if ray hit a wall
            if map_x < 0 || map_y < 0 || map_x >= maze.width as i32 || map_y >= maze.height as i32 {
                // Out of bounds - treat as wall
                break;
            }

            if maze.is_wall(map_x as usize, map_y as usize) {
                hit = true;
            }
        }

        // Calculate distance (perpendicular to camera plane to avoid fisheye)
        let distance = if hit_vertical {
            (map_x as f32 - origin_x / maze.tile_size + (1.0 - step_x as f32) / 2.0) / dir_x
        } else {
            (map_y as f32 - origin_y / maze.tile_size + (1.0 - step_y as f32) / 2.0) / dir_y
        };

        // Calculate exact hit position for texture mapping
        let wall_x = if hit_vertical {
            origin_y / maze.tile_size + distance * dir_y
        } else {
            origin_x / maze.tile_size + distance * dir_x
        };
        let wall_x = wall_x - wall_x.floor(); // Get fractional part [0, 1]

        RayHit {
            distance: distance.abs() * maze.tile_size,
            wall_x,
            hit_vertical,
            map_x: map_x as usize,
            map_y: map_y as usize,
        }
    }
}