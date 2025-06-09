// SPDX-FileCopyrightText: 2025 Sébastien Helleu <flashcode@flashtux.org>
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod app;
mod game;

use clap::Parser;

use crate::app::run_app;

/// IronFlood game
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Width of the playfield
    #[arg(short = 'x', long, default_value_t = 14, value_parser = clap::value_parser!(u16).range(5..))]
    width: u16,

    /// Height of the playfield
    #[arg(short = 'y', long, default_value_t = 14, value_parser = clap::value_parser!(u16).range(5..))]
    height: u16,

    /// Number of colors
    #[arg(short, long, default_value_t = 6, value_parser = clap::value_parser!(u16).range(2..=12))]
    colors: u16,

    /// Versus mode: play against computer
    #[arg(short, long, default_value_t = false)]
    versus: bool,

    /// Square size in pixels
    #[arg(short, long, default_value_t = 60, value_parser = clap::value_parser!(u16).range(3..))]
    square_size: u16,
}

fn main() -> eframe::Result {
    let args = Args::parse();
    run_app(
        args.width,
        args.height,
        args.colors,
        args.versus,
        args.square_size,
    )
}
