use bevy::{prelude::*};

mod grid;
mod resources;
mod components;
mod systems;
mod bundles;
mod system_sets;
mod setup;
mod drag;
use self::grid::*;
use self::resources::*;
use self::components::*;
use self::setup::*;
use self::systems::*;
use self::system_sets::*;
use self::drag::*;

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
            .register_type::<TileIsDragging>()
            .add_event::<TileDragStart>()
            .add_event::<TileDragMove>()
            .add_event::<TileDragCancel>()
            .add_startup_systems((setup_camera, setup_grid))
            .configure_set(
                GameEvents
                    .after(MouseInput)
                    .before(Cleanup)
            )
            .add_systems((handle_mousebtn, handle_mousemove).in_set(MouseInput))
            .add_systems(
                (
                    handle_tile_drag_start,
                    handle_tile_drag_cancel,
                    handle_tile_drag_move
                ).in_set(GameEvents)
            ).add_system(reset_tiles.in_set(Cleanup));
    }
}
