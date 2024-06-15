use crate::entities::entity::{Entities, Entity};
use super::tile::TileType;

pub struct BoardSize(pub u8, pub u8);

pub struct Map {
    tiles: Vec<TileType>,
    board_size: BoardSize,
    pub level: u8,
    entities: Vec<Entities>,
}

impl Map {
    pub fn new(tiles: Vec<TileType>, board_size: BoardSize, level: u8, entities: Vec<Entities>) -> Self {
        if board_size.0 * board_size.1 != tiles.len() as u8 {
            panic!("Invalid map size");
        }

        Map {
            tiles,
            board_size,
            level,
            entities,
        }
    }

    pub fn remove_entity_at(&mut self, index: usize) {
        self.entities.remove(index);
    }

    pub fn get_entity_at(&self, x: u8, y: u8) -> Option<(usize,&Entities)> {
        for entity in self.entities.iter().enumerate() {
            let position = entity.1.position();
            if position.x == x && position.y == y {
                return Some(entity);
            }
        }

        None
    }

    pub fn get_entities(&self) -> &Vec<Entities> {
        &self.entities
    }

    pub fn get_tile(&self, x: u8, y: u8) -> &TileType {
        &self.tiles[(y * self.board_size.0 + x) as usize]
    }

    pub fn size_x(&self) -> u8 {
        self.board_size.0
    }

    pub fn size_y(&self) -> u8 {
        self.board_size.1
    }
}