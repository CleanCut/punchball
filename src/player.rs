use crate::{
    arena::{Arena, ARENA_RADIUS},
    event::{EventListeners, PlayerSpawnEvent},
    gamepad::GamepadInputs,
};
use bevy::{
    prelude::*,
    utils::{AHashExt, HashMap},
};

//const MAX_PLAYERS: usize = 4;
const STARTING_LOCATIONS: [[f32; 3]; 4] = [
    [-100.0, 100.0, 0.0],
    [100.0, 100.0, 0.0],
    [100.0, -100.0, 0.0],
    [-100.0, -100.0, 0.0],
];

#[derive(Default)]
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(PlayerColors::default())
            .add_system(dead_players_system.system())
            .add_system(leave_arena_system.system())
            .add_system(player_join_system.system())
            .add_system(player_physics.system());
    }
}

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
    pub respawn_timer: Timer,
}

impl Player {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            facing: Vec2::unit_x(),
            vel: Vec2::zero(),
            respawn_timer: Timer::from_seconds(2.0, false),
        }
    }
}
/// Marks that a player is dead
pub struct Dead {}

/// Detect a player leaving the arena, and add a Dead component to him.
fn leave_arena_system(
    commands: &mut Commands,
    mut player_transforms: Query<(Entity, &Children, &Transform, &Player), Without<Dead>>,
    arena_transforms: Query<&Transform, With<Arena>>,
    mut draw_query: Query<&mut Draw>,
) {
    for arena_transform in arena_transforms.iter() {
        for (entity, children, player_transform, player) in player_transforms.iter_mut() {
            if (player_transform.translation - arena_transform.translation).length() > ARENA_RADIUS
            {
                println!("Player {} dies", player.id);
                commands.insert_one(entity, Dead {});
                if let Ok(mut draw) = draw_query.get_mut(entity) {
                    draw.is_visible = false;
                }
                // And any components of the player, like the glove
                for child in children.iter() {
                    if let Ok(mut draw) = draw_query.get_mut(*child) {
                        draw.is_visible = false;
                    }
                }
            }
        }
    }
}

/// Handle players that are dead, eventually respawning them again
pub fn dead_players_system(
    commands: &mut Commands,
    time: Res<Time>,
    mut players: Query<(Entity, &Children, &mut Player, &mut Transform), With<Dead>>,
    mut draw_query: Query<&mut Draw>,
) {
    for (entity, children, mut player, mut transform) in players.iter_mut() {
        // Decrement the timer for how long the player has left to be dead
        player.respawn_timer.tick(time.delta_seconds);
        // Is the player done being dead?
        if player.respawn_timer.finished {
            // Restart the timer for next time
            player.respawn_timer.reset();
            // Reset velocity
            player.vel = Vec2::zero();
            // Spawn at the starting location
            transform.translation = STARTING_LOCATIONS[player.id].into();
            // Remove the "Dead" component
            commands.remove_one::<Dead>(entity);
            // Make the player visible again
            if let Ok(mut draw) = draw_query.get_mut(entity) {
                draw.is_visible = true;
            }
            // And any components of the player, like the glove
            for child in children.iter() {
                if let Ok(mut draw) = draw_query.get_mut(*child) {
                    draw.is_visible = true;
                }
            }
        }
    }
}

fn angle_facing(v1: &Vec2, v2: &Vec2) -> f32 {
    (v2.y() - v1.y()).atan2(v2.x() - v1.x())
}

const DEAD_ZONE_THRESHOLD: f32 = 0.2;
const DRAG: f32 = 0.8;
const MOVE_SPEED: f32 = 25.0;
const MAX_VELOCITY: f32 = 4.0;
const COLLISION_RADIUS: f32 = 32.0; // Needs to match the size of the sprite

pub fn player_physics(
    time: Res<Time>,
    gamepad_inputs: Res<GamepadInputs>,
    mut player_query: Query<(&mut Player, &mut Transform), Without<Dead>>,
) {
    // Iterate through each player and collect positions so we can do collision detection
    let mut player_positions: HashMap<usize, Vec3> = HashMap::new();
    for (player, transform) in player_query.iter_mut() {
        player_positions.insert(player.id, transform.translation);
    }
    // For each player, store positions of other players that we are colliding with (because we could hit more than one at once)
    let mut player_collisions: HashMap<usize, Vec<Vec3>> = HashMap::new();
    for (p, pos) in player_positions.iter() {
        for (p2, pos2) in player_positions.iter() {
            // Don't collide with one's self
            if p == p2 {
                continue;
            }
            if (*pos - *pos2).length() < COLLISION_RADIUS * 2.0 {
                player_collisions.entry(*p).or_default().push(*pos2);
            }
        }
    }
    // Iterate through each player and apply physics
    for (mut player, mut transform) in player_query.iter_mut() {
        // Apply fixed drag so players slow to a stop
        player.vel *= 1.0 - time.delta_seconds * DRAG;

        // Adjust velocity based on gamepad input
        let input = gamepad_inputs.inputs.get(&player.id).unwrap();
        let left_x = input.left_stick.x();
        let left_y = input.left_stick.y();
        if Vec2::new(left_x, left_y).length() > DEAD_ZONE_THRESHOLD {
            *player.vel.x_mut() += left_x * time.delta_seconds * MOVE_SPEED;
            *player.vel.y_mut() += left_y * time.delta_seconds * MOVE_SPEED;
        }
        if player.vel.length() > MAX_VELOCITY {
            player.vel = player.vel.normalize() * MAX_VELOCITY;
        }

        // Process this player's collisions
        if let Some(collisions) = player_collisions.get(&player.id) {
            for pos2 in collisions {
                // See https://math.stackexchange.com/questions/13261/how-to-get-a-reflection-vector
                let normal_vector = (transform.translation - *pos2).normalize();
                let n = Vec2::new(normal_vector[0], normal_vector[1]);
                let d = player.vel; // already a Vec2
                let r = d - 2.0 * (d.dot(n)) * n;
                player.vel = r;
            }
        }

        // Apply velocity to position
        *transform.translation.x_mut() += player.vel.x() * time.delta_seconds * MOVE_SPEED;
        *transform.translation.y_mut() += player.vel.y() * time.delta_seconds * MOVE_SPEED;

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

/// Handle players joining the game
pub fn player_join_system(
    commands: &mut Commands,
    mut listeners: ResMut<EventListeners>,
    colors: Res<PlayerColors>,
    player_spawn_events: Res<Events<PlayerSpawnEvent>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for player_spawn_event in listeners.player_spawn_reader.iter(&player_spawn_events) {
        println!("Player {} spawns", player_spawn_event.id);
        let glove = asset_server.load("glove.png");
        let circle_texture = asset_server.load("circle.png");
        let circle_material = ColorMaterial {
            color: colors.0[player_spawn_event.id],
            texture: circle_texture.into(),
        };
        let circle_handle = materials.add(circle_material);
        commands
            .spawn(SpriteComponents {
                material: circle_handle.clone(),
                transform: Transform::from_translation(
                    STARTING_LOCATIONS[player_spawn_event.id].into(),
                ),
                ..Default::default()
            })
            .with(Player::new(player_spawn_event.id))
            .with_children(|parent| {
                // Punching Glove
                parent.spawn(SpriteComponents {
                    material: materials.add(glove.into()),
                    transform: Transform::from_translation(Vec3::new(40.0, 0.0, 0.1)),
                    ..Default::default()
                });
            });
    }
}
