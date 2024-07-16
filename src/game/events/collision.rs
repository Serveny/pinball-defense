use crate::prelude::*;

// Collision Groups
pub const BALL: Group = Group::GROUP_1;

pub const INTERACT_WITH_BALL: Group = Group::GROUP_2;

pub const COLLIDE_ONLY_WITH_BALL: CollisionGroups = CollisionGroups::new(INTERACT_WITH_BALL, BALL);

pub const ENEMY: Group = Group::GROUP_3;
pub const INTERACT_WITH_ENEMY: Group = Group::GROUP_4;

pub const COLLIDE_ONLY_WITH_ENEMY: CollisionGroups =
    CollisionGroups::new(INTERACT_WITH_ENEMY, ENEMY);
