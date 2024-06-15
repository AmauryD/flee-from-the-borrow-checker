use crate::{game::GameState, utils::user_input};

use super::screen::Screen;

pub struct MainMenu;

impl MainMenu {
    pub fn draw(screen: &mut Screen) {
        screen.set_string_at(0, 0, "=== 🦀 WELCOME RUSTACEAN 🦀 ===".to_owned());
        screen.set_string_at(0, 1, "☠ The borrow checker is trying to kill all the references ! ☠".to_owned());
        screen.set_string_at(0, 2, "Save them before it's too late!".to_owned());
        screen.set_string_at(0, 3, "Be careful of the crabs. 🦀🦀".to_owned());

        screen.set_string_at(0, 4, "> Press 'p' to play".to_owned());
        screen.set_string_at(0, 5, "> Press 'q' to quit".to_owned());
    }

    pub fn handle_input() -> Result<GameState, std::io::Error> {
        let input = user_input()?.chars().next().unwrap();
        match input {
            'p' => Ok(GameState::Playing),
            'q' => Ok(GameState::Quit),
            _ => Ok(GameState::Menu),
        }
    }
}