use crate::color_from_f32;
use bevy::prelude::*;
use bevy_lyon::{math, shapes, LyonMeshBuilder};

pub struct ArenaPlugin;
impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(spawn_arena.system());
    }
}

pub struct Arena;

fn spawn_arena(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let fill_circle = meshes.add(LyonMeshBuilder::with_only(shapes::FillCircle {
        center: math::point(0.0, 0.0),
        radius: 300.0,
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
        .with(Arena);
}
