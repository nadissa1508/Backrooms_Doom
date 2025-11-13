use std::fs;

pub struct Maze {
    pub map: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
    pub tile_size: f32,
    pub start_pos: (f32, f32),
    pub goal_pos: (f32, f32),
    pub pill_positions: Vec<(f32, f32)>, // Positions where 'p' was found
}

impl Maze {
    /// Load maze from file (e.g., "maze.txt")
    pub fn load_from_file(path: &str, tile_size: f32) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read maze file: {}", e))?;

        let mut map: Vec<Vec<char>> = content
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();

        if map.is_empty() {
            return Err("Maze file is empty".to_string());
        }

        let height = map.len();
        let width = map[0].len();

        // Find start (S) and goal (E - exit door) positions
        let mut start_pos = (1.5 * tile_size, 1.5 * tile_size);
        let mut goal_pos = (1.5 * tile_size, 1.5 * tile_size);
        let mut pill_positions = Vec::new();

        for (y, row) in map.iter_mut().enumerate() {
            for (x, tile) in row.iter_mut().enumerate() {
                if *tile == 'S' {
                    start_pos = ((x as f32 + 0.5) * tile_size, (y as f32 + 0.5) * tile_size);
                } else if *tile == 'E' {
                    goal_pos = ((x as f32 + 0.5) * tile_size, (y as f32 + 0.5) * tile_size);
                } else if *tile == 'p' {
                    // Found a pill spawn location
                    pill_positions.push(((x as f32 + 0.5) * tile_size, (y as f32 + 0.5) * tile_size));
                    // Replace 'p' with '.' so it's walkable
                    *tile = '.';
                }
            }
        }

        Ok(Maze {
            map,
            width,
            height,
            tile_size,
            start_pos,
            goal_pos,
            pill_positions,
        })
    }

    /// Get tile at grid position (returns None if out of bounds)
    #[inline]
    pub fn get_tile(&self, x: usize, y: usize) -> Option<char> {
        self.map.get(y)?.get(x).copied()
    }

    /// Check if position is a wall (optimized for raycasting)
    /// Recognizes both normal walls '#' and exit doors 'E'
    #[inline]
    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        matches!(self.get_tile(x, y), Some('#') | Some('E'))
    }

    /// Get the type of wall at position
    /// Returns the character representing the wall type ('# for normal, 'E' for exit)
    /// Returns ' ' for non-wall tiles
    #[inline]
    pub fn get_wall_type(&self, x: usize, y: usize) -> char {
        match self.get_tile(x, y) {
            Some('#') => '#',
            Some('E') => 'E',
            _ => ' ',
        }
    }

    /// Check if world position is walkable (for collision detection)
    #[inline]
    pub fn is_walkable(&self, world_x: f32, world_y: f32) -> bool {
        let grid_x = (world_x / self.tile_size) as usize;
        let grid_y = (world_y / self.tile_size) as usize;
        !self.is_wall(grid_x, grid_y)
    }

    /// Check if player reached the goal
    #[inline]
    pub fn is_goal(&self, world_x: f32, world_y: f32, threshold: f32) -> bool {
        let dx = world_x - self.goal_pos.0;
        let dy = world_y - self.goal_pos.1;
        (dx * dx + dy * dy).sqrt() < threshold
    }
}