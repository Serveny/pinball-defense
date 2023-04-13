use crate::prelude::*;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((light_on_contact_system, rotate_tower_head_system));
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
                        intensity: 48.,
                        color: Color::RED,
                        shadows_enabled: true,
                        radius: 0.01,
                        range: 0.5,
                        ..default()
                    },
                    visibility: Visibility::Hidden,
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

pub fn light_on_contact_system(
    mut col_events: EventReader<CollisionEvent>,
    q_tower: Query<&Children, With<TowerBase>>,
    mut q_light: Query<&mut Visibility, With<TowerContactLight>>,
) {
    for col_ev in col_events.iter() {
        match col_ev {
            CollisionEvent::Started(tower_base_id, _, _) => {
                if let Ok(tower) = q_tower.get(*tower_base_id) {
                    set_light(Visibility::Visible, tower, &mut q_light)
                }
            }
            CollisionEvent::Stopped(tower_base_id, _, _) => {
                if let Ok(tower) = q_tower.get(*tower_base_id) {
                    set_light(Visibility::Hidden, tower, &mut q_light)
                }
            }
        }
    }
}

fn set_light(
    visibility: Visibility,
    children: &Children,
    q_light: &mut Query<&mut Visibility, With<TowerContactLight>>,
) {
    for child in children {
        if let Ok(mut visi) = q_light.get_mut(*child) {
            *visi = visibility
        }
    }
}

fn rotate_tower_head_system(
    time: Res<Time>,
    mut q_heads: Query<(&mut Transform), With<MicrowaveTowerHead>>,
) {
    for mut trans in q_heads.iter_mut() {
        trans.rotate(Quat::from_rotation_y(time.delta_seconds()));
    }
}
