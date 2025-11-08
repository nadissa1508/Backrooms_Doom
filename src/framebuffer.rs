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
}
