use ggez::{Context, GameResult, GameError};
use ggez::graphics::{self, present, draw, clear, Image, DrawParam, Color};
use ggez::event::{EventHandler, MouseButton, quit, KeyMods};
use ggez::input::{keyboard::KeyCode, mouse::button_pressed};
use ggez::timer::sleep;
use std::time::Duration;
use crate::board::{TILE_SIZE, GameBoard};

pub struct GameState {
    tile_image: Option<Image>,
    board: GameBoard,
    paused: bool,
    delay: f32
}

impl GameState {
    fn tile_dim(ctx: &Context) -> (usize, usize) {
        let (win_width, win_height) = graphics::drawable_size(ctx);
        (win_width as usize / TILE_SIZE, win_height as usize / TILE_SIZE)
    }

    fn win_to_tile_pos(x: f32, y: f32) -> (usize, usize) {
        (x as usize/ TILE_SIZE, y as usize / TILE_SIZE)
    }

    pub fn new(ctx: &Context) -> GameState {
        let (board_width, board_height) = Self::tile_dim(ctx);

        GameState {
            tile_image: None,
            board: GameBoard::new(board_width, board_height, true),
            paused: false,
            delay: 0.0
        }
    }
}

impl EventHandler<GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if !self.paused {
            self.board.update();
            sleep(Duration::from_secs_f32(self.delay));
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, Color::BLACK);

        if self.tile_image.is_none() {
            self.tile_image = {
                let pixels = {
                    let mut vec: Vec<u8> = Vec::new();
                    for _ in 0..(TILE_SIZE * TILE_SIZE * 4) {
                        vec.push(255);
                    }
                    vec
                };
                Some(Image::from_rgba8(ctx, TILE_SIZE as u16, TILE_SIZE as u16, pixels.as_slice())?)
            }
        }

        let color = if self.paused { Color::BLUE } else { Color::WHITE };
        let batch = self.board.batch(self.tile_image.as_ref().unwrap().clone(), color);

        draw(ctx, &batch, DrawParam::new())?;
        present(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let (tile_x, tile_y) = Self::win_to_tile_pos(x, y);

        if button == MouseButton::Left {
            self.board.set(tile_x, tile_y, true);
        } else if button == MouseButton::Right {
            self.board.set(tile_x, tile_y, false);
        } else if button == MouseButton::Middle {
            self.delay = 0.0;
        }
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        let (tile_x, tile_y) = Self::win_to_tile_pos(x, y);

        if button_pressed(ctx, MouseButton::Left) {
            self.board.set(tile_x, tile_y, true);
        } else if button_pressed(ctx, MouseButton::Left) {
            self.board.set(tile_x, tile_y, false);
        }
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) {
        self.delay += y * 0.01;

        if self.delay < 0.0 {
            self.delay = 0.0;
        } else if self.delay > 1.0 {
            self.delay = 1.0;
        }
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, repeat: bool) {
        if repeat { return; }

        if keycode == KeyCode::Escape {
            quit(ctx);
        } else if keycode == KeyCode::P {
            self.paused = !self.paused;
        } else if keycode == KeyCode::R {
            self.board.recreate(true);
        } else if keycode == KeyCode::C {
            self.board.recreate(false);
        }
    }
}