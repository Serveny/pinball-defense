use crate::prelude::*;
use crate::utils::GameColor;
use bevy::input_focus::InputFocus;
use bevy::picking::hover::Hovered;

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
        (Entity, &mut BorderColor, Option<&Interaction>, Option<&Hovered>),
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
