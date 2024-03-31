use super::{EventState, GameState};
use crate::game::ball::CollisionWithBallEvent;
use crate::game::pinball_menu::PinballMenuOnSetSelectedEvent;
use crate::prelude::*;
use crate::settings::GraphicsSettings;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                flash_light_system,
                fade_out_point_light_system,
                fade_out_spot_light_system,
            )
                .run_if(in_state(GameState::Ingame)),
        )
        .add_systems(
            Update,
            (
                on_contact_light_on_system,
                on_add_flashlight_system.after(fade_out_point_light_system),
            )
                .run_if(in_state(EventState::Active)),
        );
    }
}

#[derive(Component)]
pub struct LevelUpLamp;

#[derive(Component)]
pub(super) struct ContactLight;

pub(super) fn contact_light_bundle(g_sett: &GraphicsSettings, color: Color) -> impl Bundle {
    (
        Name::new("Contact Light"),
        PointLightBundle {
            transform: Transform::from_xyz(0., 0., 0.005),
            point_light: PointLight {
                intensity: 0.,
                color,
                shadows_enabled: g_sett.is_shadows,
                radius: 0.01,
                range: 0.5,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        ContactLight,
        FadeOutLight,
    )
}

fn on_contact_light_on_system(
    mut evr: EventReader<CollisionWithBallEvent>,
    mut q_light: QueryContactLight,
    q_light_on_coll: Query<Entity, With<LightOnCollision>>,
) {
    for CollisionWithBallEvent(id, _) in evr.read() {
        if q_light_on_coll.contains(*id) {
            light_on_by_parent(*id, &mut q_light);
        }
    }
}

fn light_on_by_parent(parent_id: Entity, q_light: &mut QueryContactLight) {
    if let Some((_, mut visi, mut light)) = q_light
        .iter_mut()
        .find(|(parent, _, _)| parent_id == parent.get())
    {
        *visi = Visibility::Inherited;
        light.intensity = LIGHT_INTENSITY;
    }
}

#[derive(Component)]
pub(super) struct FlashLight;

fn on_add_flashlight_system(
    mut cmds: Commands,
    mut evr: EventReader<PinballMenuOnSetSelectedEvent>,
    mut q_light: Query<(&mut Visibility, &Parent, Entity), With<ContactLight>>,
) {
    for ev in evr.read() {
        let (mut visi, _, light_id) = q_light
            .iter_mut()
            .find(|(_, parent, _)| parent.get() == ev.0)
            .expect("Parent should have ContactLight as child");

        cmds.entity(light_id).insert(FlashLight);
        *visi = Visibility::Inherited;
    }
}

fn flash_light_system(mut q_light: Query<&mut PointLight, With<FlashLight>>, time: Res<Time>) {
    for mut light in q_light.iter_mut() {
        light.intensity = ((time.elapsed_seconds() * 16.).sin() + 1.) * LIGHT_INTENSITY * 0.5;
    }
}

pub(super) fn disable_flash_light(
    cmds: &mut Commands,
    q_light: &mut Query<(Entity, &Parent, &mut Visibility), With<FlashLight>>,
    parent_id: Entity,
) {
    let (entity, _, mut visi) = q_light
        .iter_mut()
        .find(|(_, p, _)| p.get() == parent_id)
        .expect("Here should be the selected parend 🫢");
    log!("Disable flashlight for {:?}", parent_id);
    *visi = Visibility::Hidden;
    cmds.entity(entity).remove::<FlashLight>();
}

#[derive(Component)]
pub(super) struct LightOnCollision;

const LIGHT_INTENSITY: f32 = 48000.;

type QueryContactLight<'w, 's, 'a> = Query<
    'w,
    's,
    (&'a Parent, &'a mut Visibility, &'a mut PointLight),
    (With<ContactLight>, Without<FlashLight>),
>;

#[derive(Component)]
pub(super) struct FadeOutLight;

fn fade_out_point_light_system(
    mut q_light: Query<
        (&mut Visibility, &mut PointLight),
        (With<FadeOutLight>, Without<FlashLight>),
    >,
    time: Res<Time>,
) {
    for (mut visi, mut light) in q_light.iter_mut() {
        if *visi != Visibility::Hidden {
            let time = time.delta_seconds() * LIGHT_INTENSITY;
            light.intensity -= time;
            if light.intensity <= 0. {
                light.intensity = 0.;
                *visi = Visibility::Hidden;
            }
        }
    }
}

fn fade_out_spot_light_system(
    mut q_light: Query<
        (&mut Visibility, &mut SpotLight),
        (With<FadeOutLight>, Without<FlashLight>),
    >,
    time: Res<Time>,
) {
    for (mut visi, mut light) in q_light.iter_mut() {
        if *visi != Visibility::Hidden {
            let time = time.delta_seconds() * LIGHT_INTENSITY;
            light.intensity -= time;
            if light.intensity <= 0. {
                light.intensity = 0.;
                *visi = Visibility::Hidden;
            }
        }
    }
}

#[derive(Component)]
pub(super) struct SightRadiusLight;

pub(super) fn sight_radius_light(range: f32) -> impl Bundle {
    (
        Name::new("Sight Radius Light"),
        SpotLightBundle {
            transform: Transform::from_xyz(0., 0., 1.).looking_to(Vec3::NEG_Z, Vec3::Z),
            spot_light: SpotLight {
                intensity: 18000.,
                color: Color::ANTIQUE_WHITE,
                shadows_enabled: false,
                radius: 3.,
                range: 3.,
                outer_angle: range,
                inner_angle: range,
                ..default()
            },
            visibility: Visibility::Inherited,
            ..default()
        },
        SightRadiusLight,
    )
}

#[derive(Component)]
pub struct Lamp;

pub fn spawn_lamp(
    p: &mut ChildBuilder,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
    color: Color,
    light_comp: impl Component,
) {
    p.spawn((Name::new("Lamp"), Lamp, spatial_from_pos(pos)))
        .with_children(|p| {
            p.spawn(PbrBundle {
                mesh: assets.lamp_bulb.clone(),
                material: mats.add(StandardMaterial {
                    base_color: color,
                    perceptual_roughness: 0.,
                    metallic: 0.,
                    reflectance: 0.8,
                    alpha_mode: AlphaMode::Multiply,
                    ..default()
                }),
                ..default()
            });
            p.spawn(PbrBundle {
                mesh: assets.lamp_thread.clone(),
                material: assets.lamp_thread_material.clone(),
                ..default()
            });
            p.spawn((
                PointLightBundle {
                    transform: Transform::from_xyz(0., 0., 0.035),
                    point_light: PointLight {
                        intensity: 0.,
                        color,
                        shadows_enabled: g_sett.is_shadows,
                        radius: 0.01,
                        range: 2.,
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                },
                light_comp,
            ));
        });
}
