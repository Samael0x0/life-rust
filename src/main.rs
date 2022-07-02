#![cfg_attr(windows,windows_subsystem = "windows")]

use ggez::{GameResult, event};
use ggez::conf::{WindowSetup, WindowMode};
use crate::state::GameState;

mod board;
mod state;

fn main() -> GameResult {
    let window_setup = WindowSetup::default()
        .title("Life")
        .icon("/icon.png");

    let window_mode = WindowMode::default()
        .dimensions(1600.0, 900.0);

    let (ctx, event_loop) =
        ggez::ContextBuilder::new("game_of_life", "aventuracodes")
            .window_setup(window_setup)
            .window_mode(window_mode)
        .build()?;

    let state = GameState::new(&ctx);

    event::run(ctx, event_loop, state)
}