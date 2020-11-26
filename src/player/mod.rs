use crate::prelude::*;
use crate::{
    arena::Arena,
    event::{EventListeners, PlayerSpawnEvent},
    gamepad::GamepadInputs,
};
use bevy::{
    math::Vec3Swizzles,
    prelude::*,
    utils::{AHashExt, HashMap, HashSet},
};

mod collision;
use collision::Collision;

/// Plugin for all resources and systems in this module
#[derive(Default)]
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(PlayerColors::default())
            .add_system(dead_players_system.system())
            .add_system(leave_arena_system.system())
            .add_system(player_join_system.system())
            .add_system(player_physics_system.system())
            .add_system(punch_animation_system.system());
    }
}

/// An alias to show that we're dealing with a player id
type PlayerID = usize;

/// Colors assigned to players
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
pub struct Player {
    /// Player ID
    pub id: PlayerID,
    pub facing: Vec2,
    pub vel: Vec2,
    pub respawn_timer: Timer,
}

impl Player {
    pub fn new(id: PlayerID) -> Self {
        Self {
            id,
            facing: Vec2::unit_x(),
            vel: Vec2::zero(),
            respawn_timer: Timer::from_seconds(RESPAWN_DURATION, false),
        }
    }
}
/// A component to mark that a player is dead
pub struct Dead {}

/// A component to mark that something is a boxing glove
pub struct Glove {
    pub punch_timer: Timer,
}

impl Glove {
    pub fn new() -> Self {
        let mut punch_timer = Timer::from_seconds(PUNCH_DRAWBACK_DURATION, false);
        // For the sake of animation, the timer should be "finished" to start with.
        punch_timer.tick(PUNCH_DRAWBACK_DURATION * 2.0);
        Self { punch_timer }
    }
}

/// Determine the angle from the x axis in radians between v1 as the origin and v2
fn angle_facing(v1: &Vec2, v2: &Vec2) -> f32 {
    (v2.y - v1.y).atan2(v2.x - v1.x)
}

/// Determine whether something with a position and velocity is moving towards or away from a point
fn moving_towards(toward_pos: Vec2, obj_pos: Vec2, obj_vel: Vec2) -> bool {
    let position_vector = toward_pos - obj_pos;
    position_vector.dot(obj_vel) > 0.0
}

/// Animate and respawn dead players
pub fn dead_players_system(
    commands: &mut Commands,
    time: Res<Time>,
    mut players: Query<(Entity, &mut Player, &mut Transform), With<Dead>>,
) {
    for (entity, mut player, mut transform) in players.iter_mut() {
        // Decrement the timer for how long the player has left to be dead
        player.respawn_timer.tick(time.delta_seconds);
        // Death animation
        transform.scale = Vec3::one().lerp(Vec3::zero(), player.respawn_timer.percent());
        // Is the player done being dead?
        if player.respawn_timer.finished {
            // Set the scale back to normal
            transform.scale = Vec3::one();
            // Restart the timer for next time
            player.respawn_timer.reset();
            // Reset velocity
            player.vel = Vec2::zero();
            // Spawn at the starting location
            transform.translation = STARTING_LOCATIONS[player.id].into();
            // Remove the "Dead" component
            commands.remove_one::<Dead>(entity);
        }
    }
}

/// Detect a player leaving the arena, and mark him dead.
fn leave_arena_system(
    commands: &mut Commands,
    mut player_transforms: Query<(Entity, &Transform, &Player), Without<Dead>>,
    arena_transforms: Query<&Transform, With<Arena>>,
) {
    for arena_transform in arena_transforms.iter() {
        for (entity, player_transform, player) in player_transforms.iter_mut() {
            if (player_transform.translation - arena_transform.translation).length() > ARENA_RADIUS
            {
                println!("Player {} dies", player.id);
                commands.insert_one(entity, Dead {});
            }
        }
    }
}

/// Game physics - The bulk of the movement / punching logic
pub fn player_physics_system(
    time: Res<Time>,
    gamepad_inputs: Res<GamepadInputs>,
    mut player_query: Query<(&Children, &mut Player, &mut Transform), Without<Dead>>,
    mut glove_query: Query<&mut Glove>,
) {
    // Iterate through each player and collect positions so we can do collision detection
    let mut player_positions: HashMap<PlayerID, Vec2> = HashMap::new();
    let mut player_velocities: HashMap<PlayerID, Vec2> = HashMap::new();
    for (_, player, transform) in player_query.iter_mut() {
        player_positions.insert(player.id, transform.translation.into());
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
    for (children, player, transform) in player_query.iter_mut() {
        if !gamepad_inputs
            .inputs
            .get(&player.id)
            .unwrap()
            .right_trigger2
        {
            continue;
        }
        println!("Player {} punches", player.id);
        for child in children.iter() {
            if let Ok(mut glove) = glove_query.get_mut(*child) {
                glove.punch_timer.reset();
            }
        }
        punches.push((
            player.id,
            transform.rotation,
            Vec2::from(
                transform.translation
                    + transform.rotation * (Vec3::unit_x() * (PUNCH_BASE + PUNCH_LENGTH)),
            ),
        ));
    }

    // Iterate through each player and apply physics
    for (_, mut player, mut transform) in player_query.iter_mut() {
        // Collect some info so we can deal with different slowing mechanics if you've been punched
        let starting_velocity = player.vel.length();
        let coming_down_to_max = starting_velocity > MAX_VELOCITY;

        // Apply fixed drag so players slow to a stop eventually
        player.vel *= 1.0 - time.delta_seconds * DRAG;

        // Adjust velocity based on gamepad input
        let input = gamepad_inputs.inputs.get(&player.id).unwrap();
        let left_x = input.left_stick.x;
        let left_y = input.left_stick.y;
        if Vec2::new(left_x, left_y).length() > DEAD_ZONE_THRESHOLD {
            player.vel.x += left_x * time.delta_seconds * MOVE_SPEED;
            player.vel.y += left_y * time.delta_seconds * MOVE_SPEED;
        }
        // Make sure velocity doesn't go too high
        if coming_down_to_max {
            // Recently punched, so let our velocity exceed max, but make sure it decreases each frame
            if player.vel.length() > starting_velocity {
                // let the player change direction, but cap the velocity at previous frame and add double drag
                player.vel = player.vel.normalize()
                    * starting_velocity
                    * (1.0 - time.delta_seconds * DRAG * 2.0);
            }
        } else if player.vel.length() > MAX_VELOCITY {
            // We're moving normally, so cap velocity
            player.vel = player.vel.normalize() * MAX_VELOCITY;
        }

        // Process any punches that hit this player -- a player's own punch won't collide, because it doesn't overlap the instant it is fully extended
        for (player_id, direction, punch) in &punches {
            if *player_id == player.id {
                continue;
            }
            let punch_vector = Vec2::from(transform.translation) - *punch;
            if punch_vector.length() < 2.0 * COLLISION_RADIUS {
                let delta = ((*direction * Vec3::unit_x()) * (3.0 * MAX_VELOCITY)).xy();
                player.vel = player.vel + delta;
            }
        }

        // Process this player's collisions, adjusting velocities accordingly
        for collision in player_collisions.iter() {
            if let Some(new_velocity) = collision.new_velocity(player.id) {
                // Don't collide if we aren't moving toward each other (let them depenetrate)
                let relative_vel = collision.vel2 - collision.vel1;
                if !moving_towards(collision.pos1, collision.pos2, relative_vel) {
                    continue;
                }
                // Accept the new velocity calculated by the collision
                player.vel = new_velocity;
            }
        }

        // Apply velocity to position
        transform.translation.x += player.vel.x * time.delta_seconds * MOVE_SPEED;
        transform.translation.y += player.vel.y * time.delta_seconds * MOVE_SPEED;

        // Set direction of player with right stick
        let facing_vec = Vec2::new(input.right_stick.x, input.right_stick.y);
        if facing_vec.length() > DEAD_ZONE_THRESHOLD {
            let quat = Quat::from_axis_angle(
                Vec3::new(0.0, 0.0, 1.0),
                angle_facing(&Vec2::new(0.0, 0.0), &facing_vec),
            );
            transform.rotation = quat;
        }
    }
}

/// Animate punches
pub fn punch_animation_system(
    time: Res<Time>,
    mut glove_query: Query<(&mut Glove, &mut Transform)>,
) {
    for (mut glove, mut transform) in glove_query.iter_mut() {
        glove.punch_timer.tick(time.delta_seconds);
        let punch_base_vec3 = Vec3::from(PUNCH_BASE_ARR3);
        let punch_extended_vec3 = Vec3::from(PUNCH_EXTENDED_ARR3);
        transform.translation =
            punch_base_vec3.lerp(punch_extended_vec3, glove.punch_timer.percent_left());
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
            .spawn(SpriteBundle {
                material: circle_handle.clone(),
                transform: Transform::from_translation(
                    STARTING_LOCATIONS[player_spawn_event.id].into(),
                ),
                ..Default::default()
            })
            .with(Player::new(player_spawn_event.id))
            .with_children(|parent| {
                // Punching Glove
                parent
                    .spawn(SpriteBundle {
                        material: materials.add(glove.into()),
                        transform: Transform::from_translation(Vec3::from(PUNCH_BASE_ARR3)),
                        ..Default::default()
                    })
                    .with(Glove::new());
            });
    }
}
