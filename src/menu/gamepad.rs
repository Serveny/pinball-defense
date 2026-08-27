use super::actions::MenuAction;
use super::settings::SettingsMenuState;
use super::tools::menu_btn::MenuButtonData;
use crate::prelude::*;
use bevy::input_focus::{FocusCause, InputFocus};
use bevy::math::CompassOctant;
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigation;
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigator;
use bevy::ui_widgets::{
    Checkbox, SetSliderValue, Slider, SliderValueChange, ToggleChecked,
};

pub fn navigation_system(
    gamepads: Query<&Gamepad>,
    q_slider: Query<(), With<Slider>>,
    q_nav: Query<Entity, With<AutoDirectionalNavigation>>,
    mut navigator: AutoDirectionalNavigator,
    mut commands: Commands,
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

    let focused = navigator.input_focus();
    let is_slider = focused.is_some_and(|e| q_slider.contains(e));
    if is_slider {
        match dir {
            CompassOctant::West => {
                if let Some(e) = focused {
                    commands.trigger(SetSliderValue {
                        entity: e,
                        change: SliderValueChange::RelativeStep(-1.),
                    });
                }
            }
            CompassOctant::East => {
                if let Some(e) = focused {
                    commands.trigger(SetSliderValue {
                        entity: e,
                        change: SliderValueChange::RelativeStep(1.),
                    });
                }
            }
            _ => {
                let _ = navigator.navigate(dir);
            }
        }
    } else {
        let _ = navigator.navigate(dir);
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

pub fn back_system(
    gamepads: Query<&Gamepad>,
    settings_state: Res<State<SettingsMenuState>>,
    mut settings_next: ResMut<NextState<SettingsMenuState>>,
    mut action_ev: MessageWriter<MenuAction>,
) {
    if !gamepads.iter().any(|g| g.just_pressed(GamepadButton::East)) {
        return;
    }
    if *settings_state.get() != SettingsMenuState::None {
        settings_next.set(SettingsMenuState::None);
    } else {
        action_ev.write(MenuAction::Back);
    }
}