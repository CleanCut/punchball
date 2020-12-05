use super::PlayerID;
use bevy::prelude::*;

/// A collision between two players
#[derive(Copy, Clone, Default)]
pub(crate) struct Collision {
    pub(crate) player_id1: PlayerID,
    pub(crate) player_id2: PlayerID,
    pub(crate) pos1: Vec2,
    pub(crate) pos2: Vec2,
    pub(crate) vel1: Vec2,
    pub(crate) vel2: Vec2,
}

impl Collision {
    /// Ask the collision if you're involved, if you are, what's your new velocity?
    pub(crate) fn new_velocity(&self, player_id: PlayerID) -> Option<Vec2> {
        // From the bottom of https://en.wikipedia.org/wiki/Elastic_collision assuming equal masses
        if player_id == self.player_id1 {
            Some(
                self.vel1
                    - ((self.vel1 - self.vel2).dot(self.pos1 - self.pos2)
                        / (self.pos1 - self.pos2).length_squared())
                        * (self.pos1 - self.pos2),
            )
        } else if player_id == self.player_id2 {
            Some(
                self.vel2
                    - ((self.vel2 - self.vel1).dot(self.pos2 - self.pos1)
                        / (self.pos2 - self.pos1).length_squared())
                        * (self.pos2 - self.pos1),
            )
        } else {
            None
        }
    }
}

/// Make it so that a collision involving two players is equal to any other collision involving the two players
impl PartialEq for Collision {
    fn eq(&self, other: &Self) -> bool {
        ((self.player_id1 == other.player_id1) && (self.player_id2 == other.player_id2))
            || ((self.player_id1 == other.player_id2) && (self.player_id2 == other.player_id1))
    }
}

impl Eq for Collision {
    fn assert_receiver_is_total_eq(&self) {}
}

/// Make it so that a collision involving two players hashes the same as any other collision with
/// the same two players, so a HashSet will naturally deduplicate opposing pairs of collisions
impl std::hash::Hash for Collision {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut player_ids = vec![self.player_id1, self.player_id2];
        player_ids.sort();
        for player_id in player_ids {
            player_id.hash(state);
        }
    }
}
