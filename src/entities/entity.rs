use crate::position::Position;

pub trait Draw {
    fn draw(&self) -> char;
}

pub trait Entity: Draw {
    fn position(&self) -> &Position;
}
