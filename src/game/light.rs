use super::level::LevelUpEvent;
use super::world::PinballWorld;
use super::GameState;
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
            (spawn_animation_system).run_if(in_state(GameState::Ingame)),
        )
        .add_systems(
            Update,
            (light_next).run_if(
                in_state(GameState::Ingame).and_then(on_fixed_timer(Duration::from_secs_f32(0.07))),
            ),
        );
    }
}
pub fn spawn_level_up_lights(parent: &mut ChildBuilder, g_sett: &GraphicsSettings) {
    for (i, trans) in level_up_light_posis().iter().enumerate() {
        parent.spawn(level_up_light(*trans, g_sett, i));
    }
}

#[derive(Component)]
pub struct LevelUpLight(usize);

fn level_up_light(transform: Transform, g_sett: &GraphicsSettings, i: usize) -> impl Bundle {
    (
        SpotLightBundle {
            transform,
            spot_light: SpotLight {
                intensity: 20., // lumens - roughly a 100W non-halogen incandescent bulb
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
    )
}

#[derive(Component, Default)]
pub struct LevelUpLightAnimation(usize);

fn spawn_animation_system(
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
    mut q_level_light: Query<(&mut Visibility, &LevelUpLight)>,
) {
    for (anim_id, mut anim) in q_anim.iter_mut() {
        if let Some(mut light) = q_level_light
            .iter_mut()
            .find(|(_, light)| light.0 == anim.0)
        {
            *light.0 = Visibility::Hidden;
        }
        anim.0 += 1;
        if let Some(mut light) = q_level_light
            .iter_mut()
            .find(|(_, light)| light.0 == anim.0)
        {
            *light.0 = Visibility::Inherited;
        } else {
            cmds.entity(anim_id).remove::<LevelUpLightAnimation>();
        }
    }
}
