use raylib::prelude::*;
use crate::player::Player;

pub struct UI {
    pub font_size: i32,
}

impl UI {
    pub fn new(font_size: i32) -> Self {
        Self { font_size }
    }

    /// Render the main menu
    pub fn render_menu(&self, d: &mut RaylibDrawHandle, screen_width: i32, screen_height: i32) {
        // Background
        d.clear_background(Color::BLACK);

        // Title with Backrooms aesthetic
        let title = "ENTER THE BACKROOMS";
        let title_size = 40;
        let title_width = d.measure_text(title, title_size);

        // Draw glowing title effect
        for offset in -2..=2 {
            d.draw_text(
                title,
                screen_width / 2 - title_width / 2 + offset,
                screen_height / 4 + offset,
                title_size,
                Color::new(200, 180, 0, 50),
            );
        }

        d.draw_text(
            title,
            screen_width / 2 - title_width / 2,
            screen_height / 4,
            title_size,
            Color::new(255, 220, 0, 255),
        );

        // Menu options
        let options = vec![
            "PRESS ENTER TO START",
            "WASD - Move",
            "Mouse - Look Around",
            "ESC - Quit",
        ];

        let start_y = screen_height / 2;
        for (i, option) in options.iter().enumerate() {
            let text_width = d.measure_text(option, self.font_size);
            let y = start_y + (i as i32 * (self.font_size + 10));

            let color = if i == 0 {
                // Pulsing effect for "Press Enter"
                let pulse = ((d.get_time() * 3.0).sin() * 0.3 + 0.7) as f32;
                Color::new(
                    (255.0 * pulse) as u8,
                    (220.0 * pulse) as u8,
                    0,
                    255,
                )
            } else {
                Color::new(200, 200, 200, 255)
            };

            d.draw_text(
                option,
                screen_width / 2 - text_width / 2,
                y,
                self.font_size,
                color,
            );
        }

        // Atmospheric flavor text
        let warning = "Find the blue door to escape...";
        let warning_width = d.measure_text(warning, 20);
        d.draw_text(
            warning,
            screen_width / 2 - warning_width / 2,
            screen_height - 100,
            20,
            Color::new(150, 150, 150, 200),
        );
    }

    /// Render the HUD during gameplay
    pub fn render_hud(&self, d: &mut RaylibDrawHandle, player: &Player, fps: u32) {
        // Health bar in UPPER LEFT CORNER
        let health_bar_width = 200;
        let health_bar_height = 20;
        let health_percentage = player.health as f32 / player.max_health as f32;
        let margin = 10;
        let bar_x = margin; // Left side

        // "Health" label above the bar
        let health_label = "Health";
        let label_width = d.measure_text(health_label, 16);
        d.draw_text(
            health_label,
            bar_x + (health_bar_width - label_width) / 2,
            margin - 5,
            16,
            Color::WHITE,
        );

        // Health bar background
        d.draw_rectangle(bar_x, margin + 15, health_bar_width, health_bar_height, Color::new(50, 50, 50, 200));

        // Health bar fill
        let health_color = if health_percentage > 0.5 {
            Color::new(50, 200, 50, 255)
        } else if health_percentage > 0.25 {
            Color::new(200, 200, 50, 255)
        } else {
            Color::new(200, 50, 50, 255)
        };

        d.draw_rectangle(
            bar_x,
            margin + 15,
            (health_bar_width as f32 * health_percentage) as i32,
            health_bar_height,
            health_color,
        );

        // Health value text centered on bar
        let health_text = format!("{}/{}", player.health, player.max_health);
        let health_text_width = d.measure_text(&health_text, 16);
        d.draw_text(
            &health_text,
            bar_x + (health_bar_width - health_text_width) / 2,
            margin + 17,
            16,
            Color::WHITE,
        );

        // FPS counter in UPPER LEFT below health bar
        d.draw_text(
            &format!("FPS: {}", fps),
            margin,
            margin + 15 + health_bar_height + 10,
            20,
            Color::WHITE,
        );

        // Crosshair
        let center_x = d.get_screen_width() / 2;
        let center_y = d.get_screen_height() / 2;
        let crosshair_size = 10;

        d.draw_line(
            center_x - crosshair_size,
            center_y,
            center_x + crosshair_size,
            center_y,
            Color::WHITE,
        );
        d.draw_line(
            center_x,
            center_y - crosshair_size,
            center_x,
            center_y + crosshair_size,
            Color::WHITE,
        );
    }

    /// Render the victory screen
    pub fn render_victory(&self, d: &mut RaylibDrawHandle, screen_width: i32, screen_height: i32) {
        // Dark overlay
        d.draw_rectangle(0, 0, screen_width, screen_height, Color::new(0, 0, 0, 200));

        // Victory message
        let title = "YOU ESCAPED!";
        let title_size = 50;
        let title_width = d.measure_text(title, title_size);

        d.draw_text(
            title,
            screen_width / 2 - title_width / 2,
            screen_height / 3,
            title_size,
            Color::new(50, 200, 255, 255),
        );

        // Subtitle
        let subtitle = "You found your way out of the Backrooms...";
        let subtitle_width = d.measure_text(subtitle, 24);
        d.draw_text(
            subtitle,
            screen_width / 2 - subtitle_width / 2,
            screen_height / 2,
            24,
            Color::new(200, 200, 200, 255),
        );

        // Instructions
        let restart = "Press ENTER to return to menu";
        let restart_width = d.measure_text(restart, 20);
        d.draw_text(
            restart,
            screen_width / 2 - restart_width / 2,
            screen_height * 2 / 3,
            20,
            Color::new(150, 150, 150, 255),
        );
    }
}
