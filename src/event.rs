use bevy::prelude::*;

#[derive(Default)]
pub struct EventListeners {
    pub player_spawn_reader: EventReader<PlayerSpawnEvent>,
}

#[derive(Default)]
pub struct PlayerSpawnEvent {
    pub id: usize,
}
