use bevy::{prelude::*};

mod grid;
use self::grid::*;
mod resources;
use self::resources::*;
mod components;
use self::components::*;
mod systems;
use self::systems::*;
mod bundles;

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
            .register_type::<TileType>()
            .register_type::<Tile>()
            .register_type::<GameConfig>()
            .register_type::<GameData>()
            .add_startup_system(setup);
    }
}
