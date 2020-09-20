use crate::{
    event::{EventListeners, PlayerSpawnEvent},
    gamepad::GamepadInputs,
};
use bevy::prelude::*;

//const MAX_PLAYERS: usize = 4;
const MOVE_SPEED: f32 = 150.0;

pub struct Player {
    pub id: usize,
}

pub fn player_controller(
    gamepad_inputs: Res<GamepadInputs>,
    time: Res<Time>,
    mut player_query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in &mut player_query.iter() {
        let input = gamepad_inputs.inputs.get(&player.id).unwrap();
        *transform.translation_mut().x_mut() +=
            input.left_stick.x() * time.delta_seconds * MOVE_SPEED;
        *transform.translation_mut().y_mut() +=
            input.left_stick.y() * time.delta_seconds * MOVE_SPEED;
    }
}

pub fn player_spawn(
    mut commands: Commands,
    mut listeners: ResMut<EventListeners>,
    player_spawn_events: Res<Events<PlayerSpawnEvent>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for player_spawn_event in listeners.player_spawn_reader.iter(&player_spawn_events) {
        println!("Player {} spawns", player_spawn_event.id);
        let texture_handle = asset_server.load("assets/circle.png").unwrap();
        commands
            .spawn(SpriteComponents {
                material: materials.add(texture_handle.into()),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..Default::default()
            })
            .with(Player {
                id: player_spawn_event.id,
            });
    }
}
