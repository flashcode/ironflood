// SPDX-FileCopyrightText: 2025 Sébastien Helleu <flashcode@flashtux.org>
//
// SPDX-License-Identifier: GPL-3.0-or-later


use eframe::egui;
use egui::{Color32, Pos2, Rect, ViewportCommand};

use crate::game::Game;

pub fn run_app(width: u16, height: u16, colors: u16, square_size: u16) -> eframe::Result {
    let viewport_size: [f32; 2] = [
        width as f32 * square_size as f32,
        height as f32 * square_size as f32,
    ];
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(viewport_size),
        ..Default::default()
    };
    let app = IronfloodApp {
        game: Game::new(width, height, colors, square_size),
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
            ctx.send_viewport_cmd(ViewportCommand::Title(format!(
                "IronFlood ({})",
                self.game.played
            )));
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
                let color_top_left = self.game.get_square(0, 0);
                let color = self.game.get_square(x, y);
                if color != color_top_left {
                    self.game.flood(0, 0, color_top_left);
                    self.game.flood_end(color);
                    self.game.played += 1;
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
        for y in 0..self.game.height {
            for x in 0..self.game.width {
                self.draw_square(ui, x, y, self.game.get_square(x, y))
            }
        }
    }
}
