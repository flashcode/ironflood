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
    pub playfield: Playfield,
    pub colors: u16,
    pub versus: bool,
    pub square_size: f32,
    pub played: u16,
    pub score: [u64; 2],
}

#[derive(Clone)]
pub struct Playfield {
    pub squares: Vec<Color32>,
    pub width: u16,
    pub height: u16,
}

impl Game {
    // Create a new game with the specified width/height, colors, game mode and square size
    pub fn new(width: u16, height: u16, colors: u16, versus: bool, square_size: u16) -> Self {
        let mut game = Game {
            playfield: Playfield::new(width, height, colors, versus),
            colors,
            versus,
            square_size: f32::from(square_size),
            played: 0,
            score: [0, 0],
        };
        if versus {
            game.compute_scores();
        }
        game
    }

    // Compute score of the color at (x, y)
    pub fn compute_score(&self, x: u16, y: u16) -> u64 {
        let mut playfield = self.playfield.clone();
        let color = playfield.squares[(y * playfield.width + x) as usize];
        playfield.flood(x, y, color);
        playfield
            .squares
            .iter()
            .filter(|&c| *c == COLOR_TEMP)
            .count() as u64
    }

    // Compute score of both players
    pub fn compute_scores(&mut self) {
        self.score[0] = self.compute_score(0, 0);
        self.score[1] = self.compute_score(self.playfield.width - 1, self.playfield.height - 1);
    }

    // Flood with the best color found at (x, y)
    pub fn flood_best_color(&mut self, x: u16, y: u16) {
        let mut best_color: Color32 = COLOR_TEMP;
        let mut max_squares: u64 = 0;
        let color_xy = self.playfield.squares[(y * self.playfield.width + x) as usize];
        let color_top_left = self.playfield.squares[0];
        let color_bottom_right =
            self.playfield.squares[(self.playfield.width * self.playfield.height - 1) as usize];
        for color in &COLORS[..self.colors as usize] {
            if *color == color_xy
                || (self.versus && (*color == color_top_left || *color == color_bottom_right))
            {
                continue;
            }
            let mut playfield = self.playfield.clone();
            playfield.flood(x, y, color_xy);
            playfield.flood_end(*color);
            playfield.flood(x, y, *color);
            let count = playfield
                .squares
                .iter()
                .filter(|&c| *c == COLOR_TEMP)
                .count() as u64;
            if count > max_squares {
                best_color = *color;
                max_squares = count;
            }
        }
        self.playfield.flood(x, y, color_xy);
        self.playfield.flood_end(best_color);
    }
}

impl Playfield {
    // Create a new playfield with the specified width/height, colors and game mode
    pub fn new(width: u16, height: u16, colors: u16, versus: bool) -> Self {
        let squares = Playfield::get_random_squares(width, height, colors, versus);
        Playfield {
            squares,
            width,
            height,
        }
    }

    // Create squares with random colors
    fn get_random_squares(width: u16, height: u16, colors: u16, versus: bool) -> Vec<Color32> {
        let mut squares = Vec::new();
        let num_squares = width * height;
        for _ in 0..num_squares {
            squares.push(*COLORS[..colors as usize].choose(&mut rand::rng()).unwrap());
        }
        if versus {
            squares[0] = squares[(width * height - 1) as usize];
        }
        squares
    }

    // Flood at (x, y) with a color
    pub fn flood(&mut self, x: u16, y: u16, color: Color32) {
        self.squares[(y * self.width + x) as usize] = COLOR_TEMP;
        if y > 0 && self.squares[((y - 1) * self.width + x) as usize] == color {
            self.flood(x, y - 1, color);
        }
        if y < self.height - 1 && self.squares[((y + 1) * self.width + x) as usize] == color {
            self.flood(x, y + 1, color);
        }
        if x > 0 && self.squares[(y * self.width + x - 1) as usize] == color {
            self.flood(x - 1, y, color);
        }
        if x < self.width - 1 && self.squares[(y * self.width + x + 1) as usize] == color {
            self.flood(x + 1, y, color);
        }
    }

    // End of flood: remplace temporary color by the target color
    pub fn flood_end(&mut self, color: Color32) {
        for x in 0..self.width {
            for y in 0..self.height {
                let index = (y * self.width + x) as usize;
                if self.squares[index] == COLOR_TEMP {
                    self.squares[index] = color;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Game::new(20, 10, 6, true, 20);
        assert_eq!(
            game.playfield.squares[0],
            game.playfield.squares[20 * 10 - 1]
        );
        assert_eq!(game.playfield.width, 20);
        assert_eq!(game.playfield.height, 10);
        assert_eq!(game.colors, 6);
        assert!(game.versus);
        assert!((game.square_size - 20.0).abs() < 0.001_f32);
        assert_eq!(game.played, 0);
        assert_ne!(game.score, [0, 0]);
    }

    #[test]
    fn test_compute_scores() {
        let mut game = Game {
            playfield: Playfield {
                // test squares: 3x3, ██ = blue, ░░ = red
                //   ██ ██ ░░
                //   ██ ░░ ░░
                //   ░░ ░░ ░░
                squares: vec![
                    Color32::BLUE, // line 1
                    Color32::BLUE,
                    Color32::RED,
                    Color32::BLUE, // line 2
                    Color32::RED,
                    Color32::RED,
                    Color32::RED, // line 3
                    Color32::RED,
                    Color32::RED,
                ],
                width: 3,
                height: 3,
            },
            colors: 2,
            versus: true,
            square_size: 20.0,
            played: 0,
            score: [0; 2],
        };
        game.compute_scores();
        assert_eq!(game.score, [3, 6]);
    }

    #[test]
    fn test_flood_best_color() {
        let mut game = Game::new(4, 4, 6, true, 20);
        game.playfield = Playfield {
            // test squares: 4x4, ██ = blue, ░░ = red, ▚▚ = cyan, ▄▄ = green
            //   ██ ██ ░░ ▚▚
            //   ██ ██ ▚▚ ▚▚
            //   ██ ▚▚ ░░ ░░
            //   ▄▄ ▄▄ ░░ ░░
            squares: vec![
                Color32::BLUE, // line 1
                Color32::BLUE,
                Color32::RED,
                Color32::CYAN,
                Color32::BLUE, // line 2
                Color32::BLUE,
                Color32::CYAN,
                Color32::CYAN,
                Color32::BLUE, // line 3
                Color32::CYAN,
                Color32::RED,
                Color32::RED,
                Color32::GREEN, // line 4
                Color32::GREEN,
                Color32::RED,
                Color32::RED,
            ],
            width: 4,
            height: 4,
        };
        game.flood_best_color(3, 3);
        assert_eq!(game.playfield.squares[4 * 4 - 1], Color32::CYAN);
    }
}
