extern crate piston_window;
extern crate rand;

use piston_window::{PistonWindow, WindowSettings};

use crate::draw::to_coord_u32;

mod draw;
mod game;
mod snake;

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();
}
