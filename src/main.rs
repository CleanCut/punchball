use bevy::prelude::*;
//use bevy_rapier2d::render::RapierRenderPlugin;
use punchball::{
    arena::ArenaPlugin, event::EventPlugin, gamepad::GamepadPlugin, player::PlayerPlugin,
    setup::SetupPlugin,
};

fn main() {
    App::build()
        .add_plugin(SetupPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(ArenaPlugin::default())
        .add_plugin(EventPlugin::default())
        .add_plugin(GamepadPlugin::default())
        .add_plugin(PlayerPlugin::default())
        .run();
}

trait Parent {}
trait Child: Parent {}
