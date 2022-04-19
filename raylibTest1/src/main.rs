use raylib::prelude::*;
use rand::Rng;

// global constants
static SCREEN_WIDTH: i32 = 640;
static SCREEN_HEIGHT: i32 = 480;

fn main() {

    // constants

    let vector = Vector2::new(32.0, 32.0);

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", vector.x as i32, vector.y as i32, 20, Color::BLACK);
    }
}

fn random_move() {
    let move_x = rand::thread_rng().gen_range(0..SCREEN_WIDTH);
    let move_y = rand::thread_rng().gen_range(0..SCREEN_HEIGHT);
}
