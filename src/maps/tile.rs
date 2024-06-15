use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum TileType {
    GRASS,
    WATER,
    MOUNTAIN,
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            TileType::GRASS => "🌿",
            TileType::WATER => "🌊",
            TileType::MOUNTAIN => "⛰️",
        })
    }
}
