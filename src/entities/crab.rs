use crate::position::Position;
use super::entity::{Draw, Entity};

#[derive(PartialEq)]
pub struct Crab {
    position: Position,
}

impl Draw for Crab {
    fn draw(&self) -> char {
        '🦀'
    }
}

impl Entity for Crab {
    fn position(&self) -> &Position {
        &self.position
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