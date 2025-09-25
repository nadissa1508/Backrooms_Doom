// textures.rs - Fixed version
use raylib::prelude::*;
use std::collections::HashMap;

pub struct TextureData {
    pub pixels: Vec<Color>, // Store as Vec<Color> for safer access
    pub width: u32,
    pub height: u32,
}

pub struct TextureManager {
    texture_data: HashMap<char, TextureData>,
    textures: HashMap<char, Texture2D>, // Keep for potential GPU rendering
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let texture_files = vec![
            ('.', "assets/wall_corner.png"),
            ('+', "assets/wall_fredy.png"),
            ('|', "assets/wall_chica.png"),
            ('-', "assets/wall_foxy.png"),
            ('g', "assets/wall_draws.png"),
            ('b', "assets/bonnie.png"),
            ('c', "assets/chica.png"),
            ('f', "assets/freddy.png"),
        ];

        let mut texture_data = HashMap::new();
        let mut textures = HashMap::new();

        for (ch, path) in texture_files {
            // Try to load image, use fallback if it fails
            let image = Image::load_image(path)
                .unwrap_or_else(|e| {
                    println!("Warning: Failed to load {}: {:?}", path, e);
                    println!("Using fallback color for character '{}'", ch);
                    // Create a larger fallback texture to match your 512x512 images
                    let fallback_color = match ch {
                        '.' => Color::GRAY,
                        '+' => Color::BROWN,
                        '-' => Color::YELLOW,
                        '|' => Color::RED,
                        'g' => Color::GREEN,
                        'b' => Color::BLUE,
                        'c' => Color::YELLOW,
                        'f' => Color::BROWN,
                        _ => Color::MAGENTA,
                    };
                    Image::gen_image_color(512, 512, fallback_color) // Changed to 512x512
                });

            // Convert to our safe pixel data format
            let width = image.width as u32;
            let height = image.height as u32;
            let pixels: Vec<Color> = image.get_image_data().to_vec(); // This returns Vec<Color>
            
            println!("Loaded texture '{}': {}x{} pixels", ch, width, height);
            
            texture_data.insert(ch, TextureData { pixels, width, height });

            // Load texture for potential GPU use
            if let Ok(texture) = rl.load_texture_from_image(thread, &image) {
                textures.insert(ch, texture);
            } else {
                println!("Warning: Failed to create GPU texture for {}", path);
            }
        }

        TextureManager { texture_data, textures }
    }

    #[inline(always)]
    pub fn get_pixel_color(&self, ch: char, tx: u32, ty: u32) -> Color {
        if let Some(data) = self.texture_data.get(&ch) {
            // Ensure coordinates are within bounds
            let x = tx.min(data.width - 1);
            let y = ty.min(data.height - 1);
            let idx = (y * data.width + x) as usize;
            
            // Safe array access
            if idx < data.pixels.len() {
                data.pixels[idx]
            } else {
                Color::MAGENTA // Debug color to indicate out-of-bounds access
            }
        } else {
            // Debug: show which characters are missing textures
            println!("Warning: No texture found for character '{}'", ch);
            Color::WHITE
        }
    }

    pub fn get_texture(&self, ch: char) -> Option<&Texture2D> {
        self.textures.get(&ch)
    }
}