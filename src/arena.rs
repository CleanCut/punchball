use crate::player::Player;
use bevy::prelude::*;

const ARENA_RADIUS: f32 = 384.0; // based off of circle radius in the PNG

#[derive(Default)]
pub struct ArenaPlugin;
impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_arena_system.system())
            .add_system(leave_arena_system.system());
    }
}

pub struct Arena;

fn spawn_arena_system(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let arena_texture = asset_server.load("arena.png");
    commands
        .spawn(SpriteComponents {
            material: materials.add(arena_texture.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)),
            ..Default::default()
        })
        .with(Arena);
}

fn leave_arena_system(
    player_transforms: Query<(&Transform, &Player)>,
    arena_transforms: Query<&Transform, With<Arena>>,
) {
    for arena_transform in arena_transforms.iter() {
        for (player_transform, player) in player_transforms.iter() {
            if (player_transform.translation - arena_transform.translation).length() > ARENA_RADIUS
            {
                println!("Player {} dies", player.id);
            }
        }
    }
}
