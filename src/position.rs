#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}