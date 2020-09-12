use bevy::prelude::*;
use bevy_gilrs::GilrsPlugin;
use bevy_input::gamepad::{Gamepad, GamepadButton, GamepadEvent, GamepadEventType};
use std::collections::HashSet;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(GilrsPlugin::default())
        .add_startup_system(connection_system.system())
        .add_system(connection_system.system())
        .add_system(button_system.system())
        .add_system(axis_system.system())
        .add_resource(Lobby::default())
        .run();
}

#[derive(Default)]
struct Lobby {
    gamepad: HashSet<Gamepad>,
    gamepad_event_reader: EventReader<GamepadEvent>,
}

fn connection_system(mut lobby: ResMut<Lobby>, gamepad_event: Res<Events<GamepadEvent>>) {
    for event in lobby.gamepad_event_reader.iter(&gamepad_event) {
        match event.event_type {
            GamepadEventType::Connected => {
                lobby.gamepad.insert(event.gamepad);
                println!("Connected {:?}", event.gamepad);
            }
            GamepadEventType::Disconnected => {
                lobby.gamepad.remove(&event.gamepad);
                println!("Disconnected {:?}", event.gamepad);
            }
        }
    }
}

fn button_system(manager: Res<Lobby>, inputs: Res<Input<GamepadButton>>) {
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

fn axis_system(manager: Res<Lobby>, axes: Res<Axis<GamepadAxis>>) {
    let axis_codes = [
        AxisCode::LeftStickX,
        AxisCode::LeftStickY,
        AxisCode::LeftZ,
        AxisCode::RightStickX,
        AxisCode::RightStickY,
        AxisCode::RightZ,
        AxisCode::DPadX,
        AxisCode::DPadY,
    ];
    for gamepad in manager.gamepad.iter() {
        for axis_code in axis_codes.iter() {
            if let Some(value) = axes.get(&GamepadAxis::new(*gamepad, *axis_code)) {
                if value.abs() > 0.01f32
                    && (value - 1.0f32).abs() > 0.01f32
                    && (value + 1.0f32).abs() > 0.01f32
                {
                    println!(
                        "Axis {:?} is {}",
                        GamepadAxis::new(*gamepad, *axis_code),
                        value
                    );
                }
            }
        }
    }
}
