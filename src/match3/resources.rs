use bevy::prelude::*;

use super::grid::*;

#[derive(Resource, Default, Clone, Copy, Debug, Reflect)]
pub struct GameConfig {
  pub rows: usize,
  pub columns: usize,
  pub cell_size: f32,
}

#[derive(Component, Default, Debug, Reflect)]
pub struct GameData {
  pub grid: Vec<Vec<Option<TileType>>>,
}


