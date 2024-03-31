use crate::game::ball_starter::BallSpawn;
use crate::game::camera::PinballCamera;
use crate::game::flipper::FlipperType;
use crate::game::KeyboardControls;
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::prelude::default;

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

type QKeys<'w, 's, 'a> = Query<'w, 's, (&'a mut Style, &'a UiKey)>;

pub fn keys_to_pos(
    mut q_keys: QKeys,
    controls: Res<KeyboardControls>,
    q_cam: Query<(&GlobalTransform, &Camera), (With<PinballCamera>, Changed<Transform>)>,
    q_flipper: Query<(&GlobalTransform, &FlipperType)>,
    ball_spawn: Res<BallSpawn>,
) {
    if let Ok((cam_trans, cam)) = q_cam.get_single() {
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
    let screen_pos = cam.world_to_viewport(cam_trans, obj_pos);
    let Some(screen_pos) = screen_pos else {
        return;
    };
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
    p.spawn((
        Name::new("UI Key"),
        NodeBundle {
            style: field_style(pos),
            ..default()
        },
        UiKey(key),
        ControlsUi,
    ))
    .with_children(|p| {
        p.spawn(TextBundle::from_section(
            format!("{text} "),
            TextStyle {
                font: assets.menu_font.clone(),
                font_size: 52.0,
                color: GameColor::WHITE,
            },
        ));
        p.spawn((ButtonBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Px(FIELD_HEIGHT_PX),
                border: UiRect::all(Val::Px(5.0)),
                padding: UiRect::new(Val::Px(5.), Val::Px(10.), Val::Px(0.), Val::Px(5.)),
                ..default()
            },
            border_color: GameColor::WHITE.into(),
            background_color: Color::NONE.into(),
            ..default()
        },))
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    format!("{key:?}").replace("Key", ""),
                    TextStyle {
                        font: assets.menu_font.clone(),
                        font_size: 40.0,
                        color: GameColor::WHITE,
                    },
                ));
            });
    });
}

fn field_style(pos: FieldPos) -> Style {
    let mut style = Style {
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
