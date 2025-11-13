use raylib::prelude::*;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<Color>,
    // Cache for texture rendering
    image: Option<Image>,
    texture: Option<Texture2D>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![Color::BLACK; width * height],
            image: None,
            texture: None,
        }
    }

    /// Clear the framebuffer with a color
    pub fn clear(&mut self, color: Color) {
        self.buffer.fill(color);
    }

    /// Set a pixel at (x, y) with bounds checking
    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    /// Set a pixel without bounds checking (faster, use carefully)
    #[inline]
    pub unsafe fn set_pixel_unchecked(&mut self, x: usize, y: usize, color: Color) {
        *self.buffer.get_unchecked_mut(y * self.width + x) = color;
    }

    /// Draw a vertical line (optimized for raycasting)
    pub fn draw_vertical_line(&mut self, x: usize, y_start: usize, y_end: usize, color: Color) {
        if x >= self.width {
            return;
        }

        let y_start = y_start.min(self.height);
        let y_end = y_end.min(self.height);

        for y in y_start..y_end {
            self.buffer[y * self.width + x] = color;
        }
    }

    /// Draw a vertical line with texture sampling
    pub fn draw_textured_line(
        &mut self,
        x: usize,
        y_start: usize,
        y_end: usize,
        texture: &[Color],
        tex_width: usize,
        tex_height: usize,
        tex_x: usize,
        shade: f32,
    ) {
        if x >= self.width || y_start >= y_end {
            return;
        }

        let line_height = y_end - y_start;
        let y_start_clamped = y_start.min(self.height);
        let y_end_clamped = y_end.min(self.height);

        for y in y_start_clamped..y_end_clamped {
            // Calculate texture Y coordinate
            let tex_y = ((y - y_start) * tex_height / line_height).min(tex_height - 1);
            let tex_index = tex_y * tex_width + tex_x;

            if let Some(&mut ref mut pixel) = self.buffer.get_mut(y * self.width + x) {
                if let Some(&tex_color) = texture.get(tex_index) {
                    // Apply shading
                    *pixel = Color::new(
                        (tex_color.r as f32 * shade) as u8,
                        (tex_color.g as f32 * shade) as u8,
                        (tex_color.b as f32 * shade) as u8,
                        255,
                    );
                }
            }
        }
    }

    /// Render the framebuffer to the screen using raylib (optimized with draw_pixel)
    pub fn render(&self, d: &mut RaylibDrawHandle, scale: i32) {
        if scale == 1 {
            // Fast path for 1:1 rendering - use draw_pixel
            for y in 0..self.height {
                for x in 0..self.width {
                    let color = self.buffer[y * self.width + x];
                    d.draw_pixel(x as i32, y as i32, color);
                }
            }
        } else {
            // Scaled rendering - use draw_rectangle but with larger blocks
            for y in 0..self.height {
                for x in 0..self.width {
                    let color = self.buffer[y * self.width + x];
                    d.draw_rectangle(
                        (x as i32) * scale,
                        (y as i32) * scale,
                        scale,
                        scale,
                        color,
                    );
                }
            }
        }
    }

    /// Apply vignette post-processing effect to the entire framebuffer
    pub fn apply_vignette_effect(&mut self, intensity: f32, screen_width: usize, screen_height: usize) {
        if intensity <= 0.0 {
            return;
        }

        let center_x = screen_width as f32 / 2.0;
        let center_y = screen_height as f32 / 2.0;

        for y in 0..self.height {
            for x in 0..self.width {
                let dx = (x as f32 - center_x) / center_x;
                let dy = (y as f32 - center_y) / center_y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                let vignette_strength = (distance * intensity * 0.7).min(0.8);
                
                let index = y * self.width + x;
                let color = self.buffer[index];
                
                self.buffer[index] = Color::new(
                    (color.r as f32 * (1.0 - vignette_strength)) as u8,
                    (color.g as f32 * (1.0 - vignette_strength)) as u8,
                    (color.b as f32 * (1.0 - vignette_strength)) as u8,
                    255,
                );
            }
        }
    }

    /// Optimized render using Image (faster for larger screens)
    #[allow(dead_code)]
    pub fn to_image(&self) -> Image {
        let img = Image::gen_image_color(self.width as i32, self.height as i32, Color::BLACK);

        // Note: Direct pixel manipulation would require unsafe access
        // For now, this is a placeholder implementation
        img
    }

    /// Apply fog effect based on distance
    #[inline]
    pub fn apply_fog(color: Color, distance: f32, max_distance: f32, fog_color: Color) -> Color {
        let fog_factor = (distance / max_distance).min(1.0);

        Color::new(
            ((1.0 - fog_factor) * color.r as f32 + fog_factor * fog_color.r as f32) as u8,
            ((1.0 - fog_factor) * color.g as f32 + fog_factor * fog_color.g as f32) as u8,
            ((1.0 - fog_factor) * color.b as f32 + fog_factor * fog_color.b as f32) as u8,
            255,
        )
    }

    /// Draw textured floor span with perspective correction
    /// This renders a horizontal span of floor texture for a given screen row
    pub fn draw_textured_floor_span(
        &mut self,
        y: usize,
        x_start: usize,
        x_end: usize,
        texture: &[Color],
        tex_width: usize,
        tex_height: usize,
        player_x: f32,
        player_y: f32,
        player_angle: f32,
        distance: f32,
        fog_distance: f32,
    ) {
        if y >= self.height {
            return;
        }

        let x_start = x_start.min(self.width);
        let x_end = x_end.min(self.width);

        // Improved floor texture mapping with proper perspective
        // Calculate row distance from player
        let row_distance = (self.height as f32 / 2.0) / (y as f32 - self.height as f32 / 2.0).max(1.0);
        
        for x in x_start..x_end {
            // Calculate the angle for this column
            let camera_x = 2.0 * x as f32 / self.width as f32 - 1.0;
            let ray_angle = player_angle + camera_x * 0.5; // FOV factor
            
            // Calculate floor position
            let floor_x = player_x + ray_angle.cos() * row_distance * 0.5;
            let floor_y = player_y + ray_angle.sin() * row_distance * 0.5;
            
            // Sample texture with tiling
            let tex_u = (floor_x.abs() * 2.0) as usize % tex_width;
            let tex_v = (floor_y.abs() * 2.0) as usize % tex_height;
            
            let tex_index = (tex_v * tex_width + tex_u).min(texture.len() - 1);
            let base_color = texture[tex_index];
            
            // Minimal fog/shading for better visibility
            let shade_factor = 0.9; // Keep floor bright
            let shaded_color = Color::new(
                (base_color.r as f32 * shade_factor) as u8,
                (base_color.g as f32 * shade_factor) as u8,
                (base_color.b as f32 * shade_factor) as u8,
                255,
            );
            
            self.buffer[y * self.width + x] = shaded_color;
        }
    }

    /// Draw textured ceiling span (similar to floor but can be simpler)
    pub fn draw_textured_ceiling_span(
        &mut self,
        y: usize,
        x_start: usize,
        x_end: usize,
        texture: &[Color],
        tex_width: usize,
        tex_height: usize,
        player_x: f32,
        player_y: f32,
    ) {
        if y >= self.height {
            return;
        }

        let x_start = x_start.min(self.width);
        let x_end = x_end.min(self.width);

        // Simple ceiling texture mapping based on player position with tiling
        let tex_u = (player_x * 2.0) as usize % tex_width;
        let tex_v = (player_y * 2.0) as usize % tex_height;
        let tex_index = (tex_v * tex_width + tex_u).min(texture.len() - 1);
        let color = texture[tex_index];

        // Ceiling is slightly darker than walls (subtle ambient occlusion)
        let darkened = Color::new(
            (color.r as f32 * 0.85) as u8,
            (color.g as f32 * 0.85) as u8,
            (color.b as f32 * 0.85) as u8,
            255,
        );

        for x in x_start..x_end {
            self.buffer[y * self.width + x] = darkened;
        }
    }
}
