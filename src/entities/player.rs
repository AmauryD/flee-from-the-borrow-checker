use crate::{maps::map::Map, position::Position};

use super::entity::Entity;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub position: Position,
}

impl Entity for Player {
    fn draw(&self) -> String {
        "🧍".to_string()
    }
    
    // same lifetime than the struct
    fn position(&self) -> &Position {
        &self.position
    }
}

impl Player {
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