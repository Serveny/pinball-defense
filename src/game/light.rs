use super::level::LevelUpEvent;
use super::world::PinballWorld;
use super::{EventState, GameState};
use crate::game::ball::CollisionWithBallEvent;
use crate::game::pinball_menu::PinballMenuOnSetSelectedEvent;
use crate::generated::world_1::light_posis::level_up_light_posis;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use bevy::time::common_conditions::on_fixed_timer;
use std::time::Duration;

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
                on_level_up_light_system,
                on_contact_light_on_system,
                on_add_flashlight_system.after(fade_out_point_light_system),
            )
                .run_if(in_state(EventState::Active)),
        )
        .add_systems(
            Update,
            (light_next).run_if(
                in_state(GameState::Ingame).and_then(on_fixed_timer(Duration::from_secs_f32(0.07))),
            ),
        );
    }
}

#[derive(Component)]
pub struct LevelUpLight(usize);

pub fn spawn_level_up_lights(parent: &mut ChildBuilder, g_sett: &GraphicsSettings) {
    for (i, trans) in level_up_light_posis().iter().enumerate() {
        parent.spawn(level_up_light(*trans, g_sett, i));
    }
}
fn level_up_light(transform: Transform, g_sett: &GraphicsSettings, i: usize) -> impl Bundle {
    (
        Name::new("Level Up Light"),
        SpotLightBundle {
            transform,
            spot_light: SpotLight {
                intensity: LIGHT_INTENSITY,
                color: Color::BISQUE,
                shadows_enabled: g_sett.is_shadows,
                range: 2.,
                radius: 1.,
                inner_angle: 0.2,
                outer_angle: 1.3,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        LevelUpLight(i),
        FadeOutLight,
    )
}

#[derive(Component, Default)]
pub struct LevelUpLightAnimation(usize);

fn on_level_up_light_system(
    mut cmds: Commands,
    mut level_up_ev: EventReader<LevelUpEvent>,
    q_pb_world: Query<Entity, With<PinballWorld>>,
) {
    for _ in level_up_ev.iter() {
        cmds.entity(q_pb_world.single())
            .insert(LevelUpLightAnimation::default());
    }
}

fn light_next(
    mut cmds: Commands,
    mut q_anim: Query<(Entity, &mut LevelUpLightAnimation)>,
    mut q_level_light: Query<(&mut Visibility, &mut SpotLight, &LevelUpLight)>,
) {
    for (anim_id, mut anim) in q_anim.iter_mut() {
        if let Some((mut visi, mut spot, _)) = q_level_light
            .iter_mut()
            .find(|(_, _, lvl_up_light)| lvl_up_light.0 == anim.0)
        {
            *visi = Visibility::Inherited;
            spot.intensity = LIGHT_INTENSITY;
            anim.0 += 1;
        } else {
            cmds.entity(anim_id).remove::<LevelUpLightAnimation>();
        }
    }
}

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
    mut ball_coll_ev: EventReader<CollisionWithBallEvent>,
    mut q_light: QueryContactLight,
    q_light_on_coll: Query<Entity, With<LightOnCollision>>,
) {
    for CollisionWithBallEvent(id, _) in ball_coll_ev.iter() {
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
    mut pb_menu_open_ev: EventReader<PinballMenuOnSetSelectedEvent>,
    mut q_light: Query<(&mut Visibility, &Parent, Entity), With<ContactLight>>,
) {
    for ev in pb_menu_open_ev.iter() {
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

const LIGHT_INTENSITY: f32 = 48.;

type QueryContactLight<'w, 's, 'a> = Query<
    'w,
    's,
    (&'a Parent, &'a mut Visibility, &'a mut PointLight),
    (With<ContactLight>, Without<FlashLight>),
>;

#[derive(Component)]
pub(super) struct FadeOutLight;

#[allow(clippy::type_complexity)]
fn fade_out_point_light_system(
    mut q_light: Query<
        (&mut Visibility, &mut PointLight),
        (With<FadeOutLight>, Without<FlashLight>),
    >,
    time: Res<Time>,
) {
    for (mut visi, mut light) in q_light.iter_mut() {
        if *visi != Visibility::Hidden {
            let time = time.delta_seconds() * 64.;
            light.intensity -= time;
            if light.intensity <= 0. {
                light.intensity = 0.;
                *visi = Visibility::Hidden;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn fade_out_spot_light_system(
    mut q_light: Query<
        (&mut Visibility, &mut SpotLight),
        (With<FadeOutLight>, Without<FlashLight>),
    >,
    time: Res<Time>,
) {
    for (mut visi, mut light) in q_light.iter_mut() {
        if *visi != Visibility::Hidden {
            let time = time.delta_seconds() * 64.;
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
                intensity: 18.,
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
