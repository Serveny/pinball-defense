use crate::prelude::*;
use crate::settings::GraphicsSettings;

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
