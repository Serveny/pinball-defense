use super::light::{spawn_contact_light, LightOnCollision};
use super::tower_material;
use crate::prelude::*;
use crate::settings::GraphicsSettings;

#[derive(Component)]
pub struct TowerBase;

pub(super) fn spawn_tower_base(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
) {
    parent
        .spawn((
            PbrBundle {
                mesh: assets.tower_base.clone(),
                material: materials.add(tower_material()),
                ..default()
            },
            //Ccd::enabled(),
            RigidBody::KinematicPositionBased,
            ColliderDebugColor(Color::RED),
            Collider::cylinder(0.12, 0.06),
            Restitution {
                coefficient: 2.,
                combine_rule: CoefficientCombineRule::Multiply,
            },
            ActiveEvents::COLLISION_EVENTS,
            TowerBase,
            LightOnCollision,
            Name::new("Tower Base"),
        ))
        .with_children(|parent| {
            spawn_contact_light(parent, g_sett);

            crate::progress_bar::spawn(
                parent,
                assets,
                materials,
                parent.parent_entity(),
                Transform {
                    translation: Vec3::new(0.034, -0.007, 0.),
                    scale: Vec3::new(0.5, 1., 0.5),
                    ..default()
                },
                Color::RED,
                0.,
            );
        });
}
