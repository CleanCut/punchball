use bevy::prelude::*;
#[derive(Default)]
pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .add_resource(WindowDescriptor {
                title: "Punch Ball".to_string(),
                width: 1024,
                height: 1024,
                vsync: true,
                resizable: false,
                cursor_locked: false,
                ..Default::default()
            })
            .add_startup_system(setup);
    }
}

fn setup(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}
