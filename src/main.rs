use bevy::prelude::*;
use bevy_rapier2d::render::RapierRenderPlugin;
use bevy_rapier2d::{
    na::Vector2,
    physics::{Gravity, RapierPhysicsPlugin},
};
use punchball::{
    event::EventPlugin, gamepad::GamepadPlugin, player::PlayerPlugin, setup::SetupPlugin,
};

fn main() {
    App::build()
        // Engine stuff
        .add_default_plugins()
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_resource(Gravity(Vector2::new(0.0, 0.0)))
        // Punchball stuff
        .add_plugin(SetupPlugin::default())
        .add_plugin(EventPlugin::default())
        .add_plugin(GamepadPlugin::default())
        .add_plugin(PlayerPlugin::default())
        .run();
}
