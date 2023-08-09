use super::light::{spawn_contact_light, LightOnCollision};
use super::tower_material;
use crate::events::collision::{
    collider_only_interact_with_ball, collider_only_interact_with_enemy,
};
use crate::prelude::*;
use crate::settings::GraphicsSettings;

#[derive(Component)]
pub struct TowerBase;

pub(super) fn spawn_tower_base(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    sight_radius: f32,
) {
    parent
        .spawn((
            PbrBundle {
                mesh: assets.tower_base.clone(),
                material: materials.add(tower_material()),
                ..default()
            },
            RigidBody::KinematicPositionBased,
            Restitution {
                coefficient: 2.,
                combine_rule: CoefficientCombineRule::Multiply,
            },
            ActiveEvents::COLLISION_EVENTS,
            ColliderDebugColor(Color::RED),
            Collider::cylinder(0.12, 0.06),
            collider_only_interact_with_ball(),
            TowerBase,
            LightOnCollision,
            Name::new("Tower Base"),
        ))
        .with_children(|parent| {
            spawn_contact_light(parent, g_sett, Color::RED);

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

    spawn_tower_sight_sensor(parent, sight_radius);
}

#[derive(Component)]
pub struct TowerSightSensor;

fn spawn_tower_sight_sensor(parent: &mut ChildBuilder, radius: f32) {
    parent.spawn((
        PbrBundle { ..default() },
        Sensor,
        RigidBody::KinematicPositionBased,
        ColliderDebugColor(Color::ORANGE),
        Collider::cylinder(0.1, radius),
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::KINEMATIC_KINEMATIC,
        collider_only_interact_with_enemy(),
        TowerSightSensor,
    ));
}
