use crate::{entities::{player::Player}, maps::map::Map, rendering::{ConsoleRenderer, DrawMap}};

pub struct Game{
    // 8 x 8
    pub map: Map,
    pub player: Player,
}

impl Game {
    pub fn game_loop(&mut self) {
        loop {
            let entities = &self.map.get_entities();

            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            ConsoleRenderer::draw_map(
                &self.map, 
                &self.player,
                entities
            );
            self.handle_input();
        }
    }
}