use bevy::prelude::*;
//use bevy_rapier2d::render::RapierRenderPlugin;
use bevy_rapier2d::{
    na::Vector2,
    physics::{RapierConfiguration, RapierPhysicsPlugin},
};
use punchball::{
    arena::ArenaPlugin, event::EventPlugin, gamepad::GamepadPlugin, physics::PhysicsPlugin,
    player::PlayerPlugin, setup::SetupPlugin,
};

fn main() {
    App::build()
        // Engine stuff
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        //        .add_plugin(RapierRenderPlugin) // Only shows something if nothing renderable is on the entity
        .add_resource(RapierConfiguration {
            gravity: Vector2::zeros(),
            ..Default::default()
        })
        // Punchball stuff
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(SetupPlugin::default())
        .add_plugin(ArenaPlugin::default())
        .add_plugin(EventPlugin::default())
        .add_plugin(GamepadPlugin::default())
        .add_plugin(PlayerPlugin::default())
        .run();
}

trait Parent {}
trait Child: Parent {}
