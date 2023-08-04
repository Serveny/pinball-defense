use super::light::{ContactLight, LightOnCollision};
use super::progress_bar::spawn_progress_bar;
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
            Collider::cylinder(0.05, 0.06),
            Restitution {
                coefficient: 2.6,
                combine_rule: CoefficientCombineRule::Multiply,
            },
            ActiveEvents::COLLISION_EVENTS,
            TowerBase,
            LightOnCollision,
            Name::new("Tower Base"),
        ))
        .with_children(|parent| {
            parent.spawn((
                PointLightBundle {
                    transform: Transform::from_xyz(0., 0.005, 0.),
                    point_light: PointLight {
                        intensity: 0.,
                        color: Color::RED,
                        shadows_enabled: g_sett.is_shadows,
                        radius: 0.01,
                        range: 0.5,
                        ..default()
                    },
                    ..default()
                },
                ContactLight,
            ));

            spawn_progress_bar(
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
            );
        });
}
