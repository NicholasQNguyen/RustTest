// File to hold the stucts with the characters
use raylib::prelude::*;

#[derive (Clone, Copy)]
pub struct Player {
    pub rect: Rectangle,
    pub hp: u32
}

impl Player {

    pub fn take_damage(mut self, damage: u32) -> u32 {
        self.hp - damage
    }

    pub fn get_hp(&self) -> u32 {
        self.hp
    }
}
