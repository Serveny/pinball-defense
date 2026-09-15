use super::actions::MenuAction;
use super::tools::menu_btn::MenuButtonData;
use crate::prelude::*;
use bevy::input_focus::{FocusCause, InputFocus};
use bevy::math::CompassOctant;
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigation;
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigator;
use bevy::ui_widgets::{
    Checkbox, ScrollIntoView, SetSliderValue, Slider, SliderValueChange, ToggleChecked,
};

pub fn navigation_system(
    gamepads: Query<&Gamepad>,
    keys: Res<ButtonInput<KeyCode>>,
    q_slider: Query<(), With<Slider>>,
    q_nav: Query<Entity, With<AutoDirectionalNavigation>>,
    mut navigator: AutoDirectionalNavigator,
    mut commands: Commands,
) {
    let focused = navigator.input_focus();
    let on_slider = focused.is_some_and(|e| q_slider.contains(e));

    let mut direction = None;
    for gamepad in &gamepads {
        if gamepad.just_pressed(GamepadButton::DPadUp) {
            direction = Some(CompassOctant::North);
        } else if gamepad.just_pressed(GamepadButton::DPadDown) {
            direction = Some(CompassOctant::South);
        } else if on_slider
            && gamepad.just_pressed(GamepadButton::DPadLeft)
            && let Some(e) = focused
        {
            commands.trigger(SetSliderValue {
                entity: e,
                change: SliderValueChange::RelativeStep(-1.),
            });
        } else if on_slider
            && gamepad.just_pressed(GamepadButton::DPadRight)
            && let Some(e) = focused
        {
            commands.trigger(SetSliderValue {
                entity: e,
                change: SliderValueChange::RelativeStep(1.),
            });
        }
    }

    let mut slider_step = None;
    if keys.just_pressed(KeyCode::KeyW) {
        direction = Some(CompassOctant::North);
    } else if keys.just_pressed(KeyCode::KeyS) {
        direction = Some(CompassOctant::South);
    } else if keys.just_pressed(KeyCode::ArrowUp) {
        direction = Some(CompassOctant::North);
    } else if keys.just_pressed(KeyCode::ArrowDown) {
        direction = Some(CompassOctant::South);
    } else if keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::ArrowLeft) {
        if on_slider {
            slider_step = Some(-1.);
        } else {
            direction = Some(CompassOctant::West);
        }
    } else if keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::ArrowRight) {
        if on_slider {
            slider_step = Some(1.);
        } else {
            direction = Some(CompassOctant::East);
        }
    }

    let Some(dir) = direction else {
        if let Some(step) = slider_step
            && let Some(e) = focused
        {
            commands.trigger(SetSliderValue {
                entity: e,
                change: SliderValueChange::RelativeStep(step),
            });
        }
        return;
    };

    // A menu change can despawn the previously focused widget; pick a fresh
    // target so the first D-pad press works instead of failing with NoFocus.
    if navigator.input_focus().is_none()
        && let Some(first) = q_nav.iter().next()
    {
        navigator
            .manual_directional_navigation
            .focus
            .set(first, FocusCause::Navigated);
    }

    let _ = navigator.navigate(dir);
    if let Some(focused) = navigator.input_focus() {
        commands.trigger(ScrollIntoView { entity: focused });
    }
}

pub fn activate_system(
    gamepads: Query<&Gamepad>,
    keys: Res<ButtonInput<KeyCode>>,
    focus: Res<InputFocus>,
    q_btn: Query<&MenuButtonData, With<Button>>,
    q_checkbox: Query<(), With<Checkbox>>,
    mut action_ev: MessageWriter<MenuAction>,
    mut commands: Commands,
) {
    let gamepad_pressed = gamepads
        .iter()
        .any(|g| g.just_pressed(GamepadButton::South));
    let key_pressed = keys.just_pressed(KeyCode::Enter);
    if !gamepad_pressed && !key_pressed {
        return;
    }
    let Some(focused) = focus.get() else {
        return;
    };
    if let Ok(data) = q_btn.get(focused) {
        action_ev.write(data.action.clone());
    } else if q_checkbox.contains(focused) && gamepad_pressed {
        commands.trigger(ToggleChecked { entity: focused });
    }
}

pub fn back_system(
    gamepads: Query<&Gamepad>,
    keys: Res<ButtonInput<KeyCode>>,
    mut action_ev: MessageWriter<MenuAction>,
) {
    let pressed = gamepads.iter().any(|g| g.just_pressed(GamepadButton::East))
        || keys.just_pressed(KeyCode::Escape);
    if pressed {
        action_ev.write(MenuAction::Back);
    }
}
