use super::light::{spawn_contact_light, LightOnCollision};
use super::tower_material;
use crate::game::ball::CollisionWithBallEvent;
use crate::game::events::collision::{
    collider_only_interact_with_ball, collider_only_interact_with_enemy,
};
use crate::game::progress_bar;
use crate::game::progress_bar::ProgressBarCountUpEvent;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

#[derive(Component)]
pub(super) struct TowerBase;

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

            progress_bar::spawn(
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
        Collider::cylinder(0.06, radius),
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::KINEMATIC_KINEMATIC,
        collider_only_interact_with_enemy(),
        TowerSightSensor,
    ));
}

pub(super) fn progress_system(
    mut prog_bar_ev: EventWriter<ProgressBarCountUpEvent>,
    mut ball_coll_ev: EventReader<CollisionWithBallEvent>,
    q_tower_base: Query<Entity, With<TowerBase>>,
) {
    for CollisionWithBallEvent(id, flag) in ball_coll_ev.iter() {
        if *flag != CollisionEventFlags::SENSOR && q_tower_base.contains(*id) {
            prog_bar_ev.send(ProgressBarCountUpEvent(*id, 0.05));
        }
    }
}
