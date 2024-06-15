use entities::player::{self, Player};
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

    let mut game = game::Game {
        map: demo_map(),
        player,
        screen: Screen::new(40, 10)
    };

    game.game_loop();
}

