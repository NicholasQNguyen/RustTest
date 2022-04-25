use raylib::prelude::*;
use raylib::ffi::KeyboardKey::*;
use raylib::ffi::MouseButton::*;
use rand::Rng;

// Import my rust files
// mod debug_functions;

// global constants
const SCREEN_WIDTH: i32 = 720;
const SCREEN_HEIGHT: i32 = 480;
const PLAYER_SPEED: f32 = 0.05;
const ENEMY_SPEED: f32 = 0.01;


fn update_player(rl: &RaylibHandle, rect: &mut Rectangle){
    if rl.is_key_down(KEY_W){
        rect.y = rect.y - PLAYER_SPEED;
    }
    if rl.is_key_down(KEY_S){
        rect.y = rect.y + PLAYER_SPEED;
    }
    if rl.is_key_down(KEY_D){
        rect.x = rect.x + PLAYER_SPEED;
    }
    if rl.is_key_down(KEY_A){
        rect.x = rect.x - PLAYER_SPEED;
    }
}

fn random_move(position: &mut Vector2) {
    let move_x = rand::thread_rng().gen_range(0..SCREEN_WIDTH) as f32;
    let move_y = rand::thread_rng().gen_range(0..SCREEN_HEIGHT) as f32;
    position.x = move_x;
    position.y = move_y;
}

fn move_to_player(player_rect: Rectangle, enemy_rect: &mut Rectangle){
    if player_rect.x < enemy_rect.x {
        enemy_rect.x -= ENEMY_SPEED;
    }
    else {
        enemy_rect.x += ENEMY_SPEED;
    }

    if player_rect.y < enemy_rect.y {
        enemy_rect.y -= ENEMY_SPEED;
    }

    else {
        enemy_rect.y += ENEMY_SPEED;
    }

}

fn main() {

    // constants
    let mut vector = Vector2::new(32.0, 32.0);
    let mut rectangle_vector = Vec::<Rectangle>::new();
    let rectangle = Rectangle::new((SCREEN_WIDTH / 2) as f32 , (SCREEN_HEIGHT / 2) as f32, 32.0, 32.0);
    rectangle_vector.push(rectangle);
    let mut sleep_timer: f32 = 2.0;

    let mut hit: bool = false;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        if !hit {
            // Drawing Stuff
            {
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::WHITE);
                // Draw Hello, world! and move it to a random spot
                d.draw_text("Hello, world!", vector.x as i32, vector.y as i32, 20, Color::BLACK);
                if sleep_timer <= 0.0 {
                    random_move(&mut vector);
                    sleep_timer = 2.0;
                }
                // Draw a rectangle and move it with arrows
                for item in rectangle_vector.iter() {
                    d.draw_rectangle_rec(item, Color::BLUE);
                }
            }

            // Updating stuff
            {
                update_player(&mut rl, &mut rectangle_vector[0]);
                // place a rectangle at the player's mouse
                if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
                    let mouse_position: Vector2 = rl.get_mouse_position();
                    println!("x: {}, y: {}", mouse_position.x, mouse_position.y);
                    let new_rectangle = Rectangle::new(mouse_position.x, mouse_position.y, 16.0, 16.0);
                    rectangle_vector.push(new_rectangle);
                }
                if rectangle_vector.len() > 1 {
                    for i in 1..rectangle_vector.len() {
                        move_to_player(rectangle_vector[0],&mut rectangle_vector[i]);
                    }
                }
                // Check for collisions and end the game if there is one
                for i in 1..rectangle_vector.len() {
                    if rectangle_vector[0].check_collision_recs(&rectangle_vector[i]){
                        hit = true;
                    }
                }

                sleep_timer -= rl.get_frame_time();
            }
        }
        // Got hit
        else {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::WHITE);
            d.draw_text("Game over", SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, 20, Color::BLACK);
            
        }
    } 
}
