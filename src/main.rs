use bevy::prelude::*;
#[cfg(debug)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod match3;
use crate::match3::*;

fn main(){
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window {
            title: "Match3".to_string(),
            resolution: (480., 480.).into(),
            ..Default::default()
        }),
        ..Default::default()
    }));
    #[cfg(debug)]
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_plugin(Match3Plugin)
        .run();
}
