use crate::position::Position;

use super::{crab::Crab, reference::Reference};

pub trait Draw {
    fn draw(&self) -> char;
}

pub trait Entity: Draw {
    fn position(&self) -> &Position;
}

#[derive(PartialEq)]
pub enum Entities {
    Crab(Crab),
    Reference(Reference)
}

impl Draw for Entities {
    fn draw(&self) -> char {
        match self {
            Entities::Crab(c) => c.draw(),
            Entities::Reference(r) => r.draw() 
        }
    }
}

impl Entity for Entities {
    fn position(&self) -> &Position {
        match self {
            Entities::Crab(c) => c.position(),
            Entities::Reference(r) => r.position()
        }
    }
}