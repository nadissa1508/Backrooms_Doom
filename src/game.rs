use crate::player::Player;
use crate::maze::Maze;
use crate::textures::TextureManager;
use crate::audio::AudioManager;
use crate::sprite::Sprite;
use crate::caster::RayCaster;
use crate::camera::Camera;
use crate::minimap::Minimap;
use crate::ui::UI;
use crate::effects::Effects;
use crate::framebuffer::Framebuffer;
use crate::pill::{Pill, PillType, FloatingText};
use raylib::prelude::*;
use rand::Rng;

pub struct GameState<'a> {
    pub player: Player,
    pub maze: Maze,
    pub textures: TextureManager,
    pub audio: AudioManager<'a>,
    pub sprites: Vec<Sprite>,
    pub raycaster: RayCaster,
    pub camera: Camera,
    pub minimap: Minimap,
    pub ui: UI,
    pub effects: Effects,
    pub framebuffer: Framebuffer,
    pub state: State,
    pub time_in_darkness: f32,
    pub game_timer: f32, // Timer in seconds (starts at 180.0 for 3 minutes)
    pub idle_timer: f32,  // Tracks time since last movement
    pub pills: Vec<Pill>,
    pub floating_texts: Vec<FloatingText>,
}

#[derive(PartialEq, Copy, Clone)]
pub enum State {
    Menu,
    Playing,
    Victory,
    GameOver,
}

impl<'a> GameState<'a> {
    pub fn new(screen_width: usize, screen_height: usize, audio: &'a RaylibAudio) -> Result<Self, String> {
        // Load maze
        let maze = Maze::load_from_file("maze.txt", 1.0)?;

        // Create player at start position
        let player = Player::new(maze.start_pos.0, maze.start_pos.1);

        // Initialize systems
        let textures = TextureManager::new(64); // Very small textures for maximum performance
        let audio_manager = AudioManager::new(audio);
        // Optimize: Use very few rays for maximum performance (80 rays for 640px = 8px per ray)
        let num_rays = 80;
        let raycaster = RayCaster::new(std::f32::consts::PI / 3.0, num_rays, 20.0);
        let camera = Camera::new(0.003);

        // Position minimap in top-right corner (very small for maximum performance)
        let minimap_size = 100;
        let margin = 10;
        let minimap_x = screen_width as i32 - minimap_size - margin;
        let minimap_y = margin;
        let minimap = Minimap::new(minimap_size, 8.0, (minimap_x, minimap_y));

        let ui = UI::new(24);
        let effects = Effects::new();
        let framebuffer = Framebuffer::new(screen_width, screen_height);

        // Create flickering light sprites for atmosphere
        let mut sprites = Vec::new();
        sprites.push(Sprite::new_flickering_light(5.0, 5.0));
        sprites.push(Sprite::new_flickering_light(10.0, 10.0));

        // Create pills from maze pill_positions
        let mut pills = Vec::new();
        let mut rng = rand::thread_rng();
        for (x, y) in &maze.pill_positions {
            // Randomly assign red or blue pill type
            let pill_type = if rng.gen_bool(0.5) {
                PillType::Red
            } else {
                PillType::Blue
            };
            pills.push(Pill::new(*x, *y, pill_type));
        }

        Ok(Self {
            player,
            maze,
            textures,
            audio: audio_manager,
            sprites,
            raycaster,
            camera,
            minimap,
            ui,
            effects,
            framebuffer,
            state: State::Menu,
            time_in_darkness: 0.0,
            game_timer: 180.0, // 3 minutes = 180 seconds
            idle_timer: 0.0,   // Starts at 0, no idle penalty yet
            pills,
            floating_texts: Vec::new(),
        })
    }

