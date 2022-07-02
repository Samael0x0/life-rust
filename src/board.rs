use ggez::graphics::{spritebatch::SpriteBatch, Color, Image, DrawParam};
use ggez::mint::Point2;
use bitvec::prelude::*;
use std::cmp::{max, min};

pub const TILE_SIZE : usize = 5;

pub struct GameBoard {
    board: Vec<BitVec>
}

impl GameBoard {
    pub fn new(width: usize, height: usize, random: bool) -> Self {
        let mut vec = Vec::new();
        for _ in 0..height {
            let mut vec_row = BitVec::new();
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

    pub fn recreate(&mut self, random: bool) {
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                self.board[y].set(x, {
                    if random { rand::random::<bool>() }
                    else { false }
                });
            }
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        if y < self.board.len() && x < self.board[y].len() {
            self.board[y].set(x, value);
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

    pub fn update(&mut self) {
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

    pub fn batch(&self, image: Image, color: Color) -> SpriteBatch {
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