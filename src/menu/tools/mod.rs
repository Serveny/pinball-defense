use bevy::prelude::Component;

pub mod buttons;
pub mod checkbox;
pub mod row;
pub mod sliders;

#[derive(Component)]
pub struct PropIndex(usize);

#[derive(Component)]
pub struct Active;
