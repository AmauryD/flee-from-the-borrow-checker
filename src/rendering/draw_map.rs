use crate::{entities::{entity::{Draw, Entity}, player::Player}, maps::map::Map};

use super::screen::Screen;

const MAP_SIZE_ON_SCREEN: (u8, u8) = (10, 10);
const WORLD_BORDER_CHAR: char = '❌';

pub trait DrawMap {
    fn draw_map(&self, screen: &mut Screen, player: &Player) -> Result<(), std::io::Error>;
}

impl DrawMap for Map {
    /*  
        Pseudo layer rendering
        Entities always render on top of the map
        Relative to the player. always render the player in the center of the screen
    */
    fn draw_map(&self, screen: &mut Screen, player: &Player) -> Result<(), std::io::Error>  {
        let player_position = player.position();

        for screen_y in 0..MAP_SIZE_ON_SCREEN.1 {
            for screen_x in 0..MAP_SIZE_ON_SCREEN.0 {
                let map_x: i8 = player_position.x as i8 - MAP_SIZE_ON_SCREEN.0 as i8 / 2 + screen_x as i8;
                let map_y = player_position.y as i8 - MAP_SIZE_ON_SCREEN.1 as i8 / 2 + screen_y as i8;

                if map_x < 0 || map_y < 0 || map_x >= self.size_x() as i8 || map_y >= self.size_y() as i8 {
                    screen.set_char_at(screen_x, screen_y, WORLD_BORDER_CHAR);
                    continue;
                }
                
                if player_position.x == map_x as u8 && player_position.y == map_y as u8 {
                    screen.set_char_at(screen_x, screen_y, player.draw());
                    continue;
                }

                let mut found = false;
                for entity in self.get_entities() {
                    let position = entity.position();
                    if position.x == map_x as u8 && position.y == map_y as u8 {
                        screen.set_char_at(screen_x, screen_y, entity.draw());
                        found = true;
                    }
                }

                if found {
                    continue;
                }

                screen.set_char_at(screen_x, screen_y, self.get_tile(map_x as u8, map_y as u8).draw());
            }
        }

        return Ok(());
    }
}

