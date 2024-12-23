use super::project_3d_to_2d_screen;
use crate::game::ball_starter::BallSpawn;
use crate::game::camera::PinballCamera;
use crate::game::flipper::FlipperType;
use crate::game::KeyboardControls;
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::prelude::default;
use bevy::window::WindowResized;

#[derive(Component)]
pub struct ControlsUi;

pub fn spawn(mut cmd: Commands, ctl: Res<KeyboardControls>, ass: Res<PinballDefenseAssets>) {
    use FieldPos::*;
    let cmd = &mut cmd;
    spawn_key(cmd, ctl.flipper_left, &ass, Invisible, "Flipper Left");
    spawn_key(cmd, ctl.flipper_right, &ass, Invisible, "Flipper Right");
    spawn_key(cmd, ctl.charge_ball_starter, &ass, Invisible, "Start");
    spawn_key(cmd, ctl.menu, &ass, TopLeft, "Menu");
    spawn_key(cmd, ctl.pause, &ass, TopRight(0), "Pause");
    spawn_key(cmd, ctl.toggle_key_ui, &ass, TopRight(1), "Toggle Keys UI");
}

pub fn despawn(mut cmds: Commands, q_ui: Query<Entity, With<ControlsUi>>) {
    for id in &q_ui {
        cmds.entity(id).despawn_recursive();
    }
}

type QKeys<'w, 's, 'a> = Query<'w, 's, (&'a mut Node, &'a UiKey)>;

pub fn keys_to_pos_system(
    q_keys: QKeys,
    controls: Res<KeyboardControls>,
    q_cam: Query<(&GlobalTransform, &Camera), (With<PinballCamera>, Changed<Transform>)>,
    q_flipper: Query<(&GlobalTransform, &FlipperType)>,
    ball_spawn: Res<BallSpawn>,
) {
    if let Ok(cam) = q_cam.get_single() {
        keys_to_pos(q_keys, controls, cam, q_flipper, ball_spawn)
    }
}

fn keys_to_pos(
    mut q_keys: QKeys,
    controls: Res<KeyboardControls>,
    cam: (&GlobalTransform, &Camera),
    q_flipper: Query<(&GlobalTransform, &FlipperType)>,
    ball_spawn: Res<BallSpawn>,
) {
    let (cam_trans, cam) = cam;
    q_flipper
        .iter()
        .for_each(|(obj_trans, f_type)| match f_type {
            FlipperType::Left => set_projected_pos(
                controls.flipper_left,
                &mut q_keys,
                obj_trans.compute_transform().translation,
                cam_trans,
                cam,
            ),
            FlipperType::Right => {
                set_projected_pos(
                    controls.flipper_right,
                    &mut q_keys,
                    obj_trans.compute_transform().translation,
                    cam_trans,
                    cam,
                );
            }
        });
    set_projected_pos(
        controls.charge_ball_starter,
        &mut q_keys,
        ball_spawn.0,
        cam_trans,
        cam,
    );
}

fn set_projected_pos(
    key: KeyCode,
    q_keys: &mut QKeys,
    obj_pos: Vec3,
    cam_trans: &GlobalTransform,
    cam: &Camera,
) {
    let (mut ui_style, _) = q_keys
        .iter_mut()
        .find(|(_, ui_key)| ui_key.0 == key)
        .unwrap_or_else(|| panic!("UI key {:?} not found", key));
    let screen_pos = project_3d_to_2d_screen(obj_pos, cam_trans, cam);
    ui_style.left = Val::Px(screen_pos.x);
    ui_style.top = Val::Px(screen_pos.y);
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum FieldPos {
    #[default]
    TopLeft,
    TopRight(u8),
    Invisible,
}

const FIELD_HEIGHT_PX: f32 = 55.;

#[derive(Component)]
pub struct UiKey(KeyCode);

fn spawn_key(
    p: &mut Commands,
    key: KeyCode,
    assets: &PinballDefenseAssets,
    pos: FieldPos,
    text: &str,
) {
    p.spawn((Name::new("UI Key"), field_node(pos), UiKey(key), ControlsUi))
        .with_children(|p| {
            p.spawn((
                Text(format!("{text} ")),
                TextFont {
                    font: assets.menu_font.clone(),
                    font_size: 52.0,
                    ..default()
                },
                TextColor(GameColor::WHITE),
            ));
            p.spawn((
                Node {
                    width: Val::Auto,
                    height: Val::Px(FIELD_HEIGHT_PX),
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::new(Val::Px(5.), Val::Px(10.), Val::Px(0.), Val::Px(5.)),
                    ..default()
                },
                BorderColor(GameColor::WHITE),
                BackgroundColor(Color::NONE),
            ))
            .with_children(|p| {
                p.spawn((
                    Text(format!("{key:?}").replace("Key", "")),
                    TextFont {
                        font: assets.menu_font.clone(),
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(GameColor::WHITE),
                ));
            });
        });
}

fn field_node(pos: FieldPos) -> Node {
    let mut style = Node {
        position_type: PositionType::Absolute,
        width: Val::Auto,
        height: Val::Px(FIELD_HEIGHT_PX),
        //display: Display::Flex,
        //flex_direction: FlexDirection::Column,
        //align_content: AlignContent::Center,
        ..default()
    };
    match pos {
        FieldPos::TopLeft => {
            style.top = Val::Px(10.);
            style.left = Val::Px(10.);
        }
        FieldPos::TopRight(pos) => {
            style.top = Val::Px(10. + (pos as f32) * (FIELD_HEIGHT_PX + 5.));
            style.right = Val::Px(10.);
        }
        FieldPos::Invisible => {
            style.top = Val::Percent(100.);
            style.left = Val::Percent(100.);
        }
    }
    style
}

pub(super) fn on_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    q_keys: QKeys,
    controls: Res<KeyboardControls>,
    q_cam: Query<(&GlobalTransform, &Camera), With<PinballCamera>>,
    q_flipper: Query<(&GlobalTransform, &FlipperType)>,
    ball_spawn: Res<BallSpawn>,
) {
    for _ in resize_reader.read() {
        if let Ok(cam) = q_cam.get_single() {
            keys_to_pos(q_keys, controls, cam, q_flipper, ball_spawn);
        }
        return;
    }
}
