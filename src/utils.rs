use crate::enemy::Enemy;
use crate::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct RelEntity(pub Entity);

pub fn enemies_in_radius(
    pos: Vec3,
    radius: f32,
    q_enemy: &Query<(Entity, &Transform), With<Enemy>>,
) -> Vec<Entity> {
    q_enemy
        .iter()
        .filter(|(_, enemy_pos)| distance_squared(enemy_pos.translation, pos) <= radius.powi(2))
        .map(|(entity, _)| entity)
        .collect()
}

pub fn enemy_pos_in_radius(
    pos: Vec3,
    radius: f32,
    q_enemy: &Query<&Transform, With<Enemy>>,
) -> Vec<Vec3> {
    q_enemy
        .iter()
        .filter(|enemy_pos| distance_squared(enemy_pos.translation, pos) <= radius.powi(2))
        .map(|pos| pos.translation)
        .collect()
}
fn distance_squared(pos1: Vec3, pos2: Vec3) -> f32 {
    pos1.distance_squared(pos2)
}
