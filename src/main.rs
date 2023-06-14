use bevy::prelude::*;
mod match3;
use crate::match3::Match3Plugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(Match3Plugin)
        .run();
}