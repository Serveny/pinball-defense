use crate::game::ball::CollisionWithBallEvent;
use crate::game::pinball_menu::PinballMenuOnSetSelectedEvent;
use crate::prelude::*;
use crate::settings::GraphicsSettings;

#[derive(Component)]
pub(super) struct ContactLight;

pub(super) fn contact_light_bundle(g_sett: &GraphicsSettings, color: Color) -> impl Bundle {
    (
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
    )
}

pub(super) fn add_flashlight_system(
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

#[derive(Component)]
pub(super) struct FlashLight;

pub(super) fn flash_light_system(
    mut q_light: Query<&mut PointLight, With<FlashLight>>,
    time: Res<Time>,
) {
    for mut light in q_light.iter_mut() {
        light.intensity = ((time.elapsed_seconds() * 16.).sin() + 1.) * LIGHT_INTENSITY * 0.5;
    }
}

pub(super) fn contact_light_on_system(
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

#[derive(Component)]
pub(super) struct LightOnCollision;

const LIGHT_INTENSITY: f32 = 48.;

type QueryContactLight<'w, 's, 'a> = Query<
    'w,
    's,
    (
        &'a Parent,
        &'a mut Visibility,
        &'a mut PointLight,
        &'a Parent,
    ),
    (With<ContactLight>, Without<FlashLight>),
>;

pub(super) fn disable_contact_light_system(mut q_light: QueryContactLight, time: Res<Time>) {
    for (_, mut visi, mut light, _) in q_light.iter_mut() {
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

pub(super) fn light_on_by_parent(parent_id: Entity, q_light: &mut QueryContactLight) {
    if let Some((_, mut visi, mut light, _)) = q_light
        .iter_mut()
        .find(|(_, _, _, parent)| parent_id == parent.get())
    {
        *visi = Visibility::Inherited;
        light.intensity = LIGHT_INTENSITY;
    }
}
