use bevy::prelude::*;

mod grid;
mod resources;
mod components;
mod input;
mod bundles;
mod system_sets;
mod setup;
mod drag;
use self::grid::*;
use self::resources::*;
use self::components::*;
use self::setup::*;
use self::input::*;

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
      .add_startup_systems((setup_camera, setup_grid));
    add_input_to_app(app);
    add_drag_to_app(app);
  }
}
