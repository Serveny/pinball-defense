use super::{analog_counter::AnalogCounterSetToEvent, GameState};
use crate::prelude::*;
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    time::common_conditions::on_timer,
};
use std::{f32::consts::PI, time::Duration};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Points>()
            .init_resource::<Level>()
            .add_systems(
                Update,
                (
                    count_up_points_test_system,
                    level_up_system,
                    update_analog_counter_system,
                )
                    .run_if(in_state(GameState::Ingame).and_then(on_timer(Duration::from_secs(1)))),
            );
    }
}

#[derive(Resource, Default, Reflect)]

struct Points(u32);

#[derive(Resource, Default, Reflect)]
struct Level(u16);

#[derive(Component)]
struct PointDisplay;

const SIZE: UVec2 = UVec2::new(224, 116);

pub fn spawn_point_display(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
    assets: &PinballDefenseAssets,
) -> Handle<Image> {
    let size = Extent3d {
        width: SIZE.x,
        height: SIZE.y,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // This material has the texture that has been rendered.
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        reflectance: 0.2,
        unlit: false,
        ..default()
    });

    // Main pass cube, with material containing the rendered first pass texture.
    parent.spawn((
        PbrBundle {
            mesh: assets.world_1_point_display.clone(),
            material: material_handle,
            // Do not know, how to rotate the generated texture, so I rotate the object
            transform: Transform::from_rotation(Quat::from_rotation_y(PI / 2.))
                .with_translation(Vec3::new(0.98, 0.051, 0.56)),
            ..default()
        },
        PointDisplay,
    ));

    image_handle
}

const RENDER_LAYER: RenderLayers = RenderLayers::layer(2);

#[derive(Component)]
struct PointDisplayText;

pub fn spawn_point_display_ui_and_cam(
    cmds: &mut Commands,
    assets: &PinballDefenseAssets,
    image_handle: Handle<Image>,
) {
    cmds.spawn((NodeBundle {
        style: Style {
            border: UiRect::all(Val::Px(10.)),
            width: Val::Px(SIZE.x as f32),
            height: Val::Px(SIZE.y as f32),
            ..default()
        },
        ..default()
    },))
        .with_children(|parent| {
            let text = TextBundle::from_section(
                format!("Points: {}\nLevel: {}\n\nUpgrade ready", 0, 0),
                TextStyle {
                    font: assets.digital_font.clone(),
                    font_size: 12.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Left);
            parent.spawn((
                Name::new("Points Display Text Bundle"),
                text,
                RENDER_LAYER,
                PointDisplayText,
            ));
        });
    // The cube that will be rendered to the texture.
    cmds.spawn((
        Name::new("Points Display Texture Camera"),
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
            },
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        RENDER_LAYER,
    ));
}

fn update_display_system(
    mut q_text: Query<&mut Text, With<PointDisplayText>>,
    points: Res<Points>,
    level: Res<Level>,
) {
    if points.is_changed() || level.is_changed() {
        for mut text in q_text.iter_mut() {
            text.sections[0].value = format!("Points: {}\n\nLevel: {}\n", points.0, level.0);
        }
    }
}

// WIP
fn level_up_system(points: Res<Points>, mut level: ResMut<Level>) {
    if points.is_changed() {
        let new_level = (points.0 / 1000) as u16 + 1;
        if new_level != level.0 {
            level.0 = new_level;
        }
    }
}

fn count_up_points_test_system(mut points: ResMut<Points>) {
    points.0 += 100;
}

fn update_analog_counter_system(
    points: Res<Points>,
    mut ac_set_ev: EventWriter<AnalogCounterSetToEvent>,
) {
    if points.is_changed() {
        ac_set_ev.send(AnalogCounterSetToEvent(points.0));
    }
}
