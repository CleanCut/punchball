use bevy::{prelude::*, window::WindowResolution};
//use bevy_rapier2d::render::RapierRenderPlugin;
use punchball::{
    arena::ArenaPlugin, event::EventPlugin, gamepad::GamepadPlugin, player::PlayerPlugin,
    points::PointsPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Punch Ball".to_string(),
                resolution: WindowResolution::new(1024.0, 1024.0),
                resizable: false,
                ..Default::default()
            }),
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
