use bevy::prelude::*;

use crate::prelude::*;

#[derive(Default)]
pub struct ArenaPlugin;
impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_arena_system);
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct Arena;

fn spawn_arena_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("arena.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, LAYER_ARENA)),
            ..Default::default()
        })
        .insert(Arena);
}
