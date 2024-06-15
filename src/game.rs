use crate::{entities::player::Player, maps::map::Map, rendering::{draw_map::DrawMap, menu::render_menu, screen::Screen}};

pub struct Game{
    // 8 x 8
    pub map: Map,
    pub player: Player,
    pub screen: Screen
}

impl Game {
    pub fn game_loop(&mut self) {
        loop {
            self.map.draw_map(&mut self.screen, &self.player).unwrap();
            render_menu(self, self.map.size_x(), 0).unwrap();
            self.screen.draw_screen().unwrap();

            self.handle_input();

            if let Some(entity) = self.map.get_entity_at(self.player.position.x, self.player.position.y) {
                
            }
        }
    }
}