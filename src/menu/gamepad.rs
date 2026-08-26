use super::actions::MenuAction;
use super::tools::menu_btn::MenuButtonData;
use crate::prelude::*;
use bevy::input_focus::InputFocus;
use bevy::math::CompassOctant;
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigator;

pub fn navigation_system(
    gamepads: Query<&Gamepad>,
    mut navigator: AutoDirectionalNavigator,
) {
    let mut direction = None;
    for gamepad in &gamepads {
        if gamepad.just_pressed(GamepadButton::DPadUp) {
            direction = Some(CompassOctant::North);
        } else if gamepad.just_pressed(GamepadButton::DPadDown) {
            direction = Some(CompassOctant::South);
        } else if gamepad.just_pressed(GamepadButton::DPadLeft) {
            direction = Some(CompassOctant::West);
        } else if gamepad.just_pressed(GamepadButton::DPadRight) {
            direction = Some(CompassOctant::East);
        }
    }
    if let Some(dir) = direction {
        let _ = navigator.navigate(dir);
    }
}

pub fn activate_system(
    gamepads: Query<&Gamepad>,
    focus: Res<InputFocus>,
    q_btn: Query<&MenuButtonData, With<Button>>,
    mut action_ev: MessageWriter<MenuAction>,
) {
    let pressed = gamepads
        .iter()
        .any(|g| g.just_pressed(GamepadButton::South));
    if !pressed {
        return;
    }
    if let Some(focused) = focus.get()
        && let Ok(data) = q_btn.get(focused)
    {
        action_ev.write(data.action.clone());
    }
}
