use bevy::{prelude::*};
pub struct Match3Plugin;

#[derive(Resource, Default, Clone, Copy, Debug)]
struct GameConfig {
    rows: usize,
    columns: usize,
    cell_size: f32,
}

enum TileType {
    Pentagon,
    Triangle,
    Square,
    Circle,
    Diamond,
    Star
}

fn setup(
    mut commands: Commands,
    game_config: Res<GameConfig>,
) {
    commands
        .spawn(Camera2dBundle::default());

    for row in 0..game_config.rows {
        let y = (row as f32 - game_config.rows as f32 / 2.) * game_config.cell_size;
        let r = (row as f32) / (game_config.rows as f32);
        for column in 0..game_config.columns {
            let x = (column as f32 - game_config.columns as f32 / 2.) * game_config.cell_size;
            let g = (column as f32) / (game_config.columns as f32);
            commands.spawn(SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                sprite: Sprite {
                    color: Color::rgba(r, g, 1.0, 1.0),
                    rect: Some(Rect {
                        min: Vec2::new(0.0, 0.0),
                        max: Vec2::new(game_config.cell_size, game_config.cell_size),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new(format!("Tile {} {}", row, column)));
        }
    }


}

impl Plugin for Match3Plugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameConfig>()
            .insert_resource(GameConfig {
                rows: 6,
                columns: 6,
                cell_size: 80.,
            })
            .add_startup_system(setup);
    }
}
