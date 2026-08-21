use crate::prelude::*;
use bevy::text::{FontSize, FontSourceTemplate};

pub fn headline(text: &str, font_size: f32, assets: &PinballDefenseAssets) -> impl Scene {
    let text = text.to_string();
    let font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
    bsn! {
        Node {
            width: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        Children [
            (Text({text})
             TextFont { font: {font}, font_size: FontSize::Px({font_size}) }
             TextColor(Color::srgb_u8(255, 254, 236)))
        ]
    }
}
