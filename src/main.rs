use bevy::gilrs::GilrsPlugin;
use bevy::prelude::*;
use bevy::render::pass::ClearColor;

use punchball::{
    event::{EventListeners, PlayerSpawnEvent},
    gamepad::{axis_system, button_system, connection_system, GamepadInputs, GamepadManager},
    player::{player_controller, player_spawn},
};

fn main() {
    App::build()
        // General stuff
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        // Event stuff
        .init_resource::<EventListeners>()
        .add_event::<PlayerSpawnEvent>()
        // Gamepad stuff
        .add_plugin(GilrsPlugin::default()) // enable gamepad support in bevy
        .add_resource(GamepadManager::default())
        .add_resource(GamepadInputs::default())
        .add_system(axis_system.system())
        .add_startup_system(connection_system.system())
        .add_system(connection_system.system())
        .add_system(button_system.system())
        // -- Punchball player
        .add_system(player_spawn.system())
        .add_system(player_controller.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}
