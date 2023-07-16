use bevy::{prelude::*, utils::{HashSet, HashMap}};

use super::{components::*, resources::*, grid::*};

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

#[derive(Debug, Clone, Copy)]
pub struct TileDragEnd {}

#[derive(Debug, Clone, Copy)]
pub struct TileDragSuccess {}

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

#[derive(Component, Debug)]
pub struct DragDirection {
  pub direction: Option<SwapDirection>,
}

#[derive(Component, Debug, Clone)]
pub struct DragNeighbours {
  pub neighbours: HashMap<SwapDirection, Entity>,
}

#[derive(Component, Debug, Reflect)]
pub struct ResetDrag;

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
    match self.direction {
        Some(SwapDirection::Left)|Some(SwapDirection::Right) => {
            constrained_delta.y = constrained_delta.y.max(-MIN_RAD).min(MIN_RAD);
        }
        Some(SwapDirection::Down)|Some(SwapDirection::Up) => {
            constrained_delta.x = constrained_delta.x.max(-MIN_RAD).min(MIN_RAD);
        }
        None => {}
    }
    constrained_delta
  }
  pub fn next_direction(&mut self) -> Option<SwapDirection>{
    let (x, y) = self.delta_coord.into();
    let (x_abs, y_abs) = (x.abs(), y.abs());
    if x_abs < MIN_RAD && y_abs < MIN_RAD {
      None
    } else if x_abs > y_abs {
      if x > 0. {
        Some(SwapDirection::Right)
      } else {
        Some(SwapDirection::Left)
      }
    } else {
      if y > 0. {
        Some(SwapDirection::Up)
      } else {
        Some(SwapDirection::Down)
      }
    }
  }
  pub fn move_tile(&mut self, delta_coord: Vec2) {
    self.delta_coord += delta_coord;
    self.direction = self.next_direction();
  }
}

#[derive(Component, Debug, Reflect)]
pub struct SwapTile;

#[derive(Component, Debug, Reflect)]
pub struct UnderSwapTile;

#[derive(Component, Debug, Reflect)]
pub struct ResetTile;

pub fn handle_tile_drag_start(
    mut commands: Commands,
    mut tile_drag_start_evr: EventReader<TileDragStart>,
    game_config: Res<GameConfig>,
    mut game_data: Query<(Entity, &GameData)>,
    mut tiles: Query<(Entity, &Tile)>
  ) {
    for evt in tile_drag_start_evr.iter() {
      println!("Tile drag start: {:?}", evt);
      for (entity, _) in game_data.iter_mut() {
        let mut banned_directions = HashSet::new();
        if evt.column == 0 {
          banned_directions.insert(SwapDirection::Left);
        }
        if evt.column == game_config.columns - 1 {
          banned_directions.insert(SwapDirection::Right);
        }
        if evt.row == 0 {
          banned_directions.insert(SwapDirection::Down);
        }
        if evt.row == game_config.rows - 1 {
          banned_directions.insert(SwapDirection::Up);
        }
        commands.entity(entity).insert(TileIsDragging {
          start_coord: evt.start_coord,
          delta_coord: Vec2::new(0.0, 0.0),
          dragging_tile: (evt.column, evt.row),
          direction: None,
          banned_directions,
        }).insert(DragDirection {
          direction: None,
        });
        let mut neighbours = HashMap::new();

        for (entity, tile) in tiles.iter_mut() {
          if tile.column == evt.column {
            if tile.row == evt.row {
              commands.entity(entity).insert(SwapTile);
            } else if tile.row == evt.row - 1 {
              neighbours.insert(SwapDirection::Down, entity);
            } else if tile.row == evt.row + 1 {
              neighbours.insert(SwapDirection::Up, entity);
            }
          } else if tile.row == evt.row {
            if tile.column == evt.column - 1 {
              neighbours.insert(SwapDirection::Left, entity);
            } else if tile.column == evt.column + 1 {
              neighbours.insert(SwapDirection::Right, entity);
            }
          }
        }
        commands.entity(entity).insert(DragNeighbours {
          neighbours,
        });
      }
    }
  }
  
pub fn handle_tile_drag_end(
  mut commands: Commands,
  game_config: Res<GameConfig>,
  mut tile_drag_end_evr: EventReader<TileDragEnd>,
  mut game_data: Query<(Entity, &mut GameData, &DragDirection, &TileIsDragging)>,
  mut swap_tile: Query<(Entity, &mut Tile, Option<&SwapTile>, &mut Transform, &mut Name), Or<(With<SwapTile>, With<UnderSwapTile>)>>,
) {
  if tile_drag_end_evr.iter().next().is_none() {
    return;
  }
  
  for (entity, mut game_data, drag_direction, tile_is_dragging) in game_data.iter_mut() {
    match drag_direction.direction {
      None => {
        commands.entity(entity).insert(ResetDrag);  
      },
      Some(direction) => {
        let tile1 = tile_is_dragging.dragging_tile;
        let tile2 = match direction {
          SwapDirection::Left => (tile1.0 - 1, tile1.1),
          SwapDirection::Right => (tile1.0 + 1, tile1.1),
          SwapDirection::Down => (tile1.0, tile1.1 - 1),
          SwapDirection::Up => (tile1.0, tile1.1 + 1),
        };
        if can_swap(&mut game_data.grid, tile1, tile2) {
          for (_, mut tile, is_swap_tile, mut transform, mut name) in swap_tile.iter_mut() {
            if is_swap_tile.is_some() {
              tile.column = tile2.0;
              tile.row = tile2.1;
              transform.translation = Vec3::new(
                game_config.cell_size * tile2.0 as f32,
                game_config.cell_size * tile2.1 as f32,
                0.0,
              );
              name.set(format!("Tile {} {}", tile2.0, tile2.1));
            } else {
              tile.column = tile1.0;
              tile.row = tile1.1;
              transform.translation = Vec3::new(
                game_config.cell_size * tile1.0 as f32,
                game_config.cell_size * tile1.1 as f32,
                0.0,
              );
              name.set(format!("Tile {} {}", tile1.0, tile1.1));
            }
          }
          swap_tiles(&mut game_data.grid, tile1, tile2);
          
        } 
        commands.entity(entity).insert(ResetDrag);
      }
    }
    for (tile_entity, _, _, _, _) in swap_tile.iter_mut() {
      commands.entity(tile_entity).insert(ResetTile);
    }
}
}
  
