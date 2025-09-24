use raylib::prelude::*;
use std::f32::consts::{PI, TAU};
use crate::maze::Maze;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,   // ángulo en radianes
    pub fov: f32, // campo de visión
}

impl Player {
    pub fn new(x: f32, y: f32, fov: f32) -> Self {
        Player {
            pos: Vector2::new(x, y),
            a: 0.0,
            fov,
        }
    }

    pub fn normalize_angle(&mut self) {
        if self.a < 0.0 {
            self.a += TAU;
        } else if self.a >= TAU {
            self.a -= TAU;
        }
    }
}

pub fn process_events(
    window: &RaylibHandle,
    player: &mut Player,
    delta_time: f32,
    maze: &Maze,
    block_size: usize,
) {
    const MOVE_SPEED: f32 = 150.0; // píxeles por segundo
    const ROTATION_SPEED: f32 = PI / 2.0; // rad/s

    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a -= ROTATION_SPEED * delta_time;
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a += ROTATION_SPEED * delta_time;
    }

    // Movimiento tentativa
    let mut next_x = player.pos.x;
    let mut next_y = player.pos.y;

    if window.is_key_down(KeyboardKey::KEY_UP) {
        next_x += MOVE_SPEED * delta_time * player.a.cos();
        next_y += MOVE_SPEED * delta_time * player.a.sin();
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) {
        next_x -= MOVE_SPEED * delta_time * player.a.cos();
        next_y -= MOVE_SPEED * delta_time * player.a.sin();
    }

    // --- Chequeo de colisión ---
    let cell_x = (next_x / block_size as f32).floor() as isize;
    let cell_y = (next_y / block_size as f32).floor() as isize;

    if cell_x >= 0 && cell_y >= 0 && (cell_y as usize) < maze.len() {
        if (cell_x as usize) < maze[cell_y as usize].len() {
            if maze[cell_y as usize][cell_x as usize] == ' ' {
                // Solo mover si es espacio vacío
                player.pos.x = next_x;
                player.pos.y = next_y;
            }
        }
    }

    player.normalize_angle();
}
