use crate::{entities::crab::Crab, position::Position};

use super::{map::{BoardSize, Map}, tile::TileType};


pub fn demo_map() -> Map {
    let mut tiles = vec![];

    for _ in 0..100 {
        tiles.push(TileType::GRASS);
    }

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