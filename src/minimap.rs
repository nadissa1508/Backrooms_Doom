use raylib::prelude::*;
use crate::maze::Maze;
use crate::player::Player;

pub struct Minimap {
    pub size: i32,
    pub scale: f32,
    pub position: (i32, i32), // Screen position (top-right corner)
}

impl Minimap {
    pub fn new(size: i32, scale: f32, position: (i32, i32)) -> Self {
        Self {
            size,
            scale,
            position,
        }
    }

    /// Render the minimap showing the ENTIRE map at all times
    pub fn render(&self, d: &mut RaylibDrawHandle, maze: &Maze, player: &Player) {
        let (x_offset, y_offset) = self.position;

        // Draw semi-transparent background
        d.draw_rectangle(
            x_offset,
            y_offset,
            self.size,
            self.size,
            Color::new(0, 0, 0, 180),
        );

        // Draw border
        d.draw_rectangle_lines(
            x_offset,
            y_offset,
            self.size,
            self.size,
            Color::WHITE,
        );

        // Calculate scale to fit entire map in the minimap square
        let map_width_pixels = maze.width as f32 * maze.tile_size;
        let map_height_pixels = maze.height as f32 * maze.tile_size;
        let max_dimension = map_width_pixels.max(map_height_pixels);

        // Scale to fit the entire map in the minimap
        let map_scale = (self.size as f32 - 4.0) / max_dimension; // -4 for padding

        // Calculate centered offset for non-square maps
        let map_render_width = maze.width as f32 * maze.tile_size * map_scale;
        let map_render_height = maze.height as f32 * maze.tile_size * map_scale;
        let map_x_offset = x_offset + ((self.size as f32 - map_render_width) / 2.0) as i32;
        let map_y_offset = y_offset + ((self.size as f32 - map_render_height) / 2.0) as i32;

        // Draw ALL map tiles
        for map_y in 0..maze.height {
            for map_x in 0..maze.width {
                let tile = maze.get_tile(map_x, map_y);

                let screen_x = map_x_offset + (map_x as f32 * maze.tile_size * map_scale) as i32;
                let screen_y = map_y_offset + (map_y as f32 * maze.tile_size * map_scale) as i32;
                let tile_pixel_size = (maze.tile_size * map_scale).max(1.0) as i32;

                let color = match tile {
                    Some('#') => Color::new(60, 60, 60, 255),     // Wall - dark gray
                    Some('G') => Color::new(255, 0, 0, 255),   // Goal - red (exit door)
                    Some('S') => Color::new(100, 200, 100, 255),  // Start - green
                    _ => Color::new(180, 180, 140, 255),          // Floor - light
                };

                d.draw_rectangle(
                    screen_x,
                    screen_y,
                    tile_pixel_size,
                    tile_pixel_size,
                    color,
                );
            }
        }

        // Draw player as a BLUE DOT that moves on the map
        let player_screen_x = map_x_offset + (player.pos.x * map_scale) as i32;
        let player_screen_y = map_y_offset + (player.pos.y * map_scale) as i32;
        let player_dot_size = 5.0;

        // Draw blue dot for player (no direction indicator)
        d.draw_circle(
            player_screen_x,
            player_screen_y,
            player_dot_size,
            Color::new(0, 150, 255, 255), // Bright blue
        );
    }
}
