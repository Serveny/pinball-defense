use super::project_3d_to_2d_screen;
use crate::game::KeyboardControls;
use crate::game::ball_starter::BallSpawn;
use crate::game::camera::PinballCamera;
use crate::game::controls::InputKind;
use crate::game::flipper::FlipperType;
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::prelude::default;
use bevy::text::FontSize;
use bevy::window::WindowResized;
use std::time::Duration;

#[derive(Component)]
pub struct ControlsUi;

const HIDE_DELAY_SECS: f32 = 7.;
const FADE_DURATION_SECS: f32 = 2.;

#[derive(Resource)]
pub struct ControlsUiFade {
    timer: Timer,
    started: bool,
}

impl Default for ControlsUiFade {
    fn default() -> Self {
        Self {
            timer: Timer::new(
                Duration::from_secs_f32(HIDE_DELAY_SECS + FADE_DURATION_SECS),
                TimerMode::Once,
            ),
            started: false,
        }
    }
}

impl ControlsUiFade {
    pub fn reset(&mut self) {
        self.started = false;
        self.timer.reset();
    }

    fn start(&mut self) {
        self.started = true;
        self.timer.reset();
    }

    fn alpha(&self) -> f32 {
        let elapsed = self.timer.elapsed_secs();
        if elapsed <= HIDE_DELAY_SECS {
            1.
        } else {
            let fade_t = (elapsed - HIDE_DELAY_SECS) / FADE_DURATION_SECS;
            (1. - fade_t).max(0.)
        }
    }

    fn finished(&self) -> bool {
        Timer::just_finished(&self.timer)
    }
}

#[derive(Component)]
pub(super) struct FadeableColor(Color);

pub fn spawn(mut cmd: Commands, ctl: Res<KeyboardControls>, ass: Res<PinballDefenseAssets>) {
    use FieldPos::{Invisible, TopLeft, TopRight};
    let cmd = &mut cmd;
    spawn_key(
        cmd,
        KeyId::Key(ctl.flipper_left),
        &ass,
        Invisible,
        "Flipper Left",
    );
    spawn_key(
        cmd,
        KeyId::Key(ctl.flipper_right),
        &ass,
        Invisible,
        "Flipper Right",
    );
    spawn_key(
        cmd,
        KeyId::Key(ctl.charge_ball_starter),
        &ass,
        Invisible,
        "Start",
    );
    spawn_key(cmd, KeyId::Key(ctl.menu), &ass, TopLeft, "Menu");
    spawn_key(cmd, KeyId::Key(ctl.pause), &ass, TopRight(0), "Pause");
    spawn_key(
        cmd,
        KeyId::Key(ctl.toggle_key_ui),
        &ass,
        TopRight(1),
        "Toggle Keys UI",
    );

    spawn_key(
        cmd,
        KeyId::Button(GamepadButton::LeftTrigger),
        &ass,
        Invisible,
        "Flipper Left",
    );
    spawn_key(
        cmd,
        KeyId::Button(GamepadButton::RightTrigger),
        &ass,
        Invisible,
        "Flipper Right",
    );
    spawn_key(
        cmd,
        KeyId::Button(GamepadButton::East),
        &ass,
        Invisible,
        "Start",
    );
    spawn_key(
        cmd,
        KeyId::Button(GamepadButton::Start),
        &ass,
        TopLeft,
        "Menu",
    );
    spawn_key(
        cmd,
        KeyId::Button(GamepadButton::Start),
        &ass,
        TopRight(0),
        "Pause",
    );
}

pub fn despawn(mut cmds: Commands, q_ui: Query<Entity, With<ControlsUi>>) {
    for id in &q_ui {
        cmds.entity(id).despawn();
    }
}

type QKeys<'w, 's, 'a> = Query<'w, 's, (&'a mut Node, &'a ComputedNode, &'a UiKey)>;

pub fn keys_to_pos_system(
    q_keys: QKeys,
    controls: Res<KeyboardControls>,
    kind: Res<InputKind>,
    q_cam: Query<(&GlobalTransform, &Camera), (With<PinballCamera>, Changed<Transform>)>,
    q_flipper: Query<(&GlobalTransform, &FlipperType)>,
    ball_spawn: Res<BallSpawn>,
) {
    if let Ok(cam) = q_cam.single() {
        keys_to_pos(q_keys, controls, *kind, cam, q_flipper, ball_spawn);
    }
}

pub(super) fn update_keys_pos_system(
    q_keys: QKeys,
    controls: Res<KeyboardControls>,
    kind: Res<InputKind>,
    q_cam: Query<(&GlobalTransform, &Camera), With<PinballCamera>>,
    q_flipper: Query<(&GlobalTransform, &FlipperType)>,
    ball_spawn: Res<BallSpawn>,
) {
    if let Ok(cam) = q_cam.single() {
        keys_to_pos(q_keys, controls, *kind, cam, q_flipper, ball_spawn);
    }
}

