use crate::game::ball_starter::BallSpawn;
use crate::game::camera::PinballCamera;
use crate::game::flipper::FlipperType;
use crate::game::KeyboardControls;
use crate::prelude::*;
use crate::utils::reflect::cast;
use crate::utils::GameColor;

#[derive(Component)]
pub struct ControlsUi;

pub fn spawn(
    mut cmds: Commands,
    controls: Res<KeyboardControls>,
    assets: Res<PinballDefenseAssets>,
) {
    for field in controls.iter_fields() {
        spawn_key(&mut cmds, &assets, cast::<KeyCode>(field));
    }
}

pub fn despawn(mut cmds: Commands, q_ui: Query<Entity, With<ControlsUi>>) {
    for id in &q_ui {
        cmds.entity(id).despawn_recursive();
    }
}

type QKeys<'w, 's, 'a> = Query<'w, 's, (&'a mut Style, &'a UiKey)>;

pub fn keys_to_pos(
    mut q_keys: QKeys,
    controls: Res<KeyboardControls>,
    q_cam: Query<&mut Transform, (With<PinballCamera>, Changed<Transform>)>,
    q_flipper: Query<(&GlobalTransform, &FlipperType)>,
    ball_spawn: Res<BallSpawn>,
    window: Query<&Window>,
) {
    if let Ok(cam_trans) = q_cam.get_single() {
        let win = window.single();
        q_flipper
            .iter()
            .for_each(|(obj_trans, f_type)| match f_type {
                FlipperType::Left => set_projected_pos(
                    controls.flipper_left,
                    &mut q_keys,
                    obj_trans.compute_transform().translation,
                    cam_trans,
                    win,
                ),
                FlipperType::Right => {
                    set_projected_pos(
                        controls.flipper_right,
                        &mut q_keys,
                        obj_trans.compute_transform().translation,
                        cam_trans,
                        win,
                    );
                }
            });
        set_projected_pos(
            controls.charge_ball_starter,
            &mut q_keys,
            ball_spawn.0,
            cam_trans,
            win,
        );
    }
}

fn set_projected_pos(
    key: KeyCode,
    q_keys: &mut QKeys,
    obj_pos: Vec3,
    cam_trans: &Transform,
    win: &Window,
) {
    let (mut ui_style, _) = q_keys
        .iter_mut()
        .find(|(_, ui_key)| ui_key.0 == key)
        .unwrap_or_else(|| panic!("UI key {:?} not found", key));

    // Compute the model matrix for the object
    let model_matrix = Mat4::from_translation(obj_pos);

    // Combine the model, view, and projection matrices
    let mvp_matrix = cam_trans.compute_matrix() * model_matrix;

    // Transform the object's position to homogeneous clip space
    let position_in_clip = mvp_matrix.mul_vec4(Vec4::new(0.0, 0.0, 0.0, 1.0));

    // Normalize to obtain normalized device coordinates
    let normalized_device_coordinates = position_in_clip / position_in_clip.w;

    // Convert normalized device coordinates to screen coordinates (in pixels)
    let screen_width = win.width();
    let screen_height = win.height();
    let screen_x = (normalized_device_coordinates.x + 1.0) * 0.5 * screen_width / 2.;
    let screen_y = (1.0 - normalized_device_coordinates.y) * 0.5 * screen_height / 2.;

    println!("Object position in pixels: ({}, {})", screen_x, screen_y);
    ui_style.left = Val::Px(screen_x);
    ui_style.top = Val::Px(screen_y);
}

#[derive(Component)]
pub struct UiKey(KeyCode);

fn spawn_key(p: &mut Commands, assets: &PinballDefenseAssets, key: KeyCode) {
    p.spawn((
        Name::new("UI Key"),
        ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(55.),
                height: Val::Px(55.),
                border: UiRect::all(Val::Px(5.0)),
                padding: UiRect::all(Val::Auto),
                ..default()
            },
            border_color: GameColor::WHITE.into(),
            background_color: Color::NONE.into(),
            ..default()
        },
        UiKey(key),
        ControlsUi,
    ))
    .with_children(|p| {
        p.spawn(TextBundle::from_section(
            format!("{key:?}"),
            TextStyle {
                font: assets.menu_font.clone(),
                font_size: 40.0,
                color: GameColor::WHITE,
            },
        ));
    });
}
