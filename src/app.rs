// SPDX-FileCopyrightText: 2025 Sébastien Helleu <flashcode@flashtux.org>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use eframe::egui;
use egui::{Color32, Pos2, Rect, ViewportCommand};

use crate::game::Game;

pub fn run_app(
    width: u16,
    height: u16,
    colors: u16,
    versus: bool,
    square_size: u16,
) -> eframe::Result {
    let viewport_size: [f32; 2] = [
        width as f32 * square_size as f32,
        height as f32 * square_size as f32,
    ];
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(viewport_size),
        ..Default::default()
    };
    let app = IronfloodApp {
        game: Game::new(width, height, colors, versus, square_size),
    };
    eframe::run_native("IronFlood", options, Box::new(|_cc| Ok(Box::new(app))))
}

struct IronfloodApp {
    pub game: Game,
}

impl eframe::App for IronfloodApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.mouse_events(ctx);
            self.draw_playfield(ui);
            self.set_title(ctx);
        });
    }
}

impl IronfloodApp {
    // Handle mouse events
    fn mouse_events(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.pointer.primary_released()) {
            if let Some(pos) = ctx.input(|i| i.pointer.latest_pos()) {
                let x = (pos.x / self.game.square_size) as u16;
                let y = (pos.y / self.game.square_size) as u16;
                let color_top_left = self.game.playfield.squares[0];
                let color_bottom_right = self.game.playfield.squares
                    [(self.game.playfield.width * self.game.playfield.height - 1) as usize];
                let color =
                    self.game.playfield.squares[(y * self.game.playfield.width + x) as usize];
                if color != color_top_left && (!self.game.versus || color != color_bottom_right) {
                    self.game.playfield.flood(0, 0, color_top_left);
                    self.game.playfield.flood_end(color);
                    self.game.played += 1;
                    if self.game.versus {
                        self.game.flood_best_color(
                            self.game.playfield.width - 1,
                            self.game.playfield.height - 1,
                        );
                        self.game.compute_scores();
                    }
                }
            }
        }
    }

    // Draw a square at the given position (x, y) with the specified color
    fn draw_square(&self, ui: &mut egui::Ui, x: u16, y: u16, color: Color32) {
        let rect = Rect {
            min: Pos2 {
                x: x as f32 * self.game.square_size,
                y: y as f32 * self.game.square_size,
            },
            max: Pos2 {
                x: (x as f32 * self.game.square_size) + self.game.square_size - 1.0,
                y: (y as f32 * self.game.square_size) + self.game.square_size - 1.0,
            },
        };
        ui.painter().rect_filled(rect, 0., color);
    }

    // Draw the playfield
    fn draw_playfield(&self, ui: &mut egui::Ui) {
        for y in 0..self.game.playfield.height {
            for x in 0..self.game.playfield.width {
                let color =
                    self.game.playfield.squares[(y * self.game.playfield.width + x) as usize];
                self.draw_square(ui, x, y, color);
            }
        }
    }

    // Set the window title with game info
    fn set_title(&self, ctx: &egui::Context) {
        if self.game.versus {
            ctx.send_viewport_cmd(ViewportCommand::Title(format!(
                "IronFlood - Versus mode: {} - {}",
                self.game.score[0], self.game.score[1],
            )));
        } else {
            ctx.send_viewport_cmd(ViewportCommand::Title(format!(
                "IronFlood - Single player: {}",
                self.game.played,
            )));
        }
    }
}
