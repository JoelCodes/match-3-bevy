use bevy::prelude::*;

use super::{components::{GameData, Tile}};

#[derive(Bundle)]
pub struct GameBundle {
    pub game_data: GameData,
    pub name: Name,
    #[bundle]
    pub sprite: SpriteBundle
}

impl Default for GameBundle {
    fn default() -> Self {
        GameBundle {
            game_data: GameData {
                grid: vec![],
            },
            name: Name::new("Match3 Game"),
            sprite: SpriteBundle {
                ..Default::default()
            }
        }
    }
}

#[derive(Bundle)]
pub struct TileBundle {
    #[bundle]
    pub sprite: SpriteBundle,
    pub name: Name,
    pub tile: Tile,
}
