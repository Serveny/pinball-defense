use super::project_3d_to_2d_screen;
use crate::game::KeyboardControls;
use crate::game::ball_starter::BallSpawn;
use crate::game::camera::PinballCamera;
use crate::game::controls::InputKind;
use crate::game::flipper::FlipperType;
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::text::{FontSize, FontSourceTemplate};
use bevy::window::WindowResized;
use std::time::Duration;

#[derive(Component, Clone, Default)]
pub struct ControlsUi;

const HIDE_DELAY_SECS: f32 = 7.;
const FADE_DURATION_SECS: f32 = 2.;

#[derive(Resource)]
pub struct ControlsUiFade {
    timer: Timer,
}

impl Default for ControlsUiFade {
    fn default() -> Self {
        Self {
            timer: Timer::new(
                Duration::from_secs_f32(HIDE_DELAY_SECS + FADE_DURATION_SECS),
                TimerMode::Once,
            ),
        }
    }
}

impl ControlsUiFade {
    pub fn reset(&mut self) {
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

#[derive(Component, Clone, Default)]
pub(super) struct FadeableColor(Color);

pub fn spawn(mut cmds: Commands, ctl: Res<KeyboardControls>, assets: Res<PinballDefenseAssets>) {
    use FieldPos::{Invisible, TopLeft, TopRight};
    use GamepadButton::{East, LeftTrigger, RightTrigger, Start};
    let keys = [
        (KeyId::Key(ctl.flipper_left), Invisible, "Flipper Left"),
        (KeyId::Key(ctl.flipper_right), Invisible, "Flipper Right"),
        (KeyId::Key(ctl.charge_ball_starter), Invisible, "Start"),
        (KeyId::Key(ctl.menu), TopLeft, "Menu"),
        (KeyId::Key(ctl.pause), TopRight(0), "Pause"),
        (KeyId::Key(ctl.toggle_key_ui), TopRight(1), "Toggle Keys UI"),
        (KeyId::Button(LeftTrigger), Invisible, "Flipper Left"),
        (KeyId::Button(RightTrigger), Invisible, "Flipper Right"),
        (KeyId::Button(East), Invisible, "Start"),
        (KeyId::Button(Start), TopLeft, "Menu"),
        (KeyId::Button(Start), TopRight(0), "Pause"),
    ];
    for (key, pos, text) in keys {
        cmds.spawn_scene(key_scene(key, &assets, pos, text))
            .insert(UiKey(key));
    }
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

fn key_scene(key: KeyId, assets: &PinballDefenseAssets, pos: FieldPos, text: &str) -> impl Scene {
    let font = || FontSourceTemplate::Handle(assets.menu_font.clone().into());
    let (font_text, font_label) = (font(), font());
    let text = format!("{text} ");
    let label = key.label();
    let (top, left, right) = field_offsets(pos);
    bsn! {
        Name::new("UI Key")
        ControlsUi
        Node {
            position_type: PositionType::Absolute,
            height: Val::Px(FIELD_HEIGHT_PX),
            top: {top},
            left: {left},
            right: {right},
        }
        Children [
            Text({text})
            TextFont { font: {font_text}, font_size: FontSize::Px(40.) }
            TextColor({GameColor::WHITE})
            FadeableColor({GameColor::WHITE})
            --
            Node {
                height: Val::Px(FIELD_HEIGHT_PX),
                border: UiRect::all(Val::Px(4.)),
                padding: UiRect::new(Val::Px(4.), Val::Px(8.), Val::Px(0.), Val::Px(4.)),
            }
            BorderColor::from(GameColor::WHITE)
            FadeableColor({GameColor::WHITE})
            Children [
                Text({label})
                TextFont { font: {font_label}, font_size: FontSize::Px(32.) }
                TextColor({GameColor::WHITE})
                FadeableColor({GameColor::WHITE})
            ]
        ]
    }
}

fn field_offsets(pos: FieldPos) -> (Val, Val, Val) {
    match pos {
        FieldPos::TopLeft => (Val::Px(10.), Val::Px(10.), Val::Auto),
        FieldPos::TopRight(i) => (
            Val::Px(10. + f32::from(i) * (FIELD_HEIGHT_PX + 5.)),
            Val::Auto,
            Val::Px(10.),
        ),
        FieldPos::Invisible => (Val::Percent(100.), Val::Percent(100.), Val::Auto),
    }
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
