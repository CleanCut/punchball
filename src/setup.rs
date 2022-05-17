use bevy::prelude::*;
#[derive(Default)]
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .insert_resource(WindowDescriptor {
                title: "Punch Ball".to_string(),
                width: 1024.0,
                height: 1024.0,
                resizable: false,
                cursor_locked: false,
                ..Default::default()
            })
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}
