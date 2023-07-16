use bevy::{prelude::*, input::{mouse::{MouseButtonInput, MouseMotion}, ButtonState}, window::PrimaryWindow};

use super::{drag::*, resources::*};

#[derive(SystemSet, Clone, Hash, Debug, Eq, PartialEq)]
pub struct MouseInput;

pub fn handle_mousebtn(
  mut mouse_btn_evr: EventReader<MouseButtonInput>,
  windows: Query<&Window, With<PrimaryWindow>>,
  mut tile_drag_start_evw: EventWriter<TileDragStart>,
  mut tile_drag_end_evw: EventWriter<TileDragEnd>
) {
  for evt in mouse_btn_evr.iter() {
    match (evt.button, evt.state) {
      (MouseButton::Left, ButtonState::Pressed) => {
        for window in windows.iter() {
          if let Some(position) = window.cursor_position() {
            tile_drag_start_evw.send(TileDragStart {
              start_coord: position.into(),
              column: (position.x / 80.) as usize,
              row: (position.y / 80.) as usize,
            });
          }
        }
      }
      (MouseButton::Left, ButtonState::Released) => {
        tile_drag_end_evw.send(TileDragEnd{});
      }
      _ => {}
    }
  }
}

pub fn handle_mousemove(
  game_data: Query<&GameData, With<TileIsDragging>>,
  mut mouse_moves_evr: EventReader<MouseMotion>,
  mut tile_drag_move_evw: EventWriter<TileDragMove>
) {
  if game_data.is_empty() {
    return;
  }

  for event in mouse_moves_evr.iter() {
    tile_drag_move_evw.send(TileDragMove {
      delta_coord: event.delta * Vec2 { x: 1., y: -1. },
    });
  }
}

pub fn add_input_to_app(
  app: &mut App
) -> &mut App {
  app
    .configure_set(MouseInput.before(GameEvents))
    .add_systems((handle_mousebtn, handle_mousemove).in_set(MouseInput))
}