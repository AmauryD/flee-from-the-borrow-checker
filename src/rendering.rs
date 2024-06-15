use crate::{entities::{entity::Entity, player::Player}, maps::map::Map};

pub trait DrawMap {
    fn draw_map(map: &Map, player: &Player, entities: &Vec<Box<dyn Entity>>);
}

pub struct ConsoleRenderer;

impl DrawMap for ConsoleRenderer {
    // pseudo layer rendering
    // entities always render on top of the map
    fn draw_map(map: &Map, player: &Player, entities: &Vec<Box<dyn Entity>>) {
        for y in 0..map.size_x() {
            for x in 0..map.size_y() {
                // player is a special entity, it is not tied to the map but to the game
                if player.position().x == x && player.position().y == y {
                    print!("{}", player.draw());
                    continue;
                }

                let draw_something = draw_entity(entities, x, y);

                if !draw_something {
                    draw_tile(map, x, y);
                }
            }
            print!("\n");
        }
    }
}

fn draw_tile(map: &Map, x: u8, y: u8) {
    print!("{}", map.get_tile(x, y));
}

fn draw_entity(entities: &Vec<Box<dyn Entity>>, x: u8, y: u8) -> bool {
    let mut found = false;
    for entity in entities {
        let position = entity.position();

        if position.x == x && position.y == y {
            print!("{}", entity.draw());
            found = true;
            break;
        }
    }
    found
}

