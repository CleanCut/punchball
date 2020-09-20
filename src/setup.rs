use bevy::prelude::*;
#[derive(Default)]
pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .add_startup_system(setup.system());
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}
