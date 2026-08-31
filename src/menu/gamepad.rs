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

    let Some(dir) = direction else {
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
    focus: Res<InputFocus>,
    q_btn: Query<&MenuButtonData, With<Button>>,
    q_checkbox: Query<(), With<Checkbox>>,
    mut action_ev: MessageWriter<MenuAction>,
    mut commands: Commands,
) {
    let pressed = gamepads
        .iter()
        .any(|g| g.just_pressed(GamepadButton::South));
    if !pressed {
        return;
    }
    let Some(focused) = focus.get() else {
        return;
    };
    if let Ok(data) = q_btn.get(focused) {
        action_ev.write(data.action.clone());
    } else if q_checkbox.contains(focused) {
        commands.trigger(ToggleChecked { entity: focused });
    }
}

pub fn back_system(gamepads: Query<&Gamepad>, mut action_ev: MessageWriter<MenuAction>) {
    let pressed = gamepads.iter().any(|g| g.just_pressed(GamepadButton::East));
    if pressed {
        action_ev.write(MenuAction::Back);
    }
}
