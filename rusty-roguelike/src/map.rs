pub use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Floor,
    Wall,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    return ((y * SCREEN_WIDTH) + x) as usize;
}

impl Map {
    pub fn new() -> Self {
        return Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        return point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT;
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        return self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor;
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if self.in_bounds(point) {
            return Some(map_idx(point.x, point.y));
        } else {
            return None;
        }
    }
}