    pub fn update(&mut self, rl: &RaylibHandle, delta_time: f32) {
        match self.state {
            State::Menu => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    // Transition to playing (audio handled in main.rs)
                    self.state = State::Playing;
                    self.camera.reset();
                    // Reset timer when starting a new game
                    self.game_timer = 180.0;
                }
            }
            State::Playing => {
                // Update game timer - count down
                self.game_timer -= delta_time;
                
                // Check if time ran out
                if self.game_timer <= 0.0 {
                    self.game_timer = 0.0;
                    self.state = State::GameOver;
                    return; // Don't process player input if game over
                }

                // Track idle time and apply penalty
                let is_moving = rl.is_key_down(KeyboardKey::KEY_W)
                    || rl.is_key_down(KeyboardKey::KEY_S)
                    || rl.is_key_down(KeyboardKey::KEY_A)
                    || rl.is_key_down(KeyboardKey::KEY_D);

                if is_moving {
                    // Player is moving, reset idle timer
                    self.idle_timer = 0.0;
                } else {
                    // Player is idle, increment timer
                    self.idle_timer += delta_time;
                    
                    // Check if idle for more than 5 seconds
                    if self.idle_timer >= 5.0 {
                        // Apply idle penalty
                        self.player.take_damage(10);
                        
                        // Trigger anxiety effect
                        self.effects.trigger_anxiety_effect();
                        
                        // Reset idle timer to prevent continuous damage
                        self.idle_timer = 0.0;
                        
                        // Note: Heartbeat sound will be played in main.rs
                    }
                }

                // Update camera rotation
                self.camera.update(rl, &mut self.player, delta_time);

                // Handle player movement
                if rl.is_key_down(KeyboardKey::KEY_W) {
                    self.player.move_forward(&self.maze, delta_time);
                }
                if rl.is_key_down(KeyboardKey::KEY_S) {
                    self.player.move_backward(&self.maze, delta_time);
                }
                if rl.is_key_down(KeyboardKey::KEY_A) {
                    self.player.move_left(&self.maze, delta_time);
                }
                if rl.is_key_down(KeyboardKey::KEY_D) {
                    self.player.move_right(&self.maze, delta_time);
                }

                // Keyboard rotation
                if rl.is_key_down(KeyboardKey::KEY_LEFT) {
                    self.player.rotate(-self.player.rot_speed * delta_time);
                }
                if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                    self.player.rotate(self.player.rot_speed * delta_time);
                }

                // Update sprites
                for sprite in &mut self.sprites {
                    sprite.update(delta_time);
                }

                // Update pills (glow animation)
                for pill in &mut self.pills {
                    pill.update(delta_time);
                }

                // Check for pill collection
                for pill in &mut self.pills {
                    if !pill.collected && pill.can_collect(self.player.pos.x, self.player.pos.y, 0.6) {
                        pill.collected = true;
                        
                        // Apply pill effect
                        match pill.pill_type {
                            PillType::Red => {
                                // Red pill: -15 HP and trigger anxiety
                                self.player.take_damage(15);
                                self.effects.trigger_anxiety_effect();
                                
                                // Create floating text
                                self.floating_texts.push(FloatingText::new(
                                    "-15 HP".to_string(),
                                    pill.pos.x,
                                    pill.pos.y,
                                    Color::RED,
                                ));
                            }
                            PillType::Blue => {
                                // Blue pill: +10 HP
                                self.player.heal(10);
                                
                                // Create floating text
                                self.floating_texts.push(FloatingText::new(
                                    "+10 HP".to_string(),
                                    pill.pos.x,
                                    pill.pos.y,
                                    Color::SKYBLUE,
                                ));
                            }
                        }
                    }
                }

                // Update floating texts
                self.floating_texts.retain_mut(|text| {
                    text.update(delta_time);
                    text.lifetime > 0.0
                });

                // Update effects
                self.effects.update(delta_time);

                // Check if player reached goal
                if self.maze.is_goal(self.player.pos.x, self.player.pos.y, 1.0) {
                    self.state = State::Victory;
                }

