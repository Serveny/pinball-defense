use crate::game::tower::types::gun::{GunTowerHead, MG_MUZZLE_LOCAL};
use crate::prelude::*;
use crate::utils::RelEntity;
use bevy_hanabi::prelude::*;

mod barrel_smoke;
mod muzzle_flash;
pub(in super::super) mod tesla;

use barrel_smoke::asset as barrel_smoke_asset;
use muzzle_flash::asset as muzzle_flash_asset;

#[derive(Component)]
pub(in super::super) struct GunFiringEffects;

#[derive(Component)]
pub(in super::super) struct GunEffectsSpawned;

#[derive(Resource)]
pub(in super::super) struct MuzzleEffectAssets {
    pub(super) flash: Handle<EffectAsset>,
    pub(super) smoke: Handle<EffectAsset>,
}

impl FromWorld for MuzzleEffectAssets {
    fn from_world(world: &mut World) -> Self {
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();
        Self {
            flash: effects.add(muzzle_flash_asset()),
            smoke: effects.add(barrel_smoke_asset()),
        }
    }
}

pub(in super::super) fn spawn_gun_effects_system(
    muzzle_assets: Res<MuzzleEffectAssets>,
    q_heads: Query<(Entity, &RelEntity), (With<GunTowerHead>, Without<GunEffectsSpawned>)>,
    mut cmds: Commands,
) {
    for (head_id, rel_id) in q_heads.iter() {
        cmds.entity(head_id)
            .insert(GunEffectsSpawned)
            .with_children(|barrel| {
                for (name, handle) in [
                    ("Muzzle Flash Effect", muzzle_assets.flash.clone()),
                    ("Barrel Smoke Effect", muzzle_assets.smoke.clone()),
                ] {
                    barrel.spawn((
                        Name::new(name),
                        ParticleEffect::new(handle),
                        Transform::from_translation(MG_MUZZLE_LOCAL),
                        GunFiringEffects,
                        RelEntity(rel_id.0),
                    ));
                }
            });
    }
}
