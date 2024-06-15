use crate::{entities::crab::Crab, position::Position};

use super::{map::{BoardSize, Map}, tile::TileType};


pub fn demo_map() -> Map {
    let mut tiles = vec![];

    for i in 0..100 {
        tiles.push(TileType::GRASS);
    }

    tiles[42] = TileType::WATER;
    tiles[43] = TileType::WATER;
    tiles[32] = TileType::WATER;
    tiles[33] = TileType::WATER;

    tiles[55] = TileType::MOUNTAIN;
    tiles[56] = TileType::MOUNTAIN;

    Map::new(tiles, BoardSize(10, 10), 1, vec![
        Box::new(
            Crab::new(
                Position { x: 2, y: 2 },
            )
        ),
        Box::new(
            Crab::new(
                Position { x: 3, y: 2 },
            )
        ),
        Box::new(
            Crab::new(
                Position { x: 5, y: 3 },
            )
        )
    ])
}