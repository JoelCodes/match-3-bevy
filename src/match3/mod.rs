use bevy::{prelude::*};

mod grid;
use self::grid::TileType;
mod resources;
use self::resources::{GameConfig, GameData};
mod components;
use self::components::Tile;
mod systems;
use self::systems::setup;

pub struct Match3Plugin;

impl Plugin for Match3Plugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameConfig>()
            .insert_resource(GameConfig {
                rows: 6,
                columns: 6,
                cell_size: 80.,
            })
            .init_resource::<GameData>()
            .insert_resource(GameData {
                grid: vec![],
                is_ready: false,
            })
            .register_type::<TileType>()
            .register_type::<Tile>()
            .register_type::<GameConfig>()
            .register_type::<GameData>()
            .add_startup_system(setup);
    }
}
