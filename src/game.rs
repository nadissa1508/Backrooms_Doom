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
use raylib::prelude::*;

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
}

#[derive(PartialEq, Copy, Clone)]
pub enum State {
    Menu,
    Playing,
    Victory,
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
        })
    }

    pub fn update(&mut self, rl: &RaylibHandle, delta_time: f32) {
        match self.state {
            State::Menu => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    // Transition to playing (audio handled in main.rs)
                    self.state = State::Playing;
                    self.camera.reset();
                }
            }
            State::Playing => {
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
        }
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle) {
        match self.state {
            State::Menu => {
                self.ui.render_menu(d, d.get_screen_width(), d.get_screen_height());
            }
            State::Playing => {
                self.render_3d_view();
                self.framebuffer.render(d, 1);
                self.minimap.render(d, &self.maze, &self.player);
                self.ui.render_hud(d, &self.player, d.get_fps());
            }
            State::Victory => {
                self.render_3d_view();
                self.framebuffer.render(d, 1);
                self.ui.render_victory(d, d.get_screen_width(), d.get_screen_height());
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
    }
}