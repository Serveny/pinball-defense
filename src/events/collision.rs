use crate::prelude::*;

#[derive(Event)]
pub struct TowerMenuElementCollisionStartEvent(pub Entity);

#[derive(Event)]
pub struct PinballEnemyHitEvent(pub Entity);

// Collision Groups
pub const BALL: Group = Group::GROUP_1;

pub const INTERACT_WITH_BALL: Group = Group::GROUP_2;

pub fn collider_only_interact_with_ball() -> CollisionGroups {
    CollisionGroups::new(INTERACT_WITH_BALL, BALL)
}

pub const ENEMY: Group = Group::GROUP_3;
pub const INTERACT_WITH_ENEMY: Group = Group::GROUP_4;

pub fn collider_only_interact_with_enemy() -> CollisionGroups {
    CollisionGroups::new(INTERACT_WITH_ENEMY, ENEMY)
}
