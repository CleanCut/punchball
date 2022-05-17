use bevy::prelude::*;

#[derive(Default)]
pub struct EventPlugin;
impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerSpawnEvent>();
    }
}

#[derive(Default)]
pub struct PlayerSpawnEvent {
    pub id: usize,
}
