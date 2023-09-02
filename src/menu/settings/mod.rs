use crate::prelude::*;

pub mod sound;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum SettingsMenuState {
    #[default]
    None,
    Controls,
    Sound,
    Graphics,
}
