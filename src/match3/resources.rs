use bevy::prelude::*;

#[derive(Resource, Default, Clone, Copy, Debug, Reflect)]
pub struct GameConfig {
    pub rows: usize,
    pub columns: usize,
    pub cell_size: f32,
}
