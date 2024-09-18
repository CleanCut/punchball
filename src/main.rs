use bevy::prelude::*;
//use bevy_rapier2d::render::RapierRenderPlugin;
use punchball::{
    arena::ArenaPlugin, event::EventPlugin, gamepad::GamepadPlugin, player::PlayerPlugin,
    points::PointsPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Punch Ball".to_string(),
                width: 1024.0,
                height: 1024.0,
                resizable: false,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(ArenaPlugin::default())
        .add_plugin(EventPlugin::default())
        .add_plugin(GamepadPlugin::default())
        .add_plugin(PlayerPlugin::default())
        .add_plugin(PointsPlugin::default())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
