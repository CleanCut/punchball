use std::collections::HashSet;

use crate::{color_from_f32, physics::BodyHandleToEntity, player::Player};
use bevy::{
    ecs::bevy_utils::{HashMap, HashMapExt},
    prelude::*,
};
use bevy_lyon::{math, shapes, LyonMeshBuilder};
use bevy_rapier2d::{
    physics::EventQueue,
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder, geometry::Proximity},
};

const ARENA_RADIUS: f32 = 300.0;

#[derive(Default)]
pub struct ArenaPlugin;
impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(LeaveArenaDebouncer {
            player_timers: HashMap::with_capacity(4),
        })
        .add_system(spawn_arena_system.system())
        .add_system(leave_arena_system.system());
    }
}

pub struct Arena;

fn spawn_arena_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let fill_circle = meshes.add(LyonMeshBuilder::with_only(shapes::FillCircle {
        center: math::point(0.0, 0.0),
        radius: ARENA_RADIUS,
        ..Default::default()
    }));
    commands
        .spawn(SpriteComponents {
            mesh: fill_circle,
            material: materials.add(color_from_f32(0.2, 0.2, 0.2).into()),
            sprite: Sprite::new(Vec2::new(1.0, 1.0)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)),
            ..Default::default()
        })
        .with(Arena)
        .with(RigidBodyBuilder::new_dynamic())
        .with(ColliderBuilder::ball(ARENA_RADIUS).sensor(true));
}

struct LeaveArenaDebouncer {
    player_timers: HashMap<usize, Timer>,
}

fn leave_arena_system(
    physics_events: Res<EventQueue>,
    bh_to_e: ResMut<BodyHandleToEntity>,
    players: Query<&Player>,
    arena: Query<&Arena>,
) {
    let mut leaving_players = HashSet::new();
    let mut i = 0;
    while let Ok(proximity_event) = physics_events.proximity_events.pop() {
        if proximity_event.new_status == Proximity::Disjoint {
            let e1 = *(bh_to_e.0.get(&proximity_event.collider1).unwrap());
            let e2 = *(bh_to_e.0.get(&proximity_event.collider2).unwrap());
            if players.get::<Player>(e1).is_ok() && arena.get::<Arena>(e2).is_ok() {
                leaving_players.insert(players.get::<Player>(e1).unwrap().id);
            }
            if players.get::<Player>(e2).is_ok() && arena.get::<Arena>(e1).is_ok() {
                leaving_players.insert(players.get::<Player>(e2).unwrap().id);
            }
        }
        i += 1;
    }
    if i > 0 {
        println!("{}", i);
    }
    for id in leaving_players {
        println!("Player {} left the arena", id);
    }
}
