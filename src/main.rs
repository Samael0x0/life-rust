#![cfg_attr(windows,windows_subsystem = "windows")]

use ggez::{Context, GameResult, event, GameError};
use ggez::graphics::{present, draw, clear, Image, DrawParam, Color};
use ggez::conf::{WindowSetup, WindowMode};
use ggez::input::keyboard::KeyCode;
use ggez::event::{EventHandler, MouseButton, quit, KeyMods};
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::mint::Point2;
use ggez::timer::sleep;
use std::time::Duration;
use std::path::Path;

fn main() -> GameResult {
    let config = GameConfig::new();

    let window_setup = WindowSetup::default()
        .title("Life")
        .icon("/icon.png");

    let window_mode = WindowMode::default()
        .dimensions((config.game_width * config.tile_size) as f32 , (config.game_height * config.tile_size) as f32 );
        // .resizable(true);

    let (ctx, event_loop) =
        ggez::ContextBuilder::new("game_of_life", "samael")
            .window_setup(window_setup)
            .window_mode(window_mode)
            .add_resource_path(Path::new("/resources"))
        .build()?;
    
    event::run(ctx, event_loop,GameState::new(config))
}

struct GameConfig {
    game_width: i32,
    game_height: i32,
    tile_size: i32,
}

impl GameConfig {
    fn new() -> GameConfig {
        GameConfig {
            game_width: 190,
            game_height: 100,
            tile_size: 10
        }
    }
}

struct GameState {
    config: GameConfig,
    rect_pixels: Vec<u8>,
    game_board: Vec<Vec<bool>>,
    paused: bool,
    mouse_left: bool,
    mouse_right: bool,
    slowness: f32
}

impl GameState {
    fn new(config: GameConfig) -> GameState {
        let rect_pixels = {
            let mut vec : Vec<u8> = Vec::new();
            for _ in 0..(config.tile_size * config.tile_size * 4) {
                vec.push(255);
            }
            vec
        };

        let game_board = new_board(true, &config);

        GameState {
            config,
            rect_pixels,
            game_board,
            paused: false,
            mouse_left: false,
            mouse_right: false,
            slowness: 0.0
        }
    }
}

fn new_board(random: bool ,config: &GameConfig) -> Vec<Vec<bool>> {
    let mut vec = Vec::new();

    for _ in 0..config.game_height {
        let mut vec_row = Vec::new();
        for _ in 0..config.game_width {
            if random {
                vec_row.push(rand::random::<bool>());
            } else {
                vec_row.push(false);
            }

        }
        vec.push(vec_row)
    }
     vec
}

fn get_surroundings(board: &Vec<Vec<bool>>, config: &GameConfig, x: i32, y: i32) -> i32{
    let mut alive = 0;

    for board_y in y-1..=y+1 {
        for board_x in x-1..=x+1 {
            if !(board_x < 0 || board_x >= config.game_width || board_y < 0 || board_y >= config.game_height) && !(board_x == x && board_y == y) {
                if board[board_y as usize][board_x as usize] {
                    alive +=1;
                }
            }
        }
    }

    alive
}

impl EventHandler<GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.paused {
            return Ok(())
        }


        let mut new_game_board = new_board(false, &self.config);
        for y in 0..self.config.game_height {
            for x in 0..self.config.game_width {
                let alive = get_surroundings(&self.game_board, &self.config, x, y);

                let y = y as usize;
                let x = x as usize;

                if self.game_board[y][x] {
                    if alive < 2 || alive > 3 {
                        new_game_board[y][x] = false;
                    } else {
                        new_game_board[y][x] = true;
                    }
                } else {
                    if alive == 3 {
                        new_game_board[y][x] = true;
                    } else {
                        new_game_board[y][x] = false;
                    }
                }
            }
        }

        self.game_board = new_game_board;

        sleep(Duration::from_secs_f32(self.slowness));

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, Color::BLACK);

        let mut rectangles = SpriteBatch::new(Image::from_rgba8(ctx, self.config.tile_size as u16, self.config.tile_size as u16, self.rect_pixels.as_slice())?);

        let color = if self.paused {
            Color::new(0., 0., 1., 1.)
        } else {
            Color::WHITE
        };

        for y in 0..self.config.game_height {
            for x in 0..self.config.game_width {
                if self.game_board[y as usize][x as usize] {
                    rectangles.add(DrawParam::new().dest(Point2{x: x as f32 * self.config.tile_size as f32, y: y as f32 * self.config.tile_size as f32 }).color(color));
                }
            }
        }

        draw(ctx, &rectangles, DrawParam::new())?;

        present(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let tile_x = (x / self.config.tile_size as f32) as usize;
        let tile_y = (y / self.config.tile_size as f32) as usize;

        if tile_x >= self.config.game_width as usize || tile_y >= self.config.game_height as usize{
            return;
        }

        if button == MouseButton::Left {
            self.game_board[tile_y][tile_x] = true;
            self.mouse_left = true;
        } else if button == MouseButton::Right {
            self.game_board[tile_y][tile_x] = false;
            self.mouse_right = true;
        } else if button == MouseButton::Middle {
            self.slowness = 0.0;
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        if button == MouseButton::Left {
            self.mouse_left = false;
        } else if button == MouseButton::Right {
            self.mouse_right = false;
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        if self.mouse_left || self.mouse_right {
            let tile_x = (x / self.config.tile_size as f32) as usize;
            let tile_y = (y / self.config.tile_size as f32) as usize;

            if tile_x >= self.config.game_width as usize || tile_y >= self.config.game_height as usize{
                return;
            }

            if self.mouse_left {
                self.game_board[tile_y][tile_x] = true;
            } else if self.mouse_right {
                self.game_board[tile_y][tile_x] = false;
            }
        }
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) {
        self.slowness += y * 0.01 ;
        if self.slowness < 0.0 {
            self.slowness = 0.0;
        }
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, repeat: bool) {
        if keycode == KeyCode::Escape {
            quit(ctx);
        }
        if !repeat {
            if keycode == KeyCode::P {
                self.paused = !self.paused;
            } else if keycode == KeyCode::R {
                self.game_board = new_board(true, &self.config);
            } else if keycode == KeyCode::C {
                self.game_board = new_board(false, &self.config)
            }
        }
    }
}