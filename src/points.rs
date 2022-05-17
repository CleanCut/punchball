use bevy::prelude::*;

use crate::{player::Player, prelude::*};

#[derive(Default)]
pub struct PointsPlugin;
impl Plugin for PointsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(point_decay_system)
            .add_system(win_system)
            .add_system(cleanup_win_message_system);
    }
}

#[derive(Component, Copy, Clone, Default)]
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

#[derive(Component, Clone, Default)]
pub struct WinningMessage {
    timer: Timer,
}

pub fn point_decay_system(time: Res<Time>, mut players: Query<&mut Player>) {
    for mut player in players.iter_mut() {
        if player.point_timer.tick(time.delta()).finished() {
            player.point_recipient = None;
        }
    }
}

pub fn win_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    points_query: Query<&Points>,
    winning_message_query: Query<&WinningMessage>,
) {
    if winning_message_query.iter().next().is_some() {
        // Someone has already won, so don't trigger another win until the message has disappeared
        return;
    }
    let mut winning_player = None;
    for points in points_query.iter() {
        if points.value >= WIN_POINTS {
            winning_player = Some(points.player_id);
            break;
        }
    }
    if let Some(player_id) = winning_player {
        // Create the winning message
        commands
            .spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    format!("Player {} wins!", player_id),
                    TextStyle {
                        font: asset_server.load("FiraMono-Medium.ttf"),
                        font_size: 90.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, LAYER_POINTS)),
                ..default()
            })
            .insert(WinningMessage {
                timer: Timer::from_seconds(3.0, false),
            });
    }
}

fn cleanup_win_message_system(
    mut commands: Commands,
    time: Res<Time>,
    mut winning_message_query: Query<(Entity, &mut WinningMessage)>,
    mut points_query: Query<(&mut Points, &mut Text)>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
) {
    for (entity, mut winning_message) in winning_message_query.iter_mut() {
        if winning_message.timer.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
            // Reset the game
            for (mut points, mut text) in points_query.iter_mut() {
                points.value = 0;
                text.sections[0].value = "0".to_string();
            }
            for (mut player, mut transform) in player_query.iter_mut() {
                transform.translation = STARTING_LOCATIONS[player.id].into();
                player.vel = Vec2::ZERO;
            }
        }
    }
}
