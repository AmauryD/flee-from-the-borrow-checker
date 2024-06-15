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