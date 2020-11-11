use crate::{
    event::{EventListeners, PlayerSpawnEvent},
    gamepad::GamepadInputs,
};
use bevy::prelude::*;

//const MAX_PLAYERS: usize = 4;
const MOVE_SPEED: f32 = 25.0;
const MAX_VELOCITY: f32 = 4.0;
const COLLISION_RADIUS: f32 = 32.0; // Needs to match the size of the sprite

#[derive(Default)]
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(PlayerColors::default())
            .add_system(player_spawn.system())
            .add_system(player_physics.system())
            .add_system(player_controller.system());
    }
}

// fn player_setup()

pub struct PlayerColors(Vec<Color>);
impl Default for PlayerColors {
    fn default() -> Self {
        Self(vec![
            Color::rgb(0.235, 0.349, 0.494), // blue
            Color::rgb(0.976, 0.761, 0.416), // gold
            Color::rgb(0.377, 0.565, 0.537), // green
            Color::rgb(0.592, 0.337, 0.157), // brown
        ])
    }
}
pub struct Player {
    pub id: usize,
    pub facing: Vec2,
    pub vel: Vec2,
}

pub fn player_controller(
    gamepad_inputs: Res<GamepadInputs>,
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
) {
    for (mut player, mut transform) in player_query.iter_mut() {
        let input = gamepad_inputs.inputs.get(&player.id).unwrap();
        // Move with left stick
        const DEAD_ZONE_THRESHOLD: f32 = 0.1;
        let left_x = input.left_stick.x();
        let left_y = input.left_stick.y();
        if Vec2::new(left_x, left_y).length() > DEAD_ZONE_THRESHOLD {
            *player.vel.x_mut() += left_x * time.delta_seconds * MOVE_SPEED;
            *player.vel.y_mut() += left_y * time.delta_seconds * MOVE_SPEED;
        }
        if player.vel.length() > MAX_VELOCITY {
            player.vel = player.vel.normalize() * MAX_VELOCITY;
        }
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

    fn angle_facing(v1: &Vec2, v2: &Vec2) -> f32 {
        (v2.y() - v1.y()).atan2(v2.x() - v1.x())
    }
}

pub fn player_physics(time: Res<Time>, mut player_query: Query<(&Player, &mut Transform)>) {
    for (player, mut transform) in player_query.iter_mut() {
        *transform.translation.x_mut() += player.vel.x() * time.delta_seconds * MOVE_SPEED;
        *transform.translation.y_mut() += player.vel.y() * time.delta_seconds * MOVE_SPEED;
    }
}

pub fn player_spawn(
    commands: &mut Commands,
    mut listeners: ResMut<EventListeners>,
    colors: Res<PlayerColors>,
    player_spawn_events: Res<Events<PlayerSpawnEvent>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for player_spawn_event in listeners.player_spawn_reader.iter(&player_spawn_events) {
        println!("Player {} spawns", player_spawn_event.id);
        let circle_texture = asset_server.load("circle.png");
        let circle_material = ColorMaterial {
            color: colors.0[player_spawn_event.id],
            texture: circle_texture.into(),
        };
        let circle_handle = materials.add(circle_material);
        let starting_locations = vec![
            Vec3::new(-100.0, 100.0, 0.0),
            Vec3::new(100.0, 100.0, 0.0),
            Vec3::new(100.0, -100.0, 0.0),
            Vec3::new(-100.0, -100.0, 0.0),
        ];
        commands
            .spawn(SpriteComponents {
                material: circle_handle.clone(),
                transform: Transform::from_translation(starting_locations[player_spawn_event.id]),
                ..Default::default()
            })
            .with(Player {
                id: player_spawn_event.id,
                facing: Vec2::unit_x(),
                vel: Vec2::zero(),
            })
            .with_children(|parent| {
                // Punching Glove
                parent.spawn(SpriteComponents {
                    material: circle_handle,
                    transform: Transform {
                        translation: Vec3::new(40.0, 0.0, 0.1),
                        scale: Vec3::splat(0.3),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
    }
}
