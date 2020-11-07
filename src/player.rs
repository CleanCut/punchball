use crate::{
    color_from_f32,
    event::{EventListeners, PlayerSpawnEvent},
    gamepad::GamepadInputs,
};
use bevy::prelude::*;

use bevy_lyon::{math, shapes, LyonMeshBuilder};
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
    pub facing: Vec2,
}

pub fn player_controller(
    gamepad_inputs: Res<GamepadInputs>,
    time: Res<Time>,
    mut rigid_body_set: ResMut<RigidBodySet>,
    //mut player_query: Query<(&Player, &mut Transform)>,
    mut physics_query: Query<(&mut Player, &RigidBodyHandleComponent, &mut Transform)>,
) {
    for (mut player, rigid_body_handle, mut transform) in physics_query.iter_mut() {
        let rigid_body_opt = rigid_body_set.get_mut(rigid_body_handle.handle());
        if let Some(mut rigid_body) = rigid_body_opt {
            let input = gamepad_inputs.inputs.get(&player.id).unwrap();
            // Move with left stick
            rigid_body.apply_force(Vector2::new(
                input.left_stick.x() * time.delta_seconds * MOVE_SPEED,
                0.0,
            ));
            rigid_body.apply_force(Vector2::new(
                0.0,
                input.left_stick.y() * time.delta_seconds * MOVE_SPEED,
            ));
            // Set direction of player with right stick
            let facing_vec = Vec2::new(input.right_stick.x(), input.right_stick.y());
            if facing_vec.length() > 0.1 {
                player.facing = facing_vec;
            }
            let quat = Quat::from_axis_angle(
                Vec3::new(0.0, 0.0, 1.0),
                angle_facing(&Vec2::new(0.0, 0.0), &player.facing),
            );
            transform.rotation = quat;
        }
    }
}

fn angle_facing(v1: &Vec2, v2: &Vec2) -> f32 {
    (v2.y() - v1.y()).atan2(v2.x() - v1.x())
}

pub fn player_spawn(
    mut commands: Commands,
    mut listeners: ResMut<EventListeners>,
    colors: Res<PlayerColors>,
    player_spawn_events: Res<Events<PlayerSpawnEvent>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for player_spawn_event in listeners.player_spawn_reader.iter(&player_spawn_events) {
        println!("Player {} spawns", player_spawn_event.id);
        let location_x = -200.0 + 100.0 * player_spawn_event.id as f32;
        let player_shape = meshes.add(
            LyonMeshBuilder::new()
                .with(shapes::FillCircle {
                    center: math::point(0.0, 0.0),
                    radius: COLLISION_RADIUS,
                    ..Default::default()
                })
                .with(shapes::StrokePolyline {
                    points: vec![
                        math::point(0.0, 0.0),
                        math::point(COLLISION_RADIUS * 2.5, 0.0),
                    ],
                    is_closed: false,
                    options: &lyon::tessellation::StrokeOptions::default().with_line_width(3.0),
                })
                .build(),
        );
        commands
            .spawn(SpriteComponents {
                mesh: player_shape,
                material: materials.add(colors.0[player_spawn_event.id].into()),
                sprite: Sprite::new(Vec2::new(1.0, 1.0)),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)),
                ..Default::default()
            })
            .with(Player {
                id: player_spawn_event.id,
                facing: Vec2::unit_x(),
            })
            .with(RigidBodyBuilder::new_dynamic().translation(location_x, 0.0))
            .with(ColliderBuilder::ball(COLLISION_RADIUS));
    }
}
