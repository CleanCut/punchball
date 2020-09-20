use bevy::prelude::*;
use punchball::{
    event::EventPlugin, gamepad::GamepadPlugin, player::PlayerPlugin, setup::SetupPlugin,
};

fn main() {
    App::build()
        // Engine stuff
        .add_default_plugins()
        // Punchball stuff
        .add_plugin(SetupPlugin::default())
        .add_plugin(EventPlugin::default())
        .add_plugin(GamepadPlugin::default())
        .add_plugin(PlayerPlugin::default())
        .run();
}
