use crate::events::collision::ContactLightOnEvent;
use crate::prelude::*;

#[derive(Component)]
pub struct ContactLight;

pub(super) fn add_flash_light(
    cmds: &mut Commands,
    q_light: &Query<(&Parent, Entity), With<ContactLight>>,
    parent_id: Entity,
) {
    cmds.entity(
        q_light
            .iter()
            .find_map(|(parent, light_id)| if_true!(parent.get() == parent_id, light_id))
            .expect("Parent should have ContactLight as child"),
    )
    .insert(FlashLight);
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
    mut evs: EventReader<ContactLightOnEvent>,
    mut q_light: Query<(&mut PointLight, &Parent), With<ContactLight>>,
) {
    for ev in evs.iter() {
        light_on_by_parent(ev.0, &mut q_light);
    }
}

#[derive(Component)]
pub struct LightOnCollision;

const LIGHT_INTENSITY: f32 = 48.;

pub(super) fn light_off_system(
    mut q_light: Query<&mut PointLight, With<ContactLight>>,
    time: Res<Time>,
) {
    for mut light in q_light.iter_mut() {
        let time = time.delta_seconds() * 64.;
        light.intensity = (light.intensity - time).clamp(0., LIGHT_INTENSITY);
    }
}

pub(super) fn disable_light(
    cmds: &mut Commands,
    q_light: &mut Query<(Entity, &Parent, &mut PointLight), With<ContactLight>>,
    parent_id: Entity,
) {
    let (entity, _, mut pl) = q_light
        .iter_mut()
        .find(|(_, p, _)| p.get() == parent_id)
        .expect("Here should be the selected parend 🫢");
    pl.intensity = 0.;
    cmds.entity(entity).remove::<FlashLight>();
}

pub(super) fn light_on_by_parent(
    parent_id: Entity,
    q_light: &mut Query<(&mut PointLight, &Parent), With<ContactLight>>,
) {
    if let Some((mut light, _)) = q_light
        .iter_mut()
        .find(|(_, parent)| parent_id == parent.get())
    {
        light.intensity = LIGHT_INTENSITY;
    }
}
