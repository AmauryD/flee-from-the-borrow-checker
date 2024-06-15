use entities::player;
use maps::demo::demo_map;

mod game;
mod position;
mod rendering;
pub mod utils;
pub mod user_input;
mod maps;
mod entities;

fn main() {
    let player: player::Player = player::Player {
        position: position::Position { x: 1, y: 1 },
        name: String::from("Player 1"),
        score: 0,
    };

    let mut game = game::Game {
        map: demo_map(),
        player
    };

    game.game_loop();
}

