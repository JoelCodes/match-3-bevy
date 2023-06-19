use bevy::{prelude::*, utils::HashMap};

use super::{resources::{GameConfig, GameData}, grid::{TileType, create_grid}, components::Tile};

pub fn setup(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    mut game_data: ResMut<GameData>,
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
    

    commands
        .spawn(Camera2dBundle::default());

    game_data.grid = create_grid(game_config.rows, game_config.columns);
    for column in 0..game_config.columns {
        for row in 0..game_config.rows {
            let tile_type = game_data.grid[column][row];
            let y = (row as f32 - game_config.rows as f32 / 2.) * game_config.cell_size;
            let x = (column as f32 - game_config.columns as f32 / 2.) * game_config.cell_size;
            commands.spawn(SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: tile_textures.get(&tile_type.unwrap()).unwrap().clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(game_config.cell_size, game_config.cell_size)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Tile { tile_type: tile_type.unwrap(), column, row, })
            .insert(Name::new(format!("Tile {} {}", row, column)));
        }
    }
    game_data.is_ready = true;
}
