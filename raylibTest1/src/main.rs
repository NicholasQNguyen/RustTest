use raylib::prelude::*;
use raylib::ffi::KeyboardKey::*;
use rand::Rng;

// Import my rust files
mod debug_functions;

// global constants
const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;
// const SLEEP_TIME: u64 = 2;
const SPEED: f32 = 0.1;

fn main() {

    // constants
    let mut vector = Vector2::new(32.0, 32.0);
    let mut rectangle = Rectangle::new(32.0, 32.0, 32.0, 32.0);

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    debug_functions::print_type_of(&rl);

    while !rl.window_should_close() {
        {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::WHITE);
        // Draw Hello, world! and move it to a random spot
        d.draw_text("Hello, world!", vector.x as i32, vector.y as i32, 20, Color::BLACK);
        random_move(&mut vector);
        // Draw a rectangle and move it with arrows
        d.draw_rectangle_rec(rectangle, Color::BLUE);
        }
        update(&mut rl, &mut rectangle);
    }
}

fn random_move(position: &mut Vector2) {
    let move_x = rand::thread_rng().gen_range(0..SCREEN_WIDTH) as f32;
    let move_y = rand::thread_rng().gen_range(0..SCREEN_HEIGHT) as f32;
    position.x = move_x;
    position.y = move_y;
}

fn update(rl: &mut RaylibHandle, rect: &mut Rectangle)
{
    if rl.is_key_down(KEY_UP){
        rect.y = rect.y - SPEED;
    }
    if rl.is_key_down(KEY_DOWN){
        rect.y = rect.y + SPEED;
    }
    if rl.is_key_down(KEY_RIGHT){
        rect.x = rect.x + SPEED;
    }
    if rl.is_key_down(KEY_LEFT){
        rect.x = rect.x - SPEED;
    }
}
