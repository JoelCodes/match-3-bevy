use bevy::{prelude::*, window::PrimaryWindow, utils::HashMap, sprite::Anchor};

use super::{resources::{GameConfig, GameData}, grid::*, bundles::*, components::*};

pub fn setup_camera(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
){
    let primary_window = window.get_single().unwrap();
    commands
      .spawn(Camera2dBundle{
        transform: Transform::from_xyz(primary_window.width() / 2., primary_window.height() / 2., 100.0),
        ..Default::default()
      });

}

pub fn setup_grid(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    asset_server: Res<AssetServer>,
  ) {
      
    let triangle_handle = asset_server.load("triangle.png");
    let pentagon_handle = asset_server.load("pentagon.png");
    let square_handle = asset_server.load("square.png");
    let circle_handle = asset_server.load("circle.png");
    let diamond_handle = asset_server.load("diamond.png");
    let star_handle = asset_server.load("star.png");
  
    let tile_textures = vec![
      (TileType::Pentagon, pentagon_handle),
      (TileType::Triangle, triangle_handle),
      (TileType::Square, square_handle),
      (TileType::Circle, circle_handle),
      (TileType::Diamond, diamond_handle),
      (TileType::Star, star_handle),
    ].into_iter().collect::<HashMap<TileType,Handle<Image>>>();
    
    let grid = create_grid(game_config.rows.into(), game_config.columns.into());
    commands.spawn(GameBundle{
      game_data: GameData {
        grid: grid.clone(),
        ..Default::default()
      },
      sprite: SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        visibility: Visibility::Visible,
        sprite: Sprite {
          rect: Some(Rect {
            min: Vec2::new(0.0, 0.0),
            max: Vec2::new(game_config.cell_size * game_config.columns as f32, game_config.cell_size * game_config.rows as f32),
          }),
          anchor: Anchor::BottomLeft,
          ..Default::default()
        },
        ..Default::default()
      },
      ..Default::default()
    })
      .with_children(|commands|{
        let custom_size = Some(Vec2::new(game_config.cell_size, game_config.cell_size));
        for column in 0..game_config.columns {
          for row in 0..game_config.rows {
            if let Some(tile_type) = grid[column][row] {
              let y = (row as f32) * game_config.cell_size;
              let x = (column as f32) * game_config.cell_size;
              commands.spawn(TileBundle {
                sprite: SpriteBundle {
                  transform: Transform::from_xyz(x, y, 0.0),
                  texture: tile_textures.get(&tile_type).unwrap().clone(),
                  sprite: Sprite {
                      custom_size,
                      anchor: Anchor::BottomLeft,
                      ..Default::default()
                  },
                  ..Default::default()
                },
                tile: Tile { tile_type, column, row, },
                name: Name::new(format!("Tile {} {}", column, row)),
              });
            }
          }
        }  
      });
  }
  