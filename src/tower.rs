use crate::collision_handler::TowerBaseCollisionStartEvent;
use crate::prelude::*;
use crate::GameState;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                light_on_contact_system,
                rotate_tower_head_system,
                light_off_system,
            )
                .in_set(OnUpdate(GameState::Ingame)),
        );
    }
}

#[derive(Component)]
pub struct TowerBase;

#[derive(Component)]
pub struct TowerContactLight;

#[derive(Component)]
pub struct MicrowaveTowerHead;

pub fn spawn_tower_base(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    pos: Vec3,
) {
    parent
        .spawn((
            PbrBundle {
                mesh: assets.tower_base.clone(),
                material: materials.add(StandardMaterial {
                    base_color: Color::BEIGE,
                    perceptual_roughness: 0.5,
                    metallic: 0.5,
                    reflectance: 0.5,
                    ..default()
                }),
                transform: Transform::from_translation(pos),
                ..default()
            },
            //Ccd::enabled(),
            RigidBody::KinematicPositionBased,
            ColliderDebugColor(Color::RED),
            Collider::cylinder(0.025, 0.05),
            Restitution::coefficient(1.),
            ActiveEvents::COLLISION_EVENTS,
        ))
        .insert(TowerBase)
        .with_children(|parent| {
            parent
                .spawn(PointLightBundle {
                    transform: Transform::from_xyz(0., 0.005, 0.),
                    point_light: PointLight {
                        intensity: 0.,
                        color: Color::RED,
                        shadows_enabled: true,
                        radius: 0.01,
                        range: 0.5,
                        ..default()
                    },
                    ..default()
                })
                .insert(TowerContactLight);
            parent
                .spawn(PbrBundle {
                    mesh: assets.tower_top_microwave.clone(),
                    material: materials.add(StandardMaterial {
                        base_color: Color::BEIGE,
                        perceptual_roughness: 0.5,
                        metallic: 0.5,
                        reflectance: 0.5,
                        ..default()
                    }),
                    transform: Transform::from_xyz(0., 0.04, 0.),
                    ..default()
                })
                .insert(MicrowaveTowerHead);
        });
}

const LIGHT_INTENSITY: f32 = 48.;

fn light_on_contact_system(
    mut evs: EventReader<TowerBaseCollisionStartEvent>,
    mut q_light: Query<(&mut PointLight, &Parent), With<TowerContactLight>>,
) {
    for ev in evs.iter() {
        for (mut light, parent) in q_light.iter_mut() {
            if ev.0 == parent.get() {
                light.intensity = LIGHT_INTENSITY;
                break;
            }
        }
    }
}

fn light_off_system(mut q_light: Query<&mut PointLight, With<TowerContactLight>>, time: Res<Time>) {
    for mut light in q_light.iter_mut() {
        let time = time.delta_seconds() * 64.;
        light.intensity = (light.intensity - time).clamp(0., LIGHT_INTENSITY);
    }
}

fn rotate_tower_head_system(
    time: Res<Time>,
    mut q_heads: Query<&mut Transform, With<MicrowaveTowerHead>>,
) {
    for mut trans in q_heads.iter_mut() {
        trans.rotate(Quat::from_rotation_y(time.delta_seconds()));
    }
}
