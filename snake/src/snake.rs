use std::collections::LinkedList;

use piston_window::types::Color;
use piston_window::{Context, G2d};

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let mut body: LinkedList<Block> = LinkedList::new();

        for i in (0..3).rev() {
            body.push_back(Block { x: x + i, y })
        }

        Self {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();

        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Direction) {
        self.direction = dir;

        let (next_x, next_y): (i32, i32) = self.next_head(dir);
        let new_block = Block {
            x: next_x,
            y: next_y,
        };

        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Direction) -> (i32, i32) {
        let (last_x, last_y): (i32, i32) = self.head_position();

        match dir {
            Direction::Up => (last_x, last_y - 1),
            Direction::Down => (last_x, last_y + 1),
            Direction::Left => (last_x - 1, last_y),
            Direction::Right => (last_x + 1, last_y),
        }
    }

    pub fn restore_tail(&mut self) {
        let block = self.tail.clone().unwrap();
        self.body.push_back(block);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        for (i, block) in self.body.iter().enumerate() {
            if x == block.x && y == block.y {
                return true;
            }

            if i == self.body.len() - 1 {
                break;
            }
        }

        false
    }
}
