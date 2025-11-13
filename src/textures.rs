use raylib::prelude::*;
use std::collections::HashMap;

/// Individual texture with its own dimensions
#[derive(Clone)]
pub struct Texture {
    pub pixels: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    pub fn new(pixels: Vec<Color>, width: usize, height: usize) -> Self {
        Self { pixels, width, height }
    }

    /// Sample texture at normalized UV coordinates [0, 1]
    #[inline]
    pub fn sample(&self, u: f32, v: f32) -> Color {
        let x = ((u * self.width as f32) as usize).min(self.width - 1);
        let y = ((v * self.height as f32) as usize).min(self.height - 1);
        self.pixels[y * self.width + x]
    }

    /// Sample with pixel coordinates (faster, no normalization)
    #[inline]
    pub fn sample_point(&self, tex_x: usize, tex_y: usize) -> Color {
        let x = tex_x.min(self.width - 1);
        let y = tex_y.min(self.height - 1);
        self.pixels[y * self.width + x]
    }
}

pub struct TextureManager {
    pub textures: HashMap<String, Texture>,
    pub texture_size: usize, // Keep for backwards compatibility
}

impl TextureManager {
    pub fn new(texture_size: usize) -> Self {
        let mut manager = Self {
            textures: HashMap::new(),
            texture_size,
        };

        // Try to load PNG textures from assets/textures/
        // If loading fails, fall back to procedural generation
        if !manager.load_png_textures() {
            println!("⚠ PNG loading failed or incomplete, using procedural textures");
            manager.generate_backrooms_textures();
        }

        manager
    }

    /// Load PNG textures from assets/textures/ directory
    fn load_png_textures(&mut self) -> bool {
        let texture_paths = vec![
            ("wall", "assets/textures/wall.png"),
            ("wall_exit", "assets/textures/wall_exit.png"),
            ("floor", "assets/textures/floor2.png"),
            ("ceiling", "assets/textures/ceiling.png"),
        ];

        let mut success_count = 0;

        for (name, path) in texture_paths {
            match Image::load_image(path) {
                Ok(image) => {
                    match self.convert_image_to_texture(&image, name) {
                        Ok(_) => {
                            println!("✓ Loaded texture: {} ({}x{})", name,
                                self.textures.get(name).unwrap().width,
                                self.textures.get(name).unwrap().height);
                            success_count += 1;
                        }
                        Err(e) => {
                            println!("⚠ Failed to convert {}: {}", path, e);
                        }
                    }
                }
                Err(e) => {
                    println!("⚠ Failed to load {}: {:?}", path, e);
                }
            }
        }

        // Return true if at least the essential textures loaded
        success_count >= 2
    }

    /// Convert raylib Image to our Texture format
    fn convert_image_to_texture(&mut self, image: &Image, name: &str) -> Result<(), String> {
        let width = image.width as usize;
        let height = image.height as usize;

        if width == 0 || height == 0 {
            return Err("Invalid texture dimensions".to_string());
        }

        // Get pixel data from image
        let mut pixels = Vec::with_capacity(width * height);

        unsafe {
            let data = image.data as *const Color;
            for i in 0..(width * height) {
                pixels.push(*data.offset(i as isize));
            }
        }

        let texture = Texture::new(pixels, width, height);
        self.textures.insert(name.to_string(), texture);

        Ok(())
    }

    /// Generate Backrooms-themed textures procedurally (fallback)
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
        self.textures.insert("wall".to_string(), Texture::new(yellow_wall, size, size));

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
        self.textures.insert("wall_exit".to_string(), Texture::new(blue_door, size, size));

        // Ceiling texture (off-white with panels)
        let mut ceiling = Vec::with_capacity(size * size);
        for y in 0..size {
            for x in 0..size {
                let is_seam = x % (size / 4) == 0 || y % (size / 4) == 0;
                let gray = if is_seam { 200 } else { 240 };

                ceiling.push(Color::new(gray, gray, gray - 10, 255));
            }
        }
        self.textures.insert("ceiling".to_string(), Texture::new(ceiling, size, size));

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
        self.textures.insert("floor".to_string(), Texture::new(floor, size, size));
    }

    /// Get texture data by name
    pub fn get_texture(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }
}
