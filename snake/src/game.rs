use piston_window::types::Color;
use piston_window::{Context, G2d, Key};
use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_SPEED_PER_SECOND: f64 = 0.5;
const RESTART_TIME_SECOND: f64 = 1.0;

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time_second: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            snake: Snake::new(2, 2),
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            waiting_time_second: 0.0,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time_second += delta_time;

        if self.game_over {
            if self.waiting_time_second > RESTART_TIME_SECOND {
                self.restart();
            }

            return;
        }

        if !self.food_exists {
            self.add_food()
        }

        if self.waiting_time_second > MOVING_SPEED_PER_SECOND {
            self.update_snake(self.snake.head_direction())
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir.unwrap());
    }

    pub fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();

        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    pub fn check_if_snake_alive(&self, dir: Direction) -> bool {
        let (next_x, next_y): (i32, i32) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    pub fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut food_x = rng.gen_range(1, self.width - 1);
        let mut food_y = rng.gen_range(1, self.height - 1);
        while self.snake.overlap_tail(food_x, food_y) {
            food_x = rng.gen_range(1, self.width - 1);
            food_y = rng.gen_range(1, self.height - 1);
        }

        self.food_x = food_x;
        self.food_y = food_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Direction) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }

        self.waiting_time_second = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}
