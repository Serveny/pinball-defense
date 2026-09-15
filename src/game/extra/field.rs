use super::effects::ActiveEffects;
use super::{ExtraField, ExtraFieldKind, ExtraFieldUnlockEvent, KINDS};
use crate::game::IngameTime;
use crate::game::level::{BallCollisionPoints, LevelHub};
use crate::game::light::{
    ContactLight, FlashLight, LightOnCollision, contact_light_bundle, disable_flash_light,
};
use crate::game::progress::{RadialProgressCasing, spawn_radial};
use crate::game::ui;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use bevy_tweening::{Tween, TweenAnim, lens::TransformPositionLens};
use std::time::Duration;

const BUTTON_PRESS_DEPTH: f32 = 0.003;
const BUTTON_RELEASE_SECS: Duration = Duration::from_secs(1);

const FIELD_RADIUS: f32 = 0.071;

const LANE_BOUNDS: (f32, f32, f32, f32) = (0.9, 1.15, 0.55, 0.75); // (x_min, x_max, y_min, y_max)

pub(super) fn lane_occupied(_pos: Vec3, balls: &[Vec3]) -> bool {
    let (x_min, x_max, y_min, y_max) = LANE_BOUNDS;
    balls
        .iter()
        .any(|ball| ball.x >= x_min && ball.x <= x_max && ball.y >= y_min && ball.y <= y_max)
}

fn pick_next_inactive(kinds: &[Option<ExtraFieldKind>]) -> Option<usize> {
    kinds.iter().position(Option::is_none)
}

pub fn spawn_fields(p: &mut ChildSpawnerCommands, g_sett: &GraphicsSettings, posis: [Vec3; 4]) {
    for (kind, pos) in KINDS.iter().zip(posis) {
        p.spawn(field_bundle(*kind, pos)).with_children(|p| {
            p.spawn(contact_light_bundle(g_sett, kind.color()));
        });
    }
}

fn field_bundle(kind: ExtraFieldKind, pos: Vec3) -> impl Bundle {
    (
        Name::new("Extra Field"),
        Transform::from_translation(pos),
        ExtraField { kind },
        Visibility::Hidden,
    )
}

fn activate_field(
    cmds: &mut Commands,
    field_id: Entity,
    kind: ExtraFieldKind,
    assets: &PinballDefenseGltfAssets,
    tex: &PinballDefenseAssets,
    mats: &mut Assets<StandardMaterial>,
) {
    cmds.write_message(ExtraFieldUnlockEvent(kind));
    cmds.entity(field_id)
        .insert((
            Sensor,
            Collider::circle(FIELD_RADIUS),
            CollisionEventsEnabled,
            CollisionLayers::new(
                crate::game::events::collision::GameLayer::Map,
                crate::game::events::collision::GameLayer::Ball,
            ),
            BallCollisionPoints(50),
            LightOnCollision,
            CollidingEntities::default(),
            Visibility::Inherited,
        ))
        .with_children(|p| {
            spawn_radial(
                p,
                assets,
                Some(kind.icon(tex)),
                mats,
                field_id,
                kind.color(),
                0.,
            );
        });
    ui::progress_bar::spawn_transient_with_color(cmds, field_id, 0., kind.color());
}

pub(super) fn button_press_system(
    q_field: Query<(Entity, &CollidingEntities), (With<ExtraField>, With<Collider>)>,
    q_casings: Query<(Entity, &Transform), With<RadialProgressCasing>>,
    q_children: Query<&Children>,
    mut cmds: Commands,
) {
    for (field_id, colliding) in q_field.iter() {
        if colliding.is_empty() {
            continue;
        }
        let Some(children) = q_children.get(field_id).ok() else {
            continue;
        };
        for (casing, trans) in children.iter().filter_map(|c| q_casings.get(c).ok()) {
            let mut pressed = *trans;
            pressed.translation.z = -BUTTON_PRESS_DEPTH;
            cmds.entity(casing)
                .insert((pressed, TweenAnim::new(release_tween())));
        }
    }
}

fn release_tween() -> Tween {
    Tween::new(
        bevy::math::curve::EaseFunction::QuadraticOut,
        BUTTON_RELEASE_SECS,
        TransformPositionLens {
            start: Vec3::new(0., 0., -BUTTON_PRESS_DEPTH),
            end: Vec3::ZERO,
        },
    )
}