                // Escape to menu
                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    self.state = State::Menu;
                }
            }
            State::Victory => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    // Reset game and return to menu
                    self.player = Player::new(self.maze.start_pos.0, self.maze.start_pos.1);
                    self.state = State::Menu;
                }
            }
            State::GameOver => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    // Reset game and return to menu
                    self.player = Player::new(self.maze.start_pos.0, self.maze.start_pos.1);
                    self.game_timer = 180.0;
                    self.state = State::Menu;
                }
            }
        }
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle) {
        match self.state {
            State::Menu => {
                self.ui.render_menu(d, d.get_screen_width(), d.get_screen_height());
            }
            State::Playing => {
                self.render_3d_view();
                
                // Apply anxiety vignette effect if active
                if self.effects.anxiety_intensity > 0.0 {
                    self.framebuffer.apply_vignette_effect(
                        self.effects.anxiety_intensity,
                        self.framebuffer.width,
                        self.framebuffer.height
                    );
                }
                
                self.framebuffer.render(d, 1);
                
                // Render screen shake overlay (subtle red tint during anxiety)
                if self.effects.anxiety_intensity > 0.0 {
                    let shake_alpha = (self.effects.anxiety_intensity * 30.0) as u8;
                    d.draw_rectangle(
                        0, 0,
                        d.get_screen_width(),
                        d.get_screen_height(),
                        Color::new(80, 0, 0, shake_alpha),
                    );
                }
                
                self.minimap.render(d, &self.maze, &self.player);
                self.ui.render_hud(d, &self.player, d.get_fps());
                // Render timer overlay
                self.ui.render_timer(d, self.game_timer);
                // Render floating texts
                self.render_floating_texts(d);
            }
            State::Victory => {
                self.render_3d_view();
                self.framebuffer.render(d, 1);
                self.ui.render_victory(d, d.get_screen_width(), d.get_screen_height());
            }
            State::GameOver => {
                self.render_3d_view();
                self.framebuffer.render(d, 1);
                self.ui.render_game_over(d, d.get_screen_width(), d.get_screen_height());
            }
        }
    }

    fn render_3d_view(&mut self) {
        // Get all textures we'll need
        let wall_texture = self.textures.get_texture("wall").unwrap();
        let wall_exit_texture = self.textures.get_texture("wall_exit").unwrap();
        let floor_texture = self.textures.get_texture("floor").unwrap();
        let ceiling_texture = self.textures.get_texture("ceiling").unwrap();

        // Cast rays
        let ray_hits = self.raycaster.cast_rays(&self.player, &self.maze);
        let num_rays = ray_hits.len();
        let screen_width = self.framebuffer.width;

        // Render each vertical slice with scaling
        for (ray_index, hit) in ray_hits.iter().enumerate() {
            let screen_height = self.framebuffer.height as f32;
            let wall_height = screen_height / hit.distance.max(0.1);

            let draw_start = ((screen_height / 2.0) - (wall_height / 2.0)) as usize;
            let draw_end = ((screen_height / 2.0) + (wall_height / 2.0)) as usize;

            // Calculate shading once per ray
            let orientation_shade = self.effects.calculate_shading(hit.hit_vertical);
            let distance_shade = self.effects.calculate_distance_shading(hit.distance, self.raycaster.max_depth);
            let total_shade = orientation_shade * distance_shade;

            // Select wall texture based on wall type
            let current_wall_texture = if hit.wall_type == 'E' {
                wall_exit_texture
            } else {
                wall_texture
            };

            // Sample texture with variable size support
            let tex_x = (hit.wall_x * current_wall_texture.width as f32) as usize;

            // Calculate screen x range for this ray (scale rays to screen width)
            let x_start = (ray_index * screen_width) / num_rays;
            let x_end = ((ray_index + 1) * screen_width) / num_rays;

            // Draw this ray across multiple screen columns
            for x in x_start..x_end {
                // Draw textured ceiling
                if draw_start > 0 {
                    self.framebuffer.draw_textured_ceiling_span(
                        0,
                        x,
                        x + 1,
                        &ceiling_texture.pixels,
                        ceiling_texture.width,
                        ceiling_texture.height,
                        self.player.pos.x,
                        self.player.pos.y,
                    );

                    // Fill remaining ceiling pixels with solid texture color
                    for y in 1..draw_start {
                        self.framebuffer.draw_textured_ceiling_span(
                            y,
                            x,
                            x + 1,
                            &ceiling_texture.pixels,
                            ceiling_texture.width,
                            ceiling_texture.height,
                            self.player.pos.x + y as f32 * 0.1,
                            self.player.pos.y + y as f32 * 0.1,
                        );
                    }
                }

                // Draw wall with texture (variable size support)
                self.framebuffer.draw_textured_line(
                    x,
                    draw_start,
                    draw_end,
                    &current_wall_texture.pixels,
                    current_wall_texture.width,
                    current_wall_texture.height,
                    tex_x,
                    total_shade,
                );

                // Draw textured floor
                if draw_end < self.framebuffer.height {
                    for y in draw_end..self.framebuffer.height {
                        self.framebuffer.draw_textured_floor_span(
                            y,
                            x,
                            x + 1,
                            &floor_texture.pixels,
                            floor_texture.width,
                            floor_texture.height,
                            self.player.pos.x,
                            self.player.pos.y,
                            self.player.angle,
                            hit.distance,
                            self.raycaster.max_depth,
                        );
                    }
                }
            }
        }
        
        // Render pills into the framebuffer
        self.render_pills_to_framebuffer(&ray_hits);
    }

    // Add these methods INSIDE the impl<'a> GameState<'a> { } block, BEFORE the final closing brace

    fn render_pills_to_framebuffer(&mut self, ray_hits: &[crate::caster::RayHit]) {
        let screen_width = self.framebuffer.width as f32;
        let screen_height = self.framebuffer.height as f32;

        // Get pill textures
        let red_pill_texture = self.textures.get_texture("red_pill");
        let blue_pill_texture = self.textures.get_texture("blue_pill");

        for pill in &self.pills {
            if pill.collected {
                continue;
            }

            // Calculate pill position relative to player
            let dx = pill.pos.x - self.player.pos.x;
            let dy = pill.pos.y - self.player.pos.y;
            let distance = (dx * dx + dy * dy).sqrt();

            // Rotate to player's view space (correct rotation for view transformation)
            let cos_angle = self.player.angle.cos();
            let sin_angle = self.player.angle.sin();
            let transformed_x = dy * cos_angle - dx * sin_angle;
            let transformed_y = dx * cos_angle + dy * sin_angle;

            // Skip if behind player
            if transformed_y <= 0.1 {
                continue;
            }

            // Project to screen space
            let fov = std::f32::consts::PI / 3.0;
            let screen_x = (screen_width / 2.0) * (1.0 + transformed_x / (transformed_y * (fov / 2.0).tan()));

            // Calculate which ray column this pill is in
            let ray_index = ((screen_x / screen_width) * ray_hits.len() as f32) as usize;

            // Improved occlusion check - check multiple rays around the pill
            let mut is_occluded = false;
            let check_radius = 2; // Check 2 rays on each side
            for offset in -(check_radius as i32)..=(check_radius as i32) {
                let check_index = (ray_index as i32 + offset).max(0).min(ray_hits.len() as i32 - 1) as usize;
                if check_index < ray_hits.len() {
                    // If there's a wall closer than the pill, it's occluded
                    if ray_hits[check_index].distance < distance - 0.3 {
                        is_occluded = true;
                        break;
                    }
                }
            }

            if is_occluded {
                continue;
            }

            let sprite_size = (screen_height / transformed_y) * 0.15; // Smaller size

            // Skip if off screen
            if screen_x < -sprite_size || screen_x > screen_width + sprite_size {
                continue;
            }

            // Position pill on the floor (lower on screen)
            let screen_y = screen_height * 0.65;
            
            // Select texture based on pill type
            let texture = match pill.pill_type {
                crate::pill::PillType::Red => red_pill_texture,
                crate::pill::PillType::Blue => blue_pill_texture,
            };
            
            if let Some(tex) = texture {
                // Draw textured sprite - use fixed aspect ratio based on texture
                let aspect_ratio = tex.width as f32 / tex.height as f32;
                let sprite_width = sprite_size * 2.0 * aspect_ratio;
                let sprite_height = sprite_size * 2.0;

                let sprite_left = screen_x - sprite_width / 2.0;
                let sprite_top = screen_y - sprite_height / 2.0;

                // Glitch effect parameters based on glow_timer
                let glitch_intensity = (pill.glow_timer * 3.0).sin() * 0.5 + 0.5; // 0.0 to 1.0
                let glitch_offset = ((pill.glow_timer * 7.0).sin() * glitch_intensity * 3.0) as i32;
                let rgb_separation = (glitch_intensity * 2.0) as i32;

                // Random scanline glitch every few seconds
                let scanline_glitch = ((pill.glow_timer * 0.5).sin() * 10.0) as i32;

                // Sample and draw the texture with proper aspect ratio into framebuffer
                let tex_step_x = tex.width as f32 / sprite_width;
                let tex_step_y = tex.height as f32 / sprite_height;

                for screen_pixel_y in 0..(sprite_height as usize) {
                    let y = (sprite_top as usize).saturating_add(screen_pixel_y);
                    if y >= self.framebuffer.height {
                        continue;
                    }

                    // Add horizontal glitch displacement per scanline
                    let row_glitch = if (y as i32 + scanline_glitch) % 7 == 0 {
                        glitch_offset
                    } else {
                        0
                    };

                    let tex_y = ((screen_pixel_y as f32 * tex_step_y) as usize).min(tex.height - 1);

                    for screen_pixel_x in 0..(sprite_width as usize) {
                        let x = (sprite_left as usize).saturating_add(screen_pixel_x);
                        if x >= self.framebuffer.width {
                            continue;
                        }

                        let tex_x = ((screen_pixel_x as f32 * tex_step_x) as usize).min(tex.width - 1);

                        // RGB channel separation glitch effect
                        let color_r = tex.sample_point(
                            (tex_x as i32 + rgb_separation).max(0).min(tex.width as i32 - 1) as usize,
                            tex_y
                        );
                        let color_g = tex.sample_point(tex_x, tex_y);
                        let color_b = tex.sample_point(
                            (tex_x as i32 - rgb_separation).max(0).min(tex.width as i32 - 1) as usize,
                            tex_y
                        );

                        // Combine RGB channels
                        let mut glitched_color = Color::new(
                            color_r.r,
                            color_g.g,
                            color_b.b,
                            color_g.a
                        );

                        // Skip transparent pixels
                        if glitched_color.a < 10 {
                            continue;
                        }

                        // Color distortion effect
                        if glitch_intensity > 0.7 {
                            glitched_color.r = glitched_color.r.saturating_add((glitch_intensity * 30.0) as u8);
                            glitched_color.g = glitched_color.g.saturating_sub((glitch_intensity * 20.0) as u8);
                        }

                        // Draw to framebuffer with horizontal glitch offset
                        let final_x = (x as i32 + row_glitch).max(0).min(self.framebuffer.width as i32 - 1) as usize;
                        self.framebuffer.set_pixel(final_x, y, glitched_color);
                    }
                }
            }
        }
    }

    fn render_floating_texts(&self, d: &mut RaylibDrawHandle) {
        let screen_width = d.get_screen_width() as f32;
        let screen_height = d.get_screen_height() as f32;
        
        for text in &self.floating_texts {
            // Calculate text position relative to player
            let dx = text.pos.x - self.player.pos.x;
            let dy = text.pos.y - self.player.pos.y;
            
            // Rotate to player's view space
            let cos_angle = self.player.angle.cos();
            let sin_angle = self.player.angle.sin();
            let transformed_x = dx * cos_angle + dy * sin_angle;
            let transformed_y = -dx * sin_angle + dy * cos_angle;
            
            // Skip if behind player
            if transformed_y <= 0.1 {
                continue;
            }
            
            // Project to screen space
            let fov = std::f32::consts::PI / 3.0;
            let screen_x = (screen_width / 2.0) * (1.0 + transformed_x / (transformed_y * (fov / 2.0).tan()));
            
            // Apply floating offset
            let screen_y = (screen_height / 2.0) - (text.z * 20.0);
            
            // Calculate alpha based on lifetime
            let alpha = ((text.lifetime / 1.0) * 255.0).min(255.0) as u8;
            let mut color = text.color;
            color.a = alpha;
            
            // Draw text
            d.draw_text(&text.text, screen_x as i32 - 30, screen_y as i32, 24, color);
        }
    }

}