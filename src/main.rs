use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod match3;
use crate::match3::*;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: "Match3".to_string(),
                resolution: (480., 480.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(Match3Plugin)
        .run();
}
