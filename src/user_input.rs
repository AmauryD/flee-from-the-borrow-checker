use crate::{game::{Game, GameState}, player::Direction, utils::user_input};

impl Game {
    pub fn handle_input(&mut self) -> Result<GameState, Box<dyn std::error::Error>> {
        let input = user_input()?;

        match input.as_str() {
            "z" => self.player.mov(Direction::Up, &self.map),
            "s" => self.player.mov(Direction::Down, &self.map),
            "q" => self.player.mov(Direction::Left, &self.map),
            "d" => self.player.mov(Direction::Right, &self.map),
            "m" => {
                return Ok(GameState::Menu);
            },
            _ => {}
        }

        Ok(GameState::Playing)
    }
} 