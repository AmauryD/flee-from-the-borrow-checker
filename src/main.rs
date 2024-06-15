use entities::player;
use maps::demo::demo_map;
use rendering::screen::Screen;

mod game;
mod position;
mod maps;
mod entities;
pub mod utils;
pub mod user_input;
pub mod rendering;

fn main() {
    let player: player::Player = player::Player {
        position: position::Position { x: 1, y: 1 },
        name: String::from("Player 1"),
        score: 0,
    };

    let mut game = game::Game {
        map: demo_map(),
        player,
        screen: Screen::new(40, 10)
    };

    game.game_loop();
}