pub(super) fn update_fields_system(
    mut cmds: Commands,
    level: Res<LevelHub>,
    q_field: Query<(Entity, &ExtraField, Has<Collider>)>,
    assets: Res<PinballDefenseGltfAssets>,
    tex: Res<PinballDefenseAssets>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    if !level.is_changed() {
        return;
    }
    let target = (u32::from(level.level()) / 4).min(4) as usize;
    let mut kinds: Vec<Option<ExtraFieldKind>> = q_field
        .iter()
        .map(|(_, field, active)| active.then_some(field.kind()))
        .collect();
    let entities: Vec<Entity> = q_field.iter().map(|(id, _, _)| id).collect();
    while kinds.iter().filter(|k| k.is_some()).count() < target
        && let Some(i) = pick_next_inactive(&kinds)
        && let Some(&field_id) = entities.get(i)
    {
        let Some(kind) = q_field.get(field_id).ok().map(|(_, field, _)| field.kind()) else {
            break;
        };
        if let Some(slot) = kinds.get_mut(i) {
            *slot = Some(kind);
        }
        activate_field(&mut cmds, field_id, kind, &assets, &tex, &mut mats);
    }
}

pub(super) fn effect_flash_system(
    mut cmds: Commands,
    q_field: Query<(Entity, &ExtraField)>,
    mut q_light_off: Query<
        (Entity, &ChildOf, &mut Visibility),
        (With<ContactLight>, Without<FlashLight>),
    >,
    mut q_light_on: Query<(Entity, &ChildOf, &mut Visibility), With<FlashLight>>,
    effects: Res<ActiveEffects>,
    ig_time: Res<IngameTime>,
    mut prev_active: Local<[bool; 3]>,
) {
    for (i, kind) in [
        ExtraFieldKind::SlowDown,
        ExtraFieldKind::DoubleDamage,
        ExtraFieldKind::InstaKill,
    ]
    .into_iter()
    .enumerate()
    {
        let now_active = effects.is_active(**ig_time, kind);
        let Some(prev) = prev_active.get_mut(i) else {
            continue;
        };
        if now_active && !*prev {
            for (field_id, field) in q_field.iter() {
                if field.kind() != kind {
                    continue;
                }
                if let Some((light_id, _, mut visi)) = q_light_off
                    .iter_mut()
                    .find(|(_, child_of, _)| child_of.parent() == field_id)
                {
                    cmds.entity(light_id).insert(FlashLight);
                    *visi = Visibility::Inherited;
                }
            }
        } else if *prev && !now_active {
            for (field_id, field) in q_field.iter() {
                if field.kind() != kind {
                    continue;
                }
                disable_flash_light(&mut cmds, &mut q_light_on, field_id);
            }
        }
        *prev = now_active;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extra_field_pick_returns_none_when_all_active() {
        let kinds = [
            Some(ExtraFieldKind::ExtraBall),
            Some(ExtraFieldKind::SlowDown),
            Some(ExtraFieldKind::DoubleDamage),
            Some(ExtraFieldKind::InstaKill),
        ];
        assert_eq!(pick_next_inactive(&kinds), None);
    }

    #[test]
    fn extra_field_pick_returns_first_inactive() {
        let kinds = [Some(ExtraFieldKind::ExtraBall), None, None, None];
        assert_eq!(pick_next_inactive(&kinds), Some(1));
        assert_eq!(pick_next_inactive(&[None, None, None, None]), Some(0));
    }

    #[test]
    fn extra_field_lane_occupied_guard() {
        assert!(!lane_occupied(Vec3::new(1.02, 0.657, 0.), &[]));
        assert!(lane_occupied(
            Vec3::new(1.02, 0.657, 0.),
            &[Vec3::new(1.02, 0.657, 0.)]
        ));
        assert!(!lane_occupied(
            Vec3::new(1.02, 0.657, 0.),
            &[Vec3::new(0., 0., 0.)]
        ));
        assert!(!lane_occupied(
            Vec3::new(1.02, 0.657, 0.),
            &[Vec3::new(1.2, 0.657, 0.)]
        ));
    }
}
