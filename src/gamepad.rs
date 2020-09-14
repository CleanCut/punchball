use crate::event::PlayerSpawnEvent;
use bevy::input::gamepad::{Gamepad, GamepadButton, GamepadEvent, GamepadEventType};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct GamepadInputs {
    pub inputs: HashMap<usize, GamepadInput>,
}

/// Cached gamepad input, updated once per frame
#[derive(Default)]
pub struct GamepadInput {
    pub left_stick: Vec2,
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
    for event in gamepad_manager.gamepad_event_reader.iter(&gamepad_event) {
        match event.event_type {
            GamepadEventType::Connected => {
                gamepad_manager.gamepad.insert(event.gamepad);
                println!("Connected {:?}", event.gamepad);
                player_spawn_channel.send(PlayerSpawnEvent {
                    id: event.gamepad.id,
                });
            }
            GamepadEventType::Disconnected => {
                gamepad_manager.gamepad.remove(&event.gamepad);
                // TODO: Remove player entity
                //commands.despawn(entity)
                println!("Disconnected {:?}", event.gamepad);
            }
        }
    }
}

pub fn button_system(manager: Res<GamepadManager>, inputs: Res<Input<GamepadButton>>) {
    let button_codes = [
        ButtonCode::South,
        ButtonCode::East,
        ButtonCode::North,
        ButtonCode::West,
        ButtonCode::C,
        ButtonCode::Z,
        ButtonCode::LeftTrigger,
        ButtonCode::LeftTrigger2,
        ButtonCode::RightTrigger,
        ButtonCode::RightTrigger2,
        ButtonCode::Select,
        ButtonCode::Start,
        ButtonCode::Mode,
        ButtonCode::LeftThumb,
        ButtonCode::RightThumb,
        ButtonCode::DPadUp,
        ButtonCode::DPadDown,
        ButtonCode::DPadLeft,
        ButtonCode::DPadRight,
    ];
    for gamepad in manager.gamepad.iter() {
        for button_code in button_codes.iter() {
            if inputs.just_pressed(GamepadButton::new(*gamepad, *button_code)) {
                println!("Pressed {:?}", GamepadButton::new(*gamepad, *button_code));
            } else if inputs.just_released(GamepadButton::new(*gamepad, *button_code)) {
                println!("Released {:?}", GamepadButton::new(*gamepad, *button_code));
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
        AxisCode::LeftStickX,
        AxisCode::LeftStickY,
        // AxisCode::LeftZ,
        // AxisCode::RightStickX,
        // AxisCode::RightStickY,
        // AxisCode::RightZ,
        // AxisCode::DPadX,
        // AxisCode::DPadY,
    ];
    for gamepad in gamepad_manager.gamepad.iter() {
        for axis_code in axis_codes.iter() {
            if let Some(value) = axes.get(&GamepadAxis::new(*gamepad, *axis_code)) {
                let gamepad_input = gamepad_inputs.inputs.entry(gamepad.id).or_default();
                match axis_code {
                    AxisCode::LeftStickX => gamepad_input.left_stick.set_x(value),
                    AxisCode::LeftStickY => gamepad_input.left_stick.set_y(value),
                    _ => {}
                }
            }
        }
    }
}
