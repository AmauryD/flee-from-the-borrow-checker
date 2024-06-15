use crate::position::Position;

pub trait Entity {
    fn position(&self) -> &Position;
    fn draw(&self) -> String;
}