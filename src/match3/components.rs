use bevy::prelude::*;

use super::grid::TileType;

#[derive(Component, Debug, Reflect)]
pub struct Tile {
    pub tile_type: TileType,
    pub column: usize,
    pub row: usize,
}
