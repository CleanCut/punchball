use bevy::prelude::*;

#[derive(Default)]
pub struct EventPlugin;
impl Plugin for EventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<EventListeners>()
            .add_event::<PlayerSpawnEvent>();
    }
}

#[derive(Default)]
pub struct EventListeners {
    pub player_spawn_reader: EventReader<PlayerSpawnEvent>,
}

#[derive(Default)]
pub struct PlayerSpawnEvent {
    pub id: usize,
}
