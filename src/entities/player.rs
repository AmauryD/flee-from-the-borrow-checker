use crate::{inventory::inventory::Inventory, maps::map::Map, position::Position};

use super::entity::{Draw, Entity};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player {
    pub name: String,
    pub score: u32,
    pub position: Position,
    pub inventory: Inventory,
    pub health: u8,
}

impl Draw for Player {
    fn draw(&self) -> char {
        '🧍'
    }
}

impl Entity for Player {
    /**
     * Player position on map
     */
    fn position(&self) -> &Position {
        &self.position
    }
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            score: 0,
            position: Position::default(),
            inventory: Inventory::new(),
            health: 100
        }
    }

    pub fn mov(&mut self, direction: Direction, map: &Map) {
        let future_position: Position = match direction {
            Direction::Up => {
                if self.position.y > 0 {
                    Position::new(self.position.x, self.position.y - 1)
                } else {
                    self.position
                }
            }
            Direction::Down => {
                if self.position.y < map.size_x() - 1 {
                    Position::new(self.position.x, self.position.y + 1)
                } else {
                    self.position
                }
            }
            Direction::Left => {
                if self.position.x > 0 {
                    Position::new(self.position.x - 1, self.position.y)
                } else {
                    self.position
                }
            }
            Direction::Right => {
                if self.position.x < map.size_y() - 1 {
                    Position::new(self.position.x + 1, self.position.y)
                } else {
                    self.position
                }
            }
        };

        if map.get_tile(future_position.x, future_position.y).is_walkable() {
            self.position = future_position;
        }
    }
}