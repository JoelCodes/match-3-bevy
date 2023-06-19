use bevy::prelude::*;

use super::grid::TileType;

#[derive(Resource, Default, Clone, Copy, Debug, Reflect)]
pub struct GameConfig {
    pub rows: usize,
    pub columns: usize,
    pub cell_size: f32,
}

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct GameData {
    pub grid: Vec<Vec<Option<TileType>>>,
    pub is_ready: bool,
}