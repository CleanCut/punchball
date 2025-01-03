use std::time::Duration;

use crate::{
    arena::Arena, event::PlayerSpawnEvent, gamepad::GamepadInputs, points::Points, prelude::*,
};
use bevy::{
    math::Vec3Swizzles,
    prelude::*,
    utils::{HashMap, HashSet},
};

mod collision;
use collision::Collision;

/// Plugin for all resources and systems in this module
#[derive(Default)]
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerColors::default())
            .add_system(dead_players_system)
            .add_system(leave_arena_system)
            .add_system(player_join_system)
            .add_system(player_physics_system)
            .add_system(punch_animation_system);
    }
}

/// Colors assigned to players
#[derive(Resource)]
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
/// A component to use to store most player attributes. Translation, scale, and rotation are in a
/// separate Transform component that Bevy provides.
#[derive(Component)]
pub struct Player {
    /// Player ID
    pub id: PlayerID,
    pub facing: Vec2,
    pub vel: Vec2,
    pub respawn_timer: Timer,
    pub punch_timer: Timer,
    pub point_recipient: Option<PlayerID>,
    pub point_timer: Timer,
}
impl Player {
    pub fn new(id: PlayerID) -> Self {
        let mut punch_timer = Timer::from_seconds(PUNCH_DRAWBACK_DURATION, TimerMode::Once);
        // For the sake of animation, the timer should be "finished" to start with.
        punch_timer.tick(Duration::from_secs_f32(PUNCH_DRAWBACK_DURATION * 2.0));
        Self {
            id,
            facing: Vec2::X,
            vel: Vec2::ZERO,
            respawn_timer: Timer::from_seconds(RESPAWN_DURATION, TimerMode::Once),
            punch_timer,
            point_recipient: None,
            point_timer: Timer::from_seconds(POINT_TOUCH_DURATION, TimerMode::Once),
        }
    }
}
/// A component to mark that a player is dead
#[derive(Component, Copy, Clone, Default)]
pub struct Dead {}

/// A component to mark that something is a boxing glove
#[derive(Component, Copy, Clone, Default)]
pub struct Glove {}

impl Glove {
    pub fn new() -> Self {
        Self {}
    }
}

/// Determine whether something with a position and velocity is moving towards or away from a point
fn moving_towards(toward_pos: Vec2, obj_pos: Vec2, obj_vel: Vec2) -> bool {
    let position_vector = toward_pos - obj_pos;
    position_vector.dot(obj_vel) > 0.0
}

/// Animate and respawn dead players
pub fn dead_players_system(
    mut commands: Commands,
    time: Res<Time>,
    mut players: Query<(Entity, &mut Player, &mut Transform), With<Dead>>,
) {
    for (entity, mut player, mut transform) in players.iter_mut() {
        // Decrement the timer for how long the player has left to be dead
        player.respawn_timer.tick(time.delta());
        // Death animation
        transform.scale = Vec3::ONE.lerp(Vec3::ZERO, player.respawn_timer.percent());
        // Is the player done being dead?
        if player.respawn_timer.finished() {
            // Set the scale back to normal
            transform.scale = Vec3::ONE;
            // Restart the timer for next time
            player.respawn_timer.reset();
            // Reset velocity
            player.vel = Vec2::ZERO;
            // Spawn at the starting location
            transform.translation = STARTING_LOCATIONS[player.id].into();
            // Remove the "Dead" component
            commands.entity(entity).remove::<Dead>();
        }
    }
}

/// Detect a player leaving the arena, and mark him dead.
fn leave_arena_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &mut Player), Without<Dead>>,
    arena_transform_query: Query<&Transform, With<Arena>>,
    mut points_query: Query<(&mut Points, &mut Text)>,
) {
    let mut points_awarded = Vec::new();
    for arena_transform in arena_transform_query.iter() {
        for (entity, player_transform, player) in player_query.iter_mut() {
            if (player_transform.translation - arena_transform.translation).length() > ARENA_RADIUS
            {
                if let Some(puncher_id) = player.point_recipient {
                    println!(
                        "Player {} was punched out of the arena by player {}.",
                        player.id, puncher_id
                    );
                    points_awarded.push(puncher_id);
                } else {
                    println!(
                        "Player {} didn't watch where they were going, and fell off the arena.",
                        player.id
                    );
                }
                commands.entity(entity).insert(Dead {});
            }
        }
    }
    for (mut points, mut text) in points_query.iter_mut() {
        let new_points = points_awarded
            .iter()
            .filter(|&&x| x == points.player_id)
            .count();
        if new_points == 0 {
            continue;
        }
        points.value += new_points;
        println!(
            "Player {} now has {} points",
            points.player_id, points.value
        );
        text.sections[0].value = format!("{}", points.value);
    }
}

/// Game physics - The bulk of the movement / punching logic
pub fn player_physics_system(
    time: Res<Time>,
    gamepad_inputs: Res<GamepadInputs>,
    mut player_query: Query<(&mut Player, &mut Transform), Without<Dead>>,
) {
    // Iterate through each player and collect positions so we can do collision detection
    let mut player_positions: HashMap<PlayerID, Vec2> = HashMap::new();
    let mut player_velocities: HashMap<PlayerID, Vec2> = HashMap::new();
    for (player, transform) in player_query.iter_mut() {
        player_positions.insert(player.id, transform.translation.xy());
        player_velocities.insert(player.id, player.vel);
    }
    // For each player, store positions of other players that we are colliding with (because we could hit more than one at once)
    let mut player_collisions: HashSet<Collision> = HashSet::new();
    for (&player_id1, &pos1) in player_positions.iter() {
        for (&player_id2, &pos2) in player_positions.iter() {
            // Don't collide with one's self
            if player_id1 == player_id2 {
                continue;
            }
            if (pos1 - pos2).length() < COLLISION_RADIUS * 2.0 {
                let collision = Collision {
                    player_id1,
                    player_id2,
                    pos1,
                    pos2,
                    vel1: player_velocities[&player_id1],
                    vel2: player_velocities[&player_id2],
                };
                player_collisions.insert(collision);
            }
        }
    }

    // For each player, store the direction and location of each punch
    let mut punches: Vec<(PlayerID, Quat, Vec2)> = Vec::new();
    for (mut player, transform) in player_query.iter_mut() {
        if !gamepad_inputs
            .inputs
            .get(&player.id)
            .unwrap()
            .right_trigger2
        {
            continue;
        }
        // Can't punch until previous punch has finished
        if !player.punch_timer.finished() {
            continue;
        }
        //println!("Player {} punches", player.id);
        player.punch_timer.reset();
        punches.push((
            player.id,
            transform.rotation,
            (transform.translation + transform.rotation * (Vec3::X * (PUNCH_BASE + PUNCH_LENGTH)))
                .xy(),
        ));
    }
    // For each punch, store velocity deltas for who got punched and who got pushed back from
    // punching someone else, to be resolved during the physics step.
    let mut punch_vel_deltas: HashMap<PlayerID, Vec<Vec2>> = HashMap::new();
    for (mut punchee, transform) in player_query.iter_mut() {
        for (puncher_id, direction, punch) in &punches {
            // Players are unable to punch themselves
            if *puncher_id == punchee.id {
                continue;
            }
            let punch_vector = transform.translation.xy() - *punch;
            // Did the punch connect?
            if punch_vector.length() < 2.0 * COLLISION_RADIUS {
                // Handle point timer on punchee
                punchee.point_timer.reset();
                punchee.point_recipient = Some(*puncher_id);
                // Process punch physics
                let punch_delta =
                    ((*direction * Vec3::X) * (PUNCH_PUSHBACK_OTHER * MAX_VELOCITY)).xy();
                punch_vel_deltas
                    .entry(punchee.id)
                    .or_default()
                    .push(punch_delta);
                let pushback_delta =
                    ((*direction * Vec3::X) * (-1.0 * PUNCH_PUSHBACK_SELF * MAX_VELOCITY)).xy();
                punch_vel_deltas
                    .entry(*puncher_id)
                    .or_default()
                    .push(pushback_delta);
            }
        }
    }

    // Iterate through each player and apply physics
    for (mut player, mut transform) in player_query.iter_mut() {
        // Collect some info so we can deal with different slowing mechanics if you've been punched
        let starting_velocity = player.vel.length();
        let coming_down_to_max = starting_velocity > MAX_VELOCITY;

        // Apply fixed drag so players slow to a stop eventually
        player.vel *= 1.0 - time.delta_seconds() * DRAG;

        // Adjust velocity based on gamepad input
        let input = gamepad_inputs.inputs.get(&player.id).unwrap();
        let left_x = input.left_stick.x;
        let left_y = input.left_stick.y;
        if Vec2::new(left_x, left_y).length() > DEAD_ZONE_THRESHOLD {
            player.vel.x += left_x * time.delta_seconds() * MOVE_SPEED;
            player.vel.y += left_y * time.delta_seconds() * MOVE_SPEED;
        }
        // Make sure velocity doesn't go too high
        if coming_down_to_max {
            // Recently punched, so let our velocity exceed max, but make sure it decreases each frame
            if player.vel.length() > starting_velocity {
                // let the player change direction, but cap the velocity at previous frame and add double drag
                player.vel = player.vel.normalize()
                    * starting_velocity
                    * (1.0 - time.delta_seconds() * DRAG * 2.0);
            }
        } else if player.vel.length() > MAX_VELOCITY {
            // We're moving normally, so cap velocity
            player.vel = player.vel.normalize() * MAX_VELOCITY;
        }

        // Process any punches (or pushbacks from punches) that affect velocity - these can exceed max velocity
        if let Some(vel_deltas) = punch_vel_deltas.get(&player.id) {
            for &delta in vel_deltas {
                player.vel += delta;
            }
        }

        // Process this player's collisions, adjusting velocities accordingly
        for collision in player_collisions.iter() {
            if let Some(new_velocity) = collision.new_velocity(player.id) {
                // Don't collide if we aren't moving toward each other (let them depenetrate)
                let relative_vel = collision.vel2 - collision.vel1;
                if !moving_towards(collision.pos1, collision.pos2, relative_vel) {
                    // Already still or moving away, but overlapping. Let's give the player a nudge.
                    player.vel.x *= 1.0 + MOVE_SPEED * time.delta_seconds();
                    player.vel.y *= 1.0 + MOVE_SPEED * time.delta_seconds();
                    // ...but don't shoot across the screen like a bullet. Clamp to max velocity.
                    if player.vel.length() > MAX_VELOCITY {
                        player.vel = player.vel.normalize() * MAX_VELOCITY;
                    }
                    continue;
                }
                // Accept the new velocity calculated by the collision
                player.vel = new_velocity;
            }
        }

        // Apply velocity to position
        transform.translation.x += player.vel.x * time.delta_seconds() * MOVE_SPEED;
        transform.translation.y += player.vel.y * time.delta_seconds() * MOVE_SPEED;

        // Set direction of player with right stick
        let facing_vec = Vec2::new(input.right_stick.x, input.right_stick.y);
        if facing_vec.length() > DEAD_ZONE_THRESHOLD {
            let quat =
                Quat::from_axis_angle(Vec3::Z, Vec2::new(1.0, 0.0).angle_between(facing_vec));
            // "Smooth" turning
            if transform.rotation.dot(quat) >= 0.0 {
                transform.rotation = transform
                    .rotation
                    .slerp(quat, TURN_SPEED * time.delta_seconds());
            } else {
                transform.rotation =
                    (transform.rotation * -1.0).slerp(quat, TURN_SPEED * time.delta_seconds());
            }
        }
    }
}

/// Animate punches
pub fn punch_animation_system(
    time: Res<Time>,
    mut glove_query: Query<(&mut Transform, &Parent), With<Glove>>,
    mut player_query: Query<&mut Player>,
) {
    for (mut transform, parent) in glove_query.iter_mut() {
        let mut player = player_query.get_mut(parent.get()).unwrap();
        player.punch_timer.tick(time.delta());
        let punch_base_vec3 = Vec3::from(PUNCH_BASE_ARR3);
        let punch_extended_vec3 = Vec3::from(PUNCH_EXTENDED_ARR3);
        transform.translation =
            punch_base_vec3.lerp(punch_extended_vec3, player.punch_timer.percent_left());
    }
}

/// Handle players joining the game
pub fn player_join_system(
    mut commands: Commands,
    colors: Res<PlayerColors>,
    mut player_spawn_events: EventReader<PlayerSpawnEvent>,
    asset_server: Res<AssetServer>,
) {
    for player_spawn_event in player_spawn_events.iter() {
        let player_id: PlayerID = player_spawn_event.id;
        //println!("Player {} spawns", player_id);
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("circle.png"),
                transform: Transform::from_translation(STARTING_LOCATIONS[player_id].into()),
                sprite: Sprite {
                    color: colors.0[player_id],
                    ..default()
                },
                ..default()
            })
            .insert(Player::new(player_id))
            .with_children(|parent| {
                // Punching Glove
                parent
                    .spawn(SpriteBundle {
                        texture: asset_server.load("glove.png"),
                        transform: Transform::from_translation(Vec3::from(PUNCH_BASE_ARR3)),
                        ..default()
                    })
                    .insert(Glove::new());
                parent
                    .spawn(Text2dBundle {
                        text: Text::from_section(
                            "0",
                            TextStyle {
                                font: asset_server.load("FiraMono-Medium.ttf"),
                                font_size: 48.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, LAYER_POINTS)),
                        ..default()
                    })
                    .insert(Points::new(player_id));
            });
    }
}
