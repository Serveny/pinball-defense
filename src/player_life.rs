use crate::prelude::*;
use crate::progress_bar::QueryProgressBar;
use crate::{GameState, IngameTime};

pub struct PlayerLifePlugin;

impl Plugin for PlayerLifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (recovery_system).run_if(in_state(GameState::Ingame)),
        );
    }
}

#[derive(Component)]
pub struct LifeBar;

#[derive(Component, Default)]
struct LastDamage {
    time: f32,
    value: f32,
}

pub fn spawn_life_bar(
    parent: &mut ChildBuilder,
    assets: &PinballDefenseAssets,
    materials: &mut Assets<StandardMaterial>,
    trans: Transform,
) {
    parent
        .spawn((
            SpatialBundle::from_transform(trans),
            LifeBar,
            Name::new("Life Bar"),
            LastDamage::default(),
        ))
        .with_children(|parent| {
            crate::progress_bar::spawn(
                parent,
                assets,
                materials,
                parent.parent_entity(),
                Transform::default(),
                Color::GREEN,
                1.,
            )
        });
}

const RECOVERY_TIMEOUT_SEC: f32 = 6.;

fn recovery_system(
    time: Res<Time>,
    ig_time: Res<IngameTime>,
    mut q_life_bar: Query<(Entity, &mut LastDamage)>,
    mut q_prog_bar: QueryProgressBar,
) {
    for (id, mut last_damage) in q_life_bar.iter_mut() {
        if let Some((_, mut bar)) = q_prog_bar.iter_mut().find(|(rel_id, _)| ***rel_id == id) {
            // Check if damage was done
            if last_damage.value > bar.0 {
                last_damage.time = ig_time.0;
            }
            // If no damage and life is not full, check if you can recover
            else if bar.0 < 1. && ig_time.0 >= last_damage.time + RECOVERY_TIMEOUT_SEC {
                bar.0 += time.delta_seconds() * 0.01;
            }

            last_damage.value = bar.0;
        }
    }
}
