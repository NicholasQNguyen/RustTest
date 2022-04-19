use raylib::prelude::*;
use rand::Rng;
use std::{thread, time::Duration};

// global constants
static SCREEN_WIDTH: i32 = 640;
static SCREEN_HEIGHT: i32 = 480;
static SLEEP_TIME:u64 = 2;

fn main() {

    // constants

    let mut vector = Vector2::new(32.0, 32.0);
    let mut rectangle = Rectangle::new(32.0, 32.0, 32.0, 32.0);

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", vector.x as i32, vector.y as i32, 20, Color::BLACK);
        thread::sleep(Duration:: from_secs(SLEEP_TIME));
        random_move(&mut vector);
    }
}

fn random_move(position: &mut Vector2) {
    let move_x = rand::thread_rng().gen_range(0..SCREEN_WIDTH) as f32;
    let move_y = rand::thread_rng().gen_range(0..SCREEN_HEIGHT) as f32;
    position.x = move_x;
    position.y = move_y;
}
