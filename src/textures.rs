use raylib::prelude::*;
use std::collections::HashMap;

pub struct TextureManager {
    pub textures: HashMap<String, Vec<Color>>,
    pub texture_size: usize,
}

impl TextureManager {
    pub fn new(texture_size: usize) -> Self {
        let mut manager = Self {
            textures: HashMap::new(),
            texture_size,
        };

        // Generate procedural Backrooms-style textures
        manager.generate_backrooms_textures();
        manager
    }

    /// Generate Backrooms-themed textures procedurally
    fn generate_backrooms_textures(&mut self) {
        let size = self.texture_size;

        // Yellow wallpaper texture (main Backrooms aesthetic)
        let mut yellow_wall = Vec::with_capacity(size * size);
        for y in 0..size {
            for x in 0..size {
                // Create a subtle pattern
                let noise = ((x + y) % 16) as f32 / 16.0;
                let base_yellow: u8 = 230;
                let variation = (noise * 20.0) as u8;

                yellow_wall.push(Color::new(
                    base_yellow.saturating_sub(variation),
                    (base_yellow.saturating_sub(30)).saturating_sub(variation),
                    0,
                    255,
                ));
            }
        }
        self.textures.insert("wall".to_string(), yellow_wall);

        // Blue door texture (goal/exit)
        let mut blue_door = Vec::with_capacity(size * size);
        for y in 0..size {
            for x in 0..size {
                // Create door panels
                let is_panel = (x / (size / 4)) % 2 == 0 && (y / (size / 4)) % 2 == 0;
                let blue_val = if is_panel { 200 } else { 150 };

                blue_door.push(Color::new(30, 80, blue_val, 255));
            }
        }
        self.textures.insert("door".to_string(), blue_door);

        // Ceiling texture (off-white with panels)
        let mut ceiling = Vec::with_capacity(size * size);
        for y in 0..size {
            for x in 0..size {
                let is_seam = x % (size / 4) == 0 || y % (size / 4) == 0;
                let gray = if is_seam { 200 } else { 240 };

                ceiling.push(Color::new(gray, gray, gray - 10, 255));
            }
        }
        self.textures.insert("ceiling".to_string(), ceiling);

        // Floor texture (dull carpet)
        let mut floor = Vec::with_capacity(size * size);
        for y in 0..size {
            for x in 0..size {
                let noise = ((x * 7 + y * 13) % 32) as f32 / 32.0;
                let base = 140;
                let variation = (noise * 15.0) as u8;

                floor.push(Color::new(
                    base + variation,
                    (base + 20) + variation,
                    base + variation,
                    255,
                ));
            }
        }
        self.textures.insert("floor".to_string(), floor);
    }

    /// Load texture from raylib (for custom textures)
    /// Note: This is a placeholder - actual texture loading would require
    /// converting from Image to pixel data
    #[allow(dead_code)]
    pub fn load_texture(&mut self, name: &str, _rl: &mut RaylibHandle, _thread: &RaylibThread, _path: &str) {
        // For now, just create a default texture
        // In a real implementation, you would load the image file and convert its pixels
        let size = self.texture_size;
        let mut pixels = Vec::with_capacity(size * size);
        for _ in 0..(size * size) {
            pixels.push(Color::GRAY);
        }
        self.textures.insert(name.to_string(), pixels);
    }

    /// Get texture data by name
    pub fn get_texture(&self, name: &str) -> Option<&Vec<Color>> {
        self.textures.get(name)
    }

    /// Sample a texture at (u, v) coordinates [0, 1]
    #[inline]
    pub fn sample(&self, texture: &[Color], u: f32, v: f32) -> Color {
        let size = self.texture_size;
        let x = ((u * size as f32) as usize).min(size - 1);
        let y = ((v * size as f32) as usize).min(size - 1);
        texture[y * size + x]
    }

    /// Sample with nearest-neighbor filtering (faster)
    #[inline]
    pub fn sample_point(&self, texture: &[Color], tex_x: usize, tex_y: usize) -> Color {
        let size = self.texture_size;
        texture[tex_y.min(size - 1) * size + tex_x.min(size - 1)]
    }
}
