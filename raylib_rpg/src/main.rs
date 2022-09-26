use raylib::prelude::*;
mod character;
use crate::character::Player;


// global constants
const SCREEN_WIDTH: i32 = 720;
const SCREEN_HEIGHT: i32 = 480;


fn main() {
    let player_rect = Rectangle{
                            height: 32.0,
                            width:  32.0,
                            x:  32.0,
                            y:  32.0
                      };

    let mut player = Player{
                        rect: player_rect,
                        hp: 100
                 };

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("RPG")
        .build();

    while !rl.window_should_close() {
        // Drawing Stuff
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 32, 32, 20, Color::BLACK);
        d.draw_rectangle_rec(&player.rect, Color::BLUE);
        
        println!("{}", player.get_hp());
        player.hp = player.take_damage(5);
        
    } 
}
