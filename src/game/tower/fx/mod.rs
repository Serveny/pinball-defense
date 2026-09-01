use crate::game::tower::types::gun::GunTowerBarrel;
use crate::prelude::*;
use crate::utils::RelEntity;
use bevy_hanabi::prelude::*;

mod barrel_smoke;
mod muzzle_flash;

use barrel_smoke::asset as barrel_smoke_asset;
use muzzle_flash::asset as muzzle_flash_asset;

const MUZZLE_LOCAL: Vec3 = Vec3::new(0., 0.095, 0.0235);

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
    q_barrels: Query<(Entity, &RelEntity), (With<GunTowerBarrel>, Without<GunEffectsSpawned>)>,
    mut cmds: Commands,
) {
    for (barrel_id, rel_id) in q_barrels.iter() {
        cmds.entity(barrel_id).insert(GunEffectsSpawned).with_children(
            |barrel| {
                for (name, handle) in [
                    ("Muzzle Flash Effect", muzzle_assets.flash.clone()),
                    ("Barrel Smoke Effect", muzzle_assets.smoke.clone()),
                ] {
                    barrel.spawn((
                        Name::new(name),
                        ParticleEffect::new(handle),
                        Transform::from_translation(MUZZLE_LOCAL),
                        GunFiringEffects,
                        RelEntity(rel_id.0),
                    ));
                }
            },
        );
    }
}