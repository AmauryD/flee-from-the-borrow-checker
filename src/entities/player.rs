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
        match direction {
            Direction::Up => {
                if self.position.y > 0 {
                    self.position.y -= 1;
                }
            }
            Direction::Down => {
                if self.position.y < map.size_x() - 1 {
                    self.position.y += 1;
                }
            }
            Direction::Left => {
                if self.position.x > 0 {
                    self.position.x -= 1;
                }
            }
            Direction::Right => {
                if self.position.x < map.size_y() - 1 {
                    self.position.x += 1;
                }
            }
        }
    }
}