use crate::event::PlayerSpawnEvent;
use bevy::prelude::*;
use bevy::{
    app::AppExit,
    input::gamepad::{Gamepad, GamepadButton, GamepadEvent},
};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct GamepadPlugin;
impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GamepadManager::default())
            .insert_resource(GamepadInputs::default())
            .add_system(axis_system)
            .add_system(button_system)
            .add_startup_system(connection_system)
            .add_system(connection_system)
            .add_system(keyboard_quit_system);
    }
}

#[derive(Default, Resource)]
pub struct GamepadInputs {
    pub inputs: HashMap<usize, GamepadInput>,
}

/// Cached gamepad input, updated once per frame
#[derive(Default)]
pub struct GamepadInput {
    pub left_stick: Vec2,
    pub right_stick: Vec2,
    pub right_trigger2: bool,
}

#[derive(Default, Resource)]
pub struct GamepadManager {
    gamepad: HashSet<Gamepad>,
}

pub fn connection_system(
    mut gamepad_manager: ResMut<GamepadManager>,
    mut gamepad_events: EventReader<GamepadEvent>,
    mut player_spawn_channel: EventWriter<PlayerSpawnEvent>,
) {
    for event in gamepad_events.iter() {
        match event {
            GamepadEvent::Connection(connection_event) => {
                if connection_event.connected() {
                    gamepad_manager.gamepad.insert(connection_event.gamepad);
                    //println!("Connected {:?}", gamepad);
                    player_spawn_channel.send(PlayerSpawnEvent {
                        id: connection_event.gamepad.id,
                    });
                } else {
                    gamepad_manager.gamepad.remove(&connection_event.gamepad);
                    // TODO: Remove player entity
                    //commands.despawn(entity)
                    //println!("Disconnected {:?}", gamepad);
                }
            }
            _ => {}
        }
    }
}

/// Let people quit via the keyboard
fn keyboard_quit_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        println!("Thank you for playing!");
        app_exit_events.send(AppExit);
    }
}

pub fn button_system(
    manager: Res<GamepadManager>,
    inputs: Res<Input<GamepadButton>>,
    mut gamepad_inputs: ResMut<GamepadInputs>,
    mut app_exit_events: EventWriter<AppExit>,
) {
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
    // Reset input values
    for gamepad in manager.gamepad.iter() {
        let gamepad_input = gamepad_inputs.inputs.entry(gamepad.id).or_default();
        let mut right_trigger2 = false;
        for button_code in button_codes.iter() {
            if inputs.pressed(GamepadButton::new(*gamepad, *button_code)) {
                match button_code {
                    GamepadButtonType::RightTrigger2 => right_trigger2 = true,
                    GamepadButtonType::Start => {
                        println!("Thank you for playing!");
                        app_exit_events.send(AppExit);
                    }
                    _ => {}
                }
            }
            // if inputs.just_pressed(GamepadButton(*gamepad, *button_code)) {
            //     println!("Pressed {:?}", GamepadButton(*gamepad, *button_code));
            // } else if inputs.just_released(GamepadButton(*gamepad, *button_code)) {
            //     println!("Released {:?}", GamepadButton(*gamepad, *button_code));
            // }
        }
        gamepad_input.right_trigger2 = right_trigger2;
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
            if let Some(value) = axes.get(GamepadAxis::new(*gamepad, *axis_code)) {
                let gamepad_input = gamepad_inputs.inputs.entry(gamepad.id).or_default();
                match axis_code {
                    GamepadAxisType::LeftStickX => gamepad_input.left_stick.x = value,
                    GamepadAxisType::LeftStickY => gamepad_input.left_stick.y = value,
                    GamepadAxisType::RightStickX => gamepad_input.right_stick.x = value,
                    GamepadAxisType::RightStickY => gamepad_input.right_stick.y = value,
                    _ => {}
                }
            }
        }
    }
}
