use raylib::prelude::*;
use raylib::ffi::KeyboardKey::*;
use raylib::ffi::MouseButton::*;
use rand::Rng;

// Import my rust files
// mod debug_functions;

// global constants
const SCREEN_WIDTH: i32 = 720;
const SCREEN_HEIGHT: i32 = 480;
// const SLEEP_TIME: u64 = 2;
const SPEED: f32 = 0.05;


fn update_player(rl: &mut RaylibHandle, rect: &mut Rectangle){
    if rl.is_key_down(KEY_W){
        rect.y = rect.y - SPEED;
    }
    if rl.is_key_down(KEY_S){
        rect.y = rect.y + SPEED;
    }
    if rl.is_key_down(KEY_D){
        rect.x = rect.x + SPEED;
    }
    if rl.is_key_down(KEY_A){
        rect.x = rect.x - SPEED;
    }
}

fn random_move(position: &mut Vector2) {
    let move_x = rand::thread_rng().gen_range(0..SCREEN_WIDTH) as f32;
    let move_y = rand::thread_rng().gen_range(0..SCREEN_HEIGHT) as f32;
    position.x = move_x;
    position.y = move_y;
}

fn main() {

    // constants
    let mut vector = Vector2::new(32.0, 32.0);
    let mut rectangle_vector = Vec::<Rectangle>::new();
    let mut rectangle = Rectangle::new((SCREEN_WIDTH / 2) as f32 , (SCREEN_HEIGHT / 2) as f32, 32.0, 32.0);
    rectangle_vector.push(rectangle);

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        // Drawing Stuff
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::WHITE);
            // Draw Hello, world! and move it to a random spot
            d.draw_text("Hello, world!", vector.x as i32, vector.y as i32, 20, Color::BLACK);
            random_move(&mut vector);
            // Draw a rectangle and move it with arrows
            for item in rectangle_vector.iter(){
                d.draw_rectangle_rec(item, Color::BLUE);
            }
        }
        // Updating stuff
        {
            update_player(&mut rl, &mut rectangle_vector[0]);
            if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON){
                let mouse_position: Vector2 = rl.get_mouse_position();
                println!("x: {}, y: {}", mouse_position.x, mouse_position.y)
            }
        }
    } 
}
