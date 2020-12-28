use bevy::prelude::*;

use crate::{player::Player, prelude::*};

#[derive(Default)]
pub struct PointsPlugin;
impl Plugin for PointsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(point_timer_system.system());
    }
}

#[derive(Copy, Clone, Default)]
pub struct Points {
    pub player_id: PlayerID,
    pub value: usize,
}

impl Points {
    pub fn new(player_id: PlayerID) -> Self {
        Self {
            player_id,
            value: 0,
        }
    }
}

pub fn point_timer_system(time: Res<Time>, mut players: Query<&mut Player>) {
    for mut player in players.iter_mut() {
        if player.point_timer.tick(time.delta_seconds()).finished() {
            player.point_recipient = None;
        }
    }
}
