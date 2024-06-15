use crate::entities::entity::Draw;

#[derive(Clone, Copy)]
pub enum TileType {
    GRASS,
    WATER,
    MOUNTAIN,
}

impl Draw for TileType {
    fn draw(&self) -> char {
        match self {
            TileType::GRASS => '🟩',
            TileType::WATER => '🟦',
            TileType::MOUNTAIN => '🟧',
        }
    }
}

