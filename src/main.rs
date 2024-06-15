use entities::player::{self, Player};
use game::Game;
use maps::demo::demo_map;
use rendering::screen::Screen;

mod game;
mod position;
mod maps;
mod entities;
pub mod utils;
pub mod user_input;
pub mod rendering;
mod inventory;

fn main() {
    let player = Player::new("Young Rustacean".to_owned());

    let mut game = Game::new(demo_map(), player, Screen::new(80, 10));

    match game.game_loop() {
        Ok(_) => (),
        Err(e) => println!("An error occurred {:?}", e.source()),
    } 
}

