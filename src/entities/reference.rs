use crate::position::Position;

use super::entity::{Draw, Entity};

#[derive(PartialEq)]
pub struct Reference {
    position: Position,
}

impl Reference {
    pub fn new(position: Position) -> Reference {
        Reference {
            position,
        }
    }
}

impl Entity for Reference {
    fn position(&self) -> &Position {
        &self.position
    }
}

impl Draw for Reference {
    fn draw(&self) -> char {
        '&'
    }
}