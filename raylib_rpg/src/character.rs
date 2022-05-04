// File to hold the stucts with the characters
use raylib::prelude::*;

pub struct Player {
    pub rect: Rectangle,
    pub hp: u32
}

impl Player {

    pub fn take_damage(self, damage: &u32){
        self.hp - damage;
    }

    pub fn get_hp(&self) -> u32{
        self.hp
    }
}
