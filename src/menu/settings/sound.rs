use super::settings_menu_layout;
use crate::menu::actions::{MenuAction, SoundAction};
use crate::menu::tools::sliders::spawn_slider;
use crate::prelude::*;
use crate::{menu::tools::row::row, settings::SoundSettings};

pub fn layout(mut cmds: Commands, assets: Res<PinballDefenseAssets>, settings: Res<SoundSettings>) {
    cmds.spawn(settings_menu_layout()).with_children(|p| {
        row("FX Volume", p, &assets, |p| {
            let action = MenuAction::EditSound(SoundAction::FxVolume(settings.fx_volume));
            spawn_slider(action, p);
        });
        row("Music Volume", p, &assets, |p| {
            let action = MenuAction::EditSound(SoundAction::MusicVolume(settings.music_volume));
            spawn_slider(action, p);
        });
    });
}
