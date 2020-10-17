use crate::event::PlayerSpawnEvent;
use bevy::input::gamepad::{Gamepad, GamepadButton, GamepadEvent, GamepadEventType};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct GamepadPlugin;
impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GamepadManager::default())
            .add_resource(GamepadInputs::default())
            .add_system(axis_system.system())
            .add_startup_system(connection_system.system())
            .add_system(connection_system.system())
            .add_system(button_system.system());
    }
}

#[derive(Default)]
pub struct GamepadInputs {
    pub inputs: HashMap<usize, GamepadInput>,
}

/// Cached gamepad input, updated once per frame
#[derive(Default)]
pub struct GamepadInput {
    pub left_stick: Vec2,
    pub right_stick: Vec2,
}

#[derive(Default)]
pub struct GamepadManager {
    gamepad: HashSet<Gamepad>,
    gamepad_event_reader: EventReader<GamepadEvent>,
}

pub fn connection_system(
    mut gamepad_manager: ResMut<GamepadManager>,
    gamepad_event: Res<Events<GamepadEvent>>,
    mut player_spawn_channel: ResMut<Events<PlayerSpawnEvent>>,
) {
    for GamepadEvent(gamepad, gamepad_event_type) in
        gamepad_manager.gamepad_event_reader.iter(&gamepad_event)
    {
        match gamepad_event_type {
            GamepadEventType::Connected => {
                gamepad_manager.gamepad.insert(*gamepad);
                println!("Connected {:?}", gamepad);
                player_spawn_channel.send(PlayerSpawnEvent { id: gamepad.0 });
            }
            GamepadEventType::Disconnected => {
                gamepad_manager.gamepad.remove(&gamepad);
                // TODO: Remove player entity
                //commands.despawn(entity)
                println!("Disconnected {:?}", gamepad);
            }
        }
    }
}

pub fn button_system(manager: Res<GamepadManager>, inputs: Res<Input<GamepadButton>>) {
    let button_codes = [
        GamepadButtonType::South,
        GamepadButtonType::East,
        GamepadButtonType::North,
        GamepadButtonType::West,
        GamepadButtonType::C,
        GamepadButtonType::Z,
        GamepadButtonType::LeftTrigger,
        GamepadButtonType::LeftTrigger2,
        GamepadButtonType::RightTrigger,
        GamepadButtonType::RightTrigger2,
        GamepadButtonType::Select,
        GamepadButtonType::Start,
        GamepadButtonType::Mode,
        GamepadButtonType::LeftThumb,
        GamepadButtonType::RightThumb,
        GamepadButtonType::DPadUp,
        GamepadButtonType::DPadDown,
        GamepadButtonType::DPadLeft,
        GamepadButtonType::DPadRight,
    ];
    for gamepad in manager.gamepad.iter() {
        for button_code in button_codes.iter() {
            if inputs.just_pressed(GamepadButton(*gamepad, *button_code)) {
                println!("Pressed {:?}", GamepadButton(*gamepad, *button_code));
            } else if inputs.just_released(GamepadButton(*gamepad, *button_code)) {
                println!("Released {:?}", GamepadButton(*gamepad, *button_code));
            }
        }
    }
}

pub fn axis_system(
    gamepad_manager: Res<GamepadManager>,
    axes: Res<Axis<GamepadAxis>>,
    mut gamepad_inputs: ResMut<GamepadInputs>,
) {
    let axis_codes = [
        GamepadAxisType::LeftStickX,
        GamepadAxisType::LeftStickY,
        // GamepadAxisType::LeftZ,
        GamepadAxisType::RightStickX,
        GamepadAxisType::RightStickY,
        // GamepadAxisType::RightZ,
        // GamepadAxisType::DPadX,
        // GamepadAxisType::DPadY,
    ];
    for gamepad in gamepad_manager.gamepad.iter() {
        for axis_code in axis_codes.iter() {
            if let Some(value) = axes.get(&GamepadAxis(*gamepad, *axis_code)) {
                let gamepad_input = gamepad_inputs.inputs.entry(gamepad.0).or_default();
                match axis_code {
                    GamepadAxisType::LeftStickX => gamepad_input.left_stick.set_x(value),
                    GamepadAxisType::LeftStickY => gamepad_input.left_stick.set_y(value),
                    GamepadAxisType::RightStickX => gamepad_input.right_stick.set_x(value),
                    GamepadAxisType::RightStickY => gamepad_input.right_stick.set_y(value),
                    _ => {}
                }
            }
        }
    }
}
