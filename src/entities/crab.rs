use crate::position::Position;
use super::entity::Entity;

pub struct Crab {
    position: Position,
}

impl Entity for Crab {
    fn position(&self) -> &Position {
        &self.position
    }

    fn draw(&self) -> String {
        '🦀'.to_string()
    }
}  

impl Crab {
    pub fn new(position: Position) -> Self {
        Crab {
            position: Position {
                x: position.x,
                y: position.y,  
            }
        }
    }
}