use bevy::{prelude::*, utils::HashSet};

use super::{components::*, resources::*};

#[derive(Debug, Clone, Copy)]
pub struct TileDragStart{
    pub column: usize,
    pub row: usize,
    pub start_coord: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct TileDragMove {
    pub delta_coord: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct TileSwap {
    pub from: Vec2,
    pub to: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct TileDragCancel {}

#[derive(Reflect, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SwapDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component, Debug, Reflect)]
pub struct TileIsDragging {
    pub dragging_tile: (usize, usize),
    pub start_coord: Vec2,
    pub delta_coord: Vec2,
    #[reflect(ignore)]
    pub direction: Option<SwapDirection>,
    #[reflect(ignore)]
    pub banned_directions: HashSet<SwapDirection>,
}

const MIN_RAD:f32 = 0.2;

impl TileIsDragging {
    pub fn live_delta(&self) -> Vec2 {
        let mut constrained_delta = self.delta_coord.clamp(Vec2::new(-1., -1.), Vec2::new(1., 1.));
        if self.banned_directions.contains(&SwapDirection::Left) {
            constrained_delta.x = constrained_delta.x.max(0.);
        }
        if self.banned_directions.contains(&SwapDirection::Right) {
            constrained_delta.x = constrained_delta.x.min(0.);
        }
        if self.banned_directions.contains(&SwapDirection::Down) {
            constrained_delta.y = constrained_delta.y.max(0.);
        }
        if self.banned_directions.contains(&SwapDirection::Up) {
            constrained_delta.y = constrained_delta.y.min(0.);
        }
        constrained_delta * match self.direction {
            Some(SwapDirection::Left) | Some(SwapDirection::Right) => Vec2::new(1., MIN_RAD),
            Some(SwapDirection::Up) | Some(SwapDirection::Down) => Vec2::new(MIN_RAD, 1.),
            None => Vec2::new(MIN_RAD, MIN_RAD),
        }
    }
    fn next_direction(&mut self, min_rad: f32) -> Option<SwapDirection> {
        let (x, y) = (self.delta_coord.x, self.delta_coord.y);
        let (x_a, y_a) = (x.abs(), y.abs());
        if x_a < min_rad && y_a < min_rad {
            return None;
        }
        match self.direction {
            None => {
                if x_a > y_a {
                    return Some(if x > 0. {
                        SwapDirection::Right
                    } else {
                        SwapDirection::Left
                    });
                } else {
                    Some(if y > 0. {
                        SwapDirection::Up
                    } else {
                        SwapDirection::Down
                    })
                }
            }
            // We can easily switch backward and forward on the horizontal axis
            Some(SwapDirection::Left) | Some(SwapDirection::Right) => Some(if x > 0. {
                SwapDirection::Right
            } else {
                SwapDirection::Left
            }),
            // We can easily switch backward and forward on the vertical axis
            Some(SwapDirection::Up) | Some(SwapDirection::Down) => Some(if y < 0. {
                SwapDirection::Down
            } else {
                SwapDirection::Up
            }),
        }        
    }
    pub fn move_tile(&mut self, delta_coord: Vec2) {
        self.delta_coord += delta_coord;
        self.direction = self.next_direction(MIN_RAD);
    }
}

#[derive(Component, Debug, Reflect)]
pub struct SwapTile;

#[derive(Component, Debug, Reflect)]
pub struct ResetTile;

pub fn handle_tile_drag_start(
    mut commands: Commands,
    mut tile_drag_start_evr: EventReader<TileDragStart>,
    game_config: Res<GameConfig>,
    mut game_data: Query<(Entity, &GameData)>,
    mut tiles: Query<(Entity, &Tile)>
  ) {
    for event in tile_drag_start_evr.iter() {
      println!("Tile drag start: {:?}", event);
      for (entity, _) in game_data.iter_mut() {
        let mut banned_directions = HashSet::new();
        if event.column == 0 {
          banned_directions.insert(SwapDirection::Left);
        }
        if event.column == game_config.columns - 1 {
          banned_directions.insert(SwapDirection::Right);
        }
        if event.row == 0 {
          banned_directions.insert(SwapDirection::Down);
        }
        if event.row == game_config.rows - 1 {
          banned_directions.insert(SwapDirection::Up);
        }
        commands.entity(entity).insert(TileIsDragging {
          start_coord: event.start_coord,
          delta_coord: Vec2::new(0.0, 0.0),
          dragging_tile: (event.column, event.row),
          direction: None,
          banned_directions,
        });
      }
      for (entity, tile) in tiles.iter_mut() {
        if tile.column == event.column && tile.row == event.row {
          commands.entity(entity).insert(SwapTile);
          break;
        }
      }
    }
  }
  
  pub fn handle_tile_drag_cancel(
    mut commands: Commands,
    mut tile_drag_cancel_evr: EventReader<TileDragCancel>,
    mut game_data: Query<(Entity, &TileIsDragging)>,
    mut swap_tile: Query<(Entity, &Transform, &SwapTile, &Tile)>,
  ) {
    if tile_drag_cancel_evr.iter().next().is_none() {
      return;
    }
    println!("cancel");
    for (tile_entity, transform, swap_tile, tile) in swap_tile.iter_mut() {
      println!("Clearing swap tile: {:?} {:?} {:?}", transform.translation, tile, swap_tile);
      commands.entity(tile_entity).remove::<SwapTile>().insert(ResetTile);
    }
    for (entity, _) in game_data.iter_mut() {
      commands.entity(entity).remove::<TileIsDragging>();
    }
  }
  
  pub fn handle_tile_drag_move(
    mut tile_drag_move_evr: EventReader<TileDragMove>,
    game_config: Res<GameConfig>,
    mut game_data: Query<&mut TileIsDragging>,
    mut swap_tile: Query<(&mut Transform, &Tile), With<SwapTile>>
  ) {
    for event in tile_drag_move_evr.iter() {
      
      if let Some(mut tile_is_dragging) = game_data.iter_mut().next() {
        tile_is_dragging.move_tile(event.delta_coord / game_config.cell_size);
        if let Some((mut transform, tile)) = swap_tile.iter_mut().next() {
          let flat =(Vec2::new(tile.column as f32, tile.row as f32) + tile_is_dragging.live_delta()) * game_config.cell_size;
          transform.translation = Vec3::new(flat.x, flat.y, 1.0);
        }
      }
    }
  }
  

pub fn reset_tiles(
  mut tiles_to_reset: Query<(Entity, &Tile, &mut Transform), With<ResetTile>>,
  game_config: Res<GameConfig>,
  mut commands: Commands,
) {
  for (entity, tile, mut transform) in tiles_to_reset.iter_mut() {
    println!("Resetting tile: {:?}", tile);
    transform.translation = Vec3::new(
      tile.column as f32 * game_config.cell_size,
      tile.row as f32 * game_config.cell_size,
      0.0,
    );
    commands.entity(entity).remove::<ResetTile>();
  }
}