use super::settings_menu_layout;
use crate::menu::actions::{GraphicsAction, MenuAction};
use crate::menu::tools::checkbox::spawn_checkbox;
use crate::menu::tools::row::row;
use crate::menu::tools::sliders::spawn_slider;
use crate::prelude::*;
use crate::settings::GraphicsSettings;

pub fn layout(
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
    settings: Res<GraphicsSettings>,
) {
    cmds.spawn(settings_menu_layout()).with_children(|p| {
        row("Shadows", p, &assets, |p| {
            let action = MenuAction::EditGraphics(GraphicsAction::IsShadows(settings.is_shadows));
            spawn_checkbox(action, p);
        });
        row("HDR", p, &assets, |p| {
            let action = MenuAction::EditGraphics(GraphicsAction::IsHdr(settings.is_hdr));
            spawn_checkbox(action, p);
        });
        row("Bloom", p, &assets, |p| {
            let action =
                MenuAction::EditGraphics(GraphicsAction::BloomIntensity(settings.bloom.intensity));
            spawn_slider(action, p);
        });
    });
}
