use crate::position::Position;

use super::entity::{Draw, Entity};

pub struct Reference {
    position: Position,
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