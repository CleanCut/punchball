use crate::{
    color_from_f32,
    event::{EventListeners, PlayerSpawnEvent},
    gamepad::GamepadInputs,
};
use bevy::prelude::*;

use bevy_rapier2d::{
    na::Vector2,
    rapier::{dynamics::RigidBodySet, geometry::ColliderBuilder},
};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodyBuilder};

//const MAX_PLAYERS: usize = 4;
const MOVE_SPEED: f32 = 150000000.0;
const COLLISION_RADIUS: f32 = 32.0; // Needs to match the size of the sprite

#[derive(Default)]
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(PlayerColors::default())
            .add_system(player_spawn.system())
            .add_system(player_controller.system());
    }
}

// fn player_setup()

pub struct PlayerColors(Vec<Color>);
impl Default for PlayerColors {
    fn default() -> Self {
        Self(vec![
            color_from_f32(0.235, 0.349, 0.494), // blue
            color_from_f32(0.976, 0.761, 0.416), // gold
            color_from_f32(0.377, 0.565, 0.537), // green
            color_from_f32(0.592, 0.337, 0.157), // brown
        ])
    }
}
pub struct Player {
    pub id: usize,
}

pub fn player_controller(
    gamepad_inputs: Res<GamepadInputs>,
    time: Res<Time>,
    mut rigid_body_set: ResMut<RigidBodySet>,
    //mut player_query: Query<(&Player, &mut Transform)>,
    mut physics_query: Query<(&Player, &RigidBodyHandleComponent)>,
) {
    // for (player, mut transform) in &mut player_query.iter() {
    // let input = gamepad_inputs.inputs.get(&player.id).unwrap();
    // *transform.translation_mut().x_mut() +=
    // input.left_stick.x() * time.delta_seconds * MOVE_SPEED;
    // *transform.translation_mut().y_mut() +=
    // input.left_stick.y() * time.delta_seconds * MOVE_SPEED;
    // }

    for (player, rigid_body_handle) in &mut physics_query.iter() {
        let rigid_body_opt = rigid_body_set.get_mut(rigid_body_handle.handle());
        if let Some(mut rigid_body) = rigid_body_opt {
            let input = gamepad_inputs.inputs.get(&player.id).unwrap();
            rigid_body.apply_force(Vector2::new(
                input.left_stick.x() * time.delta_seconds * MOVE_SPEED,
                0.0,
            ));
            rigid_body.apply_force(Vector2::new(
                0.0,
                input.left_stick.y() * time.delta_seconds * MOVE_SPEED,
            ));
        }
    }
}

pub fn player_spawn(
    mut commands: Commands,
    mut listeners: ResMut<EventListeners>,
    colors: Res<PlayerColors>,
    player_spawn_events: Res<Events<PlayerSpawnEvent>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for player_spawn_event in listeners.player_spawn_reader.iter(&player_spawn_events) {
        println!("Player {} spawns", player_spawn_event.id);
        let texture_handle = asset_server.load("assets/circle.png").unwrap();
        let color_material = ColorMaterial {
            color: colors.0[player_spawn_event.id],
            texture: texture_handle.into(),
        };
        commands
            .spawn(SpriteComponents {
                material: materials.add(color_material),
                transform: Transform::from_translation(Vec3::new(
                    -200.0 + 100.0 * player_spawn_event.id as f32,
                    0.0,
                    0.0,
                )),
                ..Default::default()
            })
            .with(Player {
                id: player_spawn_event.id,
            })
            .with(
                RigidBodyBuilder::new_dynamic()
                    .translation(-200.0 + 100.0 * player_spawn_event.id as f32, 0.0),
            )
            .with(ColliderBuilder::ball(COLLISION_RADIUS));
    }
}