pub fn handle_tile_drag_move(
  mut tile_drag_move_evr: EventReader<TileDragMove>,
  game_config: Res<GameConfig>,
  mut game_data: Query<(&mut TileIsDragging, &mut DragDirection)>,
  mut swap_tile: Query<(&mut Transform, &Tile, Option<&SwapTile>, Option<&UnderSwapTile>), Or<(With<SwapTile>, With<UnderSwapTile>)>>,
) {
  for event in tile_drag_move_evr.iter() {
    
    if let Some((mut tile_is_dragging, mut drag_direction)) = game_data.iter_mut().next() {
      tile_is_dragging.move_tile(event.delta_coord / game_config.cell_size);
      let new_direction = tile_is_dragging.next_direction();
      if new_direction != drag_direction.direction {
        drag_direction.direction = new_direction;
      }
      let drag_delta = tile_is_dragging.live_delta();
      let underswap_drag_delta = match drag_direction.direction {
        Some(SwapDirection::Left) | Some(SwapDirection::Right) => drag_delta * Vec2::new(-1.0, 0.0),
        Some(SwapDirection::Up) | Some(SwapDirection::Down) => drag_delta * Vec2::new(0.0, -1.0),
        None => Vec2::new(0.0, 0.0),
      };
      for (mut transform, tile, swap_tile, under_swap_tile) in swap_tile.iter_mut() {
        if swap_tile.is_some() {
          let flat =(Vec2::new(tile.column as f32, tile.row as f32) + drag_delta) * game_config.cell_size;
          transform.translation = Vec3::new(flat.x, flat.y, 2.0);            
        } else if under_swap_tile.is_some() {
          let flat =(Vec2::new(tile.column as f32, tile.row as f32) + underswap_drag_delta) * game_config.cell_size;
          transform.translation = Vec3::new(flat.x, flat.y, 1.0);  
        }
      }
    }
  }
}

pub fn handle_drag_direction_change(
  mut commands: Commands,
  drag_direction_q: Query<(&DragDirection, &DragNeighbours), Changed<DragDirection>>,
  mut last_underswap_tile:Query<Entity, With<UnderSwapTile>>
) {
  for (drag_direction, drag_neighbours) in drag_direction_q.iter()  {
    println!("Drag direction change: {:?}", drag_direction.direction);
    for entity in last_underswap_tile.iter_mut() {
      commands.entity(entity).insert(ResetTile);
    }
    let direction = drag_direction.direction;
    if drag_direction.direction.is_none() {
      continue;
    }
    let neighbour_entity = drag_neighbours.neighbours.get(&direction.unwrap());
    if neighbour_entity.is_none() {
      continue;
    }
    commands.entity(*neighbour_entity.unwrap()).insert(UnderSwapTile);
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
    commands.entity(entity)
      .remove::<ResetTile>()
      .remove::<SwapTile>()
      .remove::<UnderSwapTile>();
  }
}

pub fn reset_drag(
  mut drag_to_reset: Query<Entity, With<ResetDrag>>,
  mut commands: Commands
) {
  for entity in drag_to_reset.iter_mut() {
    commands.entity(entity)
      .remove::<ResetDrag>()
      .remove::<TileIsDragging>()
      .remove::<DragDirection>()
      .remove::<DragNeighbours>();
  }
}



#[derive(SystemSet, Clone, Hash, Debug, Eq, PartialEq)]
pub struct GameEvents;

#[derive(SystemSet, Clone, Hash, Debug, Eq, PartialEq)]
pub struct Cleanup;

pub fn add_drag_to_app(app:&mut App) -> &mut App {
  app
    .register_type::<TileIsDragging>()
    .add_event::<TileDragStart>()
    .add_event::<TileDragMove>()
    .add_event::<TileDragCancel>()
    .add_event::<TileDragEnd>()
    .add_event::<TileDragSuccess>()
    .configure_set(
        GameEvents
          .before(Cleanup)
      )
      .add_systems(
        (
          handle_tile_drag_start,
          handle_tile_drag_end,
          handle_tile_drag_move,
          handle_drag_direction_change
        ).in_set(GameEvents)
      ).add_systems((reset_tiles, reset_drag).in_set(Cleanup))
}