fn keys_to_pos(
    mut q_keys: QKeys,
    controls: Res<KeyboardControls>,
    kind: InputKind,
    cam: (&GlobalTransform, &Camera),
    q_flipper: Query<(&GlobalTransform, &FlipperType)>,
    ball_spawn: Res<BallSpawn>,
) {
    let (cam_trans, cam) = cam;
    let (fl_left, fl_right, start) = match kind {
        InputKind::Keyboard => (
            KeyId::Key(controls.flipper_left),
            KeyId::Key(controls.flipper_right),
            KeyId::Key(controls.charge_ball_starter),
        ),
        InputKind::Gamepad => (
            KeyId::Button(GamepadButton::LeftTrigger),
            KeyId::Button(GamepadButton::RightTrigger),
            KeyId::Button(GamepadButton::East),
        ),
    };
    q_flipper
        .iter()
        .for_each(|(obj_trans, f_type)| match f_type {
            FlipperType::Left => set_projected_pos(
                fl_left,
                &mut q_keys,
                obj_trans.compute_transform().translation,
                cam_trans,
                cam,
            ),
            FlipperType::Right => {
                set_projected_pos(
                    fl_right,
                    &mut q_keys,
                    obj_trans.compute_transform().translation,
                    cam_trans,
                    cam,
                );
            }
        });
    set_projected_pos(start, &mut q_keys, ball_spawn.0, cam_trans, cam);
}

fn set_projected_pos(
    key: KeyId,
    q_keys: &mut QKeys,
    obj_pos: Vec3,
    cam_trans: &GlobalTransform,
    cam: &Camera,
) {
    let Some((mut ui_style, computed, _)) =
        q_keys.iter_mut().find(|(_, _, ui_key)| ui_key.0 == key)
    else {
        debug!("UI key {key:?} not found");
        return;
    };
    let screen_pos = project_3d_to_2d_screen(obj_pos, cam_trans, cam);
    let half = computed.size() * computed.inverse_scale_factor() * 0.5;
    ui_style.left = Val::Px(screen_pos.x - half.x);
    ui_style.top = Val::Px(screen_pos.y - half.y);
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum FieldPos {
    #[default]
    TopLeft,
    TopRight(u8),
    Invisible,
}

const FIELD_HEIGHT_PX: f32 = 42.;

#[derive(Component, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct UiKey(KeyId);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum KeyId {
    Key(KeyCode),
    Button(GamepadButton),
}

impl KeyId {
    fn label(self) -> String {
        match self {
            KeyId::Key(k) => format!("{k:?}").replace("Key", ""),
            KeyId::Button(b) => format!("{b:?}"),
        }
    }
}

fn spawn_key(
    p: &mut Commands,
    key: KeyId,
    assets: &PinballDefenseAssets,
    pos: FieldPos,
    text: &str,
) {
    p.spawn((Name::new("UI Key"), field_node(pos), UiKey(key), ControlsUi))
        .with_children(|p| {
            p.spawn((
                Text(format!("{text} ")),
                TextFont {
                    font: assets.menu_font.clone().into(),
                    font_size: FontSize::Px(40.0),
                    ..default()
                },
                TextColor(GameColor::WHITE),
                FadeableColor(GameColor::WHITE),
            ));
            p.spawn((
                Node {
                    width: Val::Auto,
                    height: Val::Px(FIELD_HEIGHT_PX),
                    border: UiRect::all(Val::Px(4.0)),
                    padding: UiRect::new(Val::Px(4.), Val::Px(8.), Val::Px(0.), Val::Px(4.)),
                    ..default()
                },
                BorderColor::from(GameColor::WHITE),
                BackgroundColor(Color::NONE),
                FadeableColor(GameColor::WHITE),
            ))
            .with_children(|p| {
                p.spawn((
                    Text(key.label()),
                    TextFont {
                        font: assets.menu_font.clone().into(),
                        font_size: FontSize::Px(32.0),
                        ..default()
                    },
                    TextColor(GameColor::WHITE),
                    FadeableColor(GameColor::WHITE),
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
            style.top = Val::Px(10. + f32::from(pos) * (FIELD_HEIGHT_PX + 5.));
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
    mut resize_reader: MessageReader<WindowResized>,
    q_keys: QKeys,
    controls: Res<KeyboardControls>,
    kind: Res<InputKind>,
    q_cam: Query<(&GlobalTransform, &Camera), With<PinballCamera>>,
    q_flipper: Query<(&GlobalTransform, &FlipperType)>,
    ball_spawn: Res<BallSpawn>,
) {
    if resize_reader.read().next().is_some()
        && let Ok(cam) = q_cam.single()
    {
        keys_to_pos(q_keys, controls, *kind, cam, q_flipper, ball_spawn);
    }
}

pub(super) fn switch_input_kind_system(
    kind: Res<InputKind>,
    mut q_keys: Query<(&UiKey, &mut Node)>,
) {
    if !kind.is_changed() {
        return;
    }
    for (ui_key, mut node) in q_keys.iter_mut() {
        let matches = matches!(
            (ui_key.0, *kind),
            (KeyId::Key(_), InputKind::Keyboard) | (KeyId::Button(_), InputKind::Gamepad)
        );
        node.display = if matches {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub(super) fn auto_hide_system(
    mut fade: ResMut<ControlsUiFade>,
    mut q_fadeable: Query<(&FadeableColor, &mut TextColor)>,
    mut q_border: Query<(&FadeableColor, &mut BorderColor)>,
    time: Res<Time>,
    mut set_ui_state: ResMut<NextState<crate::game::ui::UiState>>,
) {
    if !fade.started {
        fade.start();
    }
    if !fade.started {
        return;
    }
    fade.timer.tick(time.delta());
    let alpha = fade.alpha();
    for (base, mut color) in q_fadeable.iter_mut() {
        color.0 = base.0.with_alpha(base.0.alpha() * alpha);
    }
    for (base, mut border) in q_border.iter_mut() {
        *border = BorderColor::from(base.0.with_alpha(base.0.alpha() * alpha));
    }
    if fade.finished() {
        set_ui_state.set(crate::game::ui::UiState::None);
    }
}
