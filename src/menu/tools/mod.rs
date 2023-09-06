use bevy::prelude::Component;

pub mod checkbox;
pub mod keybox;
pub mod menu_btn;
pub mod row;
pub mod sliders;

#[derive(Component)]
pub struct PropIndex(usize);

#[derive(Component)]
pub struct Active;
