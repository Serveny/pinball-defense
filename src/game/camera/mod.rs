use self::fps::{FpsCamSettings, LookDirection};
use super::GameState;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::core_pipeline::Skybox;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};
use bevy_tweening::{Animator, EaseFunction, Lens, Tween};

mod ball;
mod dynamic;
mod fps;

pub struct PinballCameraPlugin;

impl Plugin for PinballCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CameraState>()
            .init_resource::<FpsCamSettings>()
            .add_systems(OnEnter(GameState::Init), spawn)
            .add_systems(
                Update,
                ball::follow_ball.run_if(in_state(CameraState::BallCamera)),
            )
            .add_systems(
                Update,
                (fps::on_keyboard_mouse_motion_system, fps::gamepad_input)
                    .run_if(in_state(CameraState::FpsCamera)),
            )
            .add_systems(
                Update,
                dynamic::on_ball_start_cam_system.run_if(in_state(CameraState::Dynamic)),
            );
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum CameraState {
    #[default]
    Dynamic,
    BallCamera,
    FpsCamera,
}

#[derive(Component)]
pub struct PinballCamera;

fn spawn(
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
    images: ResMut<Assets<Image>>,
    g_setting: Res<GraphicsSettings>,
) {
    let start = Transform::from_translation(START_POS).looking_at(Vec3::ZERO, Vec3::Z);
    cmds.spawn((
        Name::new("Camera"),
        Camera3dBundle {
            transform: start,
            camera: Camera {
                order: 1,
                hdr: g_setting.is_hdr,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings {
            intensity: g_setting.bloom_intensity,
            ..default()
        },
        //UiCameraConfig { show_ui: false },
        Skybox(assets.skybox.clone()),
        EnvironmentMapLight {
            diffuse_map: assets.skybox.clone(),
            specular_map: assets.skybox.clone(),
        },
        LookDirection::default(),
        PinballCamera,
        Animator::new(init_tracking_shot()),
    ));
    place_skybox(assets, images)
}

const START_POS: Vec3 = Vec3::new(2.9, 1.8, 1.9);
const LOOK_POS: Vec3 = Vec3::new(0.7, 0., 0.);
const CAM_LOW_POS: Vec3 = Vec3::new(1.9, 0., 0.9);
struct CamTransformLens {
    start: Vec3,
    end: Vec3,
    look_at_start: Vec3,
    look_at_end: Vec3,
}

impl CamTransformLens {
    pub fn new(start: Vec3, end: Vec3, look_at_start: Vec3, look_at_end: Vec3) -> Self {
        Self {
            start,
            end,
            look_at_start,
            look_at_end,
        }
    }

    fn look_static(start: Vec3, end: Vec3, look_at: Vec3) -> Self {
        Self {
            start,
            end,
            look_at_start: look_at,
            look_at_end: look_at,
        }
    }
}

impl Lens<Transform> for CamTransformLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        target.translation = self.start + (self.end - self.start) * ratio;
        let look_at = self.look_at_start + (self.look_at_end - self.look_at_start) * ratio;
        target.look_at(look_at, Vec3::Z);
    }
}

fn init_tracking_shot() -> Tween<Transform> {
    Tween::new(
        EaseFunction::CubicOut,
        std::time::Duration::from_secs(4),
        CamTransformLens::look_static(START_POS, CAM_LOW_POS, LOOK_POS),
    )
}

// NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
// so they appear as one texture. The following code reconfigures the texture as necessary.
fn place_skybox(assets: Res<PinballDefenseAssets>, mut images: ResMut<Assets<Image>>) {
    let image = images.get_mut(&assets.skybox).unwrap();
    if image.texture_descriptor.array_layer_count() == 1 {
        image.reinterpret_stacked_2d_as_array(
            image.texture_descriptor.size.height / image.texture_descriptor.size.width,
        );
        image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });
    }
}
