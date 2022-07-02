#![cfg_attr(windows,windows_subsystem = "windows")]

use ggez::{Context, GameResult, event, GameError};
use ggez::graphics::{present, draw, clear, Image, DrawParam, Color};
use ggez::conf::{WindowSetup, WindowMode};
use ggez::input::{keyboard::KeyCode, mouse::button_pressed};
use ggez::event::{EventHandler, MouseButton, quit, KeyMods};
use ggez::graphics::{self, spritebatch::SpriteBatch};
use ggez::mint::Point2;
use ggez::timer::sleep;
use std::time::Duration;
use std::path::Path;
use std::cmp::{max, min};

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
            .add_resource_path(Path::new("/resources"))
        .build()?;

    let state = GameState::new(&ctx);

    event::run(ctx, event_loop, state)
}

const TILE_SIZE : usize = 5;

struct GameBoard {
    board: Vec<Vec<bool>>
}

impl GameBoard {
    fn new(width: usize, height: usize, random: bool) -> Self {
        let mut vec = Vec::new();
        for _ in 0..height {
            let mut vec_row = Vec::new();
            for _ in 0..width {
                if random { vec_row.push(rand::random::<bool>()); }
                else { vec_row.push(false); }
            }
            vec.push(vec_row)
        }

        GameBoard {
            board: vec
        }
    }

    fn recreate(&mut self, random: bool) {
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                self.board[y][x] = {
                    if random { rand::random::<bool>() }
                    else { false }
                }
            }
        }
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        if y < self.board.len() && x < self.board[y].len() {
            self.board[y][x] = value;
        }
    }

    fn get_neighbor_count(&self, x: usize, y: usize) -> u32 {
        let mut alive = 0;

        let y_min = max(y, 1) - 1;
        let y_max = min(y + 1, self.board.len() - 1);
        let x_min = max(x, 1) - 1;
        let x_max = min(x + 1, self.board[0].len() - 1);

        for board_y in y_min..=y_max {
            for board_x in x_min..=x_max {
                if !(board_x == x && board_y == y) && self.board[board_y][board_x] {
                    alive += 1;
                }
            }
        }

        alive
    }

    fn update(&mut self) {
        if self.board.len() == 0 { return; }

        let mut new_board = Self::new(self.board[0].len(), self.board.len(), false);

        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                let alive = self.get_neighbor_count(x, y);

                if (self.board[y][x] && alive == 2) || alive == 3 {
                        new_board.set(x, y, true);
                }
            }
        }

        self.board = new_board.board;
    }

    fn batch(&self, image: Image, color: Color) -> SpriteBatch {
        let mut sprite_batch = SpriteBatch::new(image);

        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                if self.board[y][x] {
                    let point = Point2{ x: (x * TILE_SIZE) as f32, y: (y * TILE_SIZE) as f32 };
                    sprite_batch.add(DrawParam::new().dest(point)
                        .color(color));
                }
            }
        }

        sprite_batch
    }
}

struct GameState {
    tile_image: Option<Image>,
    board: GameBoard,
    paused: bool,
    delay: f32
}

impl GameState {
    fn tile_dim(ctx: &Context) -> (usize, usize) {
        let (win_width, win_height) = graphics::size(ctx);
        (win_width as usize / TILE_SIZE, win_height as usize / TILE_SIZE)
    }

    fn win_to_tile_pos(x: f32, y: f32) -> (usize, usize) {
        (x as usize/ TILE_SIZE, y as usize / TILE_SIZE)
    }

    fn new(ctx: &Context) -> GameState {
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
        if self.paused { return Ok(()) }
        self.board.update();
        sleep(Duration::from_secs_f32(self.delay));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, Color::BLACK);

        if self.tile_image.is_none() {
            self.tile_image = {
                let pixels = {
                    let mut vec : Vec<u8> = Vec::new();
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
        self.delay += y * 0.01 ;

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