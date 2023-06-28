use bevy::{prelude::*, input::{mouse::{MouseButtonInput, MouseMotion}, ButtonState}, window::PrimaryWindow};

use super::{components::*, drag::*};

pub fn handle_mousebtn(
  mut mouse_btn_evr: EventReader<MouseButtonInput>,
  windows: Query<&Window, With<PrimaryWindow>>,
  mut tile_drag_start_evw: EventWriter<TileDragStart>,
  mut tile_drag_cancel_evw: EventWriter<TileDragCancel>
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
        tile_drag_cancel_evw.send(TileDragCancel{});
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