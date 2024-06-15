use crate::{game::Game, player::Direction, utils::user_input};

impl Game {
    pub fn handle_input(&mut self) {
        let input = user_input();

        match input.as_str() {
            "z\n" => self.player.mov(Direction::Up, &self.map),
            "s\n" => self.player.mov(Direction::Down, &self.map),
            "q\n" => self.player.mov(Direction::Left, &self.map),
            "d\n" => self.player.mov(Direction::Right, &self.map),
            _ => (),
        }
    }
} 