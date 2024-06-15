use crate::{entities::player::Player, inventory::{item::Items, reference::Reference}, maps::map::Map, rendering::{draw_map::DrawMap, screen::Screen, side_menu::render_menu}};

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

            let remove: Option<usize> = {
                let entity_on_player = self.map.get_entity_at(self.player.position.x, self.player.position.y);
                let mut ret: Option<usize> = None;
                
                if let Some((index,entity_on_player)) = entity_on_player {
                    match entity_on_player {
                        crate::entities::entity::Entities::Crab(_) => {
                            self.player.health -= 10;
                        },
                        crate::entities::entity::Entities::Reference(r) => {
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
    }
}