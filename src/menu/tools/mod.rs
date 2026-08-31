use crate::prelude::*;
use crate::utils::GameColor;
use bevy::input_focus::{FocusCause, InputFocus};
use bevy::picking::hover::Hovered;
use bevy::ui::Interaction;

pub mod checkbox;
pub mod keybox;
pub mod menu_btn;
pub mod row;
pub mod scrollbar;
pub mod sliders;

#[derive(Component, Clone, Default)]
pub struct PropIndex(usize);

#[derive(Component, Clone, Default)]
pub struct Active;

#[derive(Component, Clone, Default)]
pub struct Focusable;

pub fn focus_hover_system(
    focus: Res<InputFocus>,
    mut q: Query<
        (
            Entity,
            &mut BorderColor,
            Option<&Interaction>,
            Option<&Hovered>,
        ),
        With<Focusable>,
    >,
) {
    for (entity, mut border, interaction, hovered) in &mut q {
        let active = focus.get() == Some(entity)
            || interaction.is_some_and(|i| *i == Interaction::Hovered)
            || hovered.is_some_and(|h| h.0);
        border.set_all(if active {
            GameColor::WHITE
        } else {
            GameColor::GOLD
        });
    }
}

pub fn hover_focus_system(
    q_hovered: Query<(Entity, Option<&Hovered>, Option<&Interaction>)>,
    mut last_hovered: Local<Option<Entity>>,
    mut focus: ResMut<InputFocus>,
) {
    let hovered = q_hovered
        .iter()
        .find(|(_, h, i)| h.is_some_and(|h| h.0) || i.is_some_and(|i| *i == Interaction::Hovered))
        .map(|(e, _, _)| e);
    if hovered != *last_hovered {
        *last_hovered = hovered;
        if let Some(target) = hovered {
            focus.set(target, FocusCause::Navigated);
        }
    }
}
