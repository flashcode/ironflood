// SPDX-FileCopyrightText: 2025 Sébastien Helleu <flashcode@flashtux.org>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use egui::Color32;
use rand::seq::IndexedRandom;

pub const COLORS: [Color32; 12] = [
    Color32::BLUE,
    Color32::RED,
    Color32::GREEN,
    Color32::YELLOW,
    Color32::MAGENTA,
    Color32::CYAN,
    Color32::BLACK,
    Color32::WHITE,
    Color32::ORANGE,
    Color32::DARK_GRAY,
    Color32::DARK_GREEN,
    Color32::PURPLE,
];

const COLOR_TEMP: Color32 = Color32::from_rgb(1, 1, 1);

pub struct Game {
    pub playfield: Vec<Color32>,
    pub width: u16,
    pub height: u16,
    pub square_size: f32,
    pub played: u16,
}

impl Game {
    // Create a new game with the specified width and height
    pub fn new(width: u16, height: u16, colors: u16, square_size: u16) -> Self {
        Game {
            playfield: Game::get_random_playfield(width, height, colors),
            width,
            height,
            square_size: square_size as f32,
            played: 0,
        }
    }

    // Create a random playfield
    fn get_random_playfield(width: u16, height: u16, colors: u16) -> Vec<Color32> {
        let mut playfield = Vec::new();
        let squares = width * height;
        for _ in 0..squares {
            playfield.push(*COLORS[..colors as usize].choose(&mut rand::rng()).unwrap());
        }
        playfield
    }

    // Get the color of the square at (x, y) in the playfield
    pub fn get_square(&self, x: u16, y: u16) -> Color32 {
        self.playfield[(y * self.width + x) as usize]
    }

    // Set the color of the square at (x, y) in the playfield
    pub fn set_square(&mut self, x: u16, y: u16, color: Color32) {
        self.playfield[(y * self.width + x) as usize] = color;
    }

    // Flood at (x, y) with a color
    pub fn flood(&mut self, x: u16, y: u16, color: Color32) {
        self.set_square(x, y, COLOR_TEMP);
        if y > 0 && self.get_square(x, y - 1) == color {
            self.flood(x, y - 1, color)
        }
        if y < self.height - 1 && self.get_square(x, y + 1) == color {
            self.flood(x, y + 1, color)
        }
        if x > 0 && self.get_square(x - 1, y) == color {
            self.flood(x - 1, y, color)
        }
        if x < self.width - 1 && self.get_square(x + 1, y) == color {
            self.flood(x + 1, y, color)
        }
    }

    // End of flood: remplace temporary color by the target color
    pub fn flood_end(&mut self, color: Color32) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get_square(x, y) == COLOR_TEMP {
                    self.set_square(x, y, color)
                }
            }
        }
    }
}
