use crate::damage::{DamageList, RadiusDamage};
use crate::enemy::Enemy;
use crate::prelude::*;
use crate::utils::enemy_pos_in_radius;

#[derive(Component)]
pub(super) struct SightRadius(pub f32);

#[derive(Component)]
pub(super) struct AimFirstEnemy;

pub(super) fn aim_first_enemy_system(
    mut q_afe: Query<(&mut DamageList, &Transform, &SightRadius), With<AimFirstEnemy>>,
    q_enemy: Query<&Transform, With<Enemy>>,
) {
    for (mut dmg_list, trans, sight_radius) in q_afe.iter_mut() {
        if let Some(enemy_pos) =
            enemy_pos_in_radius(trans.translation, sight_radius.0, &q_enemy).first()
        {
            if let Some(dmg) = dmg_list.0.first_mut() {
                dmg.pos = *enemy_pos;
                log!("👾 New damage pos at {}", dmg.pos);
            } else {
                let dmg = RadiusDamage::new(*enemy_pos, 0.01, 0.2, None);
                log!("🦀 New first enemy at {}", dmg.pos);
                dmg_list.0.push(dmg);
            }
        } else if !dmg_list.0.is_empty() {
            log!("👾 Remove damage from tower at {}", trans.translation);
            dmg_list.0.remove(0);
        }
    }
}
