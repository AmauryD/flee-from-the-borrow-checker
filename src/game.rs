use std::error::Error;

use crate::{entities::player::Player, inventory::{item::Items, reference::Reference}, maps::map::Map, rendering::{draw_map::DrawMap, main_menu::MainMenu, screen::Screen, side_menu::render_menu}};

pub enum GameState {
    Menu,
    Playing,
    GameOver,
    Win,
    Quit
}

pub struct Game{
    pub map: Map,
    pub player: Player,
    pub screen: Screen,
    pub state: GameState
}

impl Game {
    pub fn new(map: Map, player: Player, screen: Screen) -> Self {
        Self {
            map,
            player,
            screen,
            state: GameState::Menu
        }
    }

    pub fn game_loop(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            match self.state {
                GameState::Menu => {
                    MainMenu::draw(&mut self.screen);
                    self.screen.draw()?;
                    self.state = MainMenu::handle_input()?;
                },
                GameState::Playing => {
                    self.map.draw_map(&mut self.screen, &self.player)?;
                    render_menu(self, self.map.size_x(), 0)?;
                    self.screen.draw()?;
                    self.state = self.handle_input()?;
                    self.compute_interactions();
                    if self.check_win_condition() {
                        self.state = GameState::Win;
                    }
                    if self.check_loose_condition() {
                        self.state = GameState::GameOver;
                    }
                },
                GameState::Quit => {
                    self.screen.write_centered_string("See you soon ! 👋".to_owned());
                    self.screen.draw()?;
                    break Ok(());
                },
                GameState::GameOver => {
                    self.screen.write_centered_string("Game Over 💀".to_owned());
                    self.screen.draw()?;
                    break Ok(());
                },
                GameState::Win => {
                    self.screen.write_centered_string("*** You Won ! 🎉 ***".to_owned());
                    self.screen.draw()?;
                    break Ok(());
                },
            }
        }
    }


    fn compute_interactions(&mut self) {
        let remove: Option<usize> = {
            let entity_on_player = self.map.get_entity_at(self.player.position.x, self.player.position.y);
            let mut ret: Option<usize> = None;
    
            if let Some((index,entity_on_player)) = entity_on_player {
                match entity_on_player {
                    crate::entities::entity::Entities::Crab(_) => {
                        self.player.health -= 10;
                    },
                    crate::entities::entity::Entities::Reference(_) => {
                        self.player.score += 10;
                        self.player.inventory.add_item(
                            Items::Reference(Reference::new())
                            );
                        ret = Some(index);
                    }
                };
            }
    
            ret
        };
    
        if let Some(index) = remove {
            self.map.remove_entity_at(index);
        }
    }

    /**
     * There must be no more reference on the map
     */
    fn check_win_condition(&self) -> bool {
        self.map.get_entities().iter().all(|entity| {
            match entity {
                crate::entities::entity::Entities::Reference(_) => false,
                _ => true
            }
        })
    }

    fn check_loose_condition(&self) -> bool {
        self.player.health == 0
    }
}

