use bevy::prelude::*;

use crate::prelude::*;

#[derive(Default)]
pub struct ArenaPlugin;
impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_arena_system);
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
        .spawn(SpriteBundle {
            material: materials.add(arena_texture.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, LAYER_ARENA)),
            ..Default::default()
        })
        .with(Arena);
}
