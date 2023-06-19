use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod match3;
use crate::match3::*;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(Match3Plugin)
        .run();
}
