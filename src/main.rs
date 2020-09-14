use bevy::gilrs::GilrsPlugin;
use bevy::input::gamepad::{Gamepad, GamepadButton, GamepadEvent, GamepadEventType};
use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use std::collections::HashSet;

fn main() {
    App::build()
        .add_default_plugins()
        .add_event::<PlayerMoveEvent>()
        .init_resource::<State>()
        .add_resource(Lobby::default())
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugin(GilrsPlugin::default()) // under-the-hood gamepad stuff
        .add_startup_system(setup.system())
        .add_system(axis_system.system())
        .add_startup_system(connection_system.system())
        .add_system(connection_system.system())
        .add_system(button_system.system())
        .add_system(event_consumer.system())
        .run();
}

#[derive(Default)]
struct Lobby {
    gamepad: HashSet<Gamepad>,
    gamepad_event_reader: EventReader<GamepadEvent>,
}

const MAX_PLAYERS: usize = 4;

struct Player {
    id: usize,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}

fn connection_system(
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    gamepad_event: Res<Events<GamepadEvent>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in lobby.gamepad_event_reader.iter(&gamepad_event) {
        match event.event_type {
            GamepadEventType::Connected => {
                if lobby.gamepad.len() < MAX_PLAYERS {
                    lobby.gamepad.insert(event.gamepad);
                    println!("Connected {:?}", event.gamepad);
                    let texture_handle = asset_server.load("assets/circle.png").unwrap();
                    commands.spawn(Camera2dComponents::default());
                    commands
                        .spawn(SpriteComponents {
                            material: materials.add(texture_handle.into()),
                            translation: Translation::new(0.0, 0.0, 0.0),
                            ..Default::default()
                        })
                        .with(Player {
                            id: event.gamepad.id,
                        });
                    println!("done spawning!");
                } else {
                    println!(
                        "Not allowing {:?} to connect to the game! Already at max players of {}",
                        event.gamepad, MAX_PLAYERS
                    );
                }
            }
            GamepadEventType::Disconnected => {
                lobby.gamepad.remove(&event.gamepad);
                // TODO: Remove player entity
                //commands.despawn(entity)
                println!("Disconnected {:?}", event.gamepad);
            }
        }
    }
}

//fn player_control(player: &Player, position: &Position) {
//    println!("Player is {}", player.id);
//}

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

struct PlayerMoveEvent {
    axis: AxisCode,
    player_id: usize,
    value: f32,
}

#[derive(Default)]
struct State {
    reader: EventReader<PlayerMoveEvent>,
}

fn event_consumer(
    mut state: ResMut<State>,
    player_move_events: Res<Events<PlayerMoveEvent>>,
    mut query: Query<(&Player, &mut Translation)>, // mut sprite_components: Mut<SpriteComponents>,
) {
    for event in state.reader.iter(&player_move_events) {
        for (player, mut translation) in &mut query.iter() {
            if player.id == event.player_id {
                println!("---> {:?} {}", event.axis, event.value);
                match event.axis {
                    AxisCode::LeftStickX => {
                        let old_x = dbg!(translation.x());
                        translation.set_x(old_x + event.value)
                    }
                    AxisCode::LeftStickY => {
                        let old_y = translation.y();
                        translation.set_y(old_y + event.value)
                    }
                    _ => {
                        println!("argumundo");
                    }
                }
            }
        }
    }
}

fn axis_system(
    manager: Res<Lobby>,
    axes: Res<Axis<GamepadAxis>>,
    mut player_move_event_channel: ResMut<Events<PlayerMoveEvent>>,
) {
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
                    // println!(
                    //     "Axis {:?} is {}",
                    //     GamepadAxis::new(*gamepad, *axis_code),
                    //     value
                    // );
                    player_move_event_channel.send(PlayerMoveEvent {
                        axis: *axis_code,
                        player_id: gamepad.id,
                        value,
                    });
                }
            }
        }
    }
}
