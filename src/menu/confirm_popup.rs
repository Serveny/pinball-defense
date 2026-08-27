use super::actions::MenuAction;
use super::tools::menu_btn::{self, ButtonStyle, MenuButton};
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::input_focus::{FocusCause, InputFocus};
use bevy::text::{FontSize, FontSourceTemplate};
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigation;

#[derive(Component, Clone, Default)]
pub struct ConfirmPopup;

pub fn spawn(cmds: &mut Commands, assets: &PinballDefenseAssets) {
    let font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
    let msg = "Unsaved progress will be lost.\nReturn to main menu anyway?".to_string();
    cmds.spawn_scene(bsn! {
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.),
            right: Val::Px(0.),
            top: Val::Px(0.),
            bottom: Val::Px(0.),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        BackgroundColor(Color::srgba_u8(0, 0, 0, 160))
        ConfirmPopup
        Children [
            (Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Px(600.),
                padding: UiRect::all(Val::Px(20.)),
                row_gap: Val::Px(20.),
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.)),
             }
             BorderColor::from(GameColor::GOLD)
             BackgroundColor(Color::srgb_u8(23, 24, 26))
             Children [
                 (Node {
                     width: Val::Percent(100.),
                     justify_content: JustifyContent::Center,
                     align_items: AlignItems::Center,
                  }
                  Children [
                     (Text({msg})
                      TextFont { font: {font}, font_size: FontSize::Px(24.0) }
                      TextColor({GameColor::WHITE}))
                  ]),
                 (menu_btn::scene(MenuAction::ConfirmBackToMainMenu, ButtonStyle::Primary, assets, UiRect::default())),
                 (menu_btn::scene(MenuAction::CancelBackToMainMenu, ButtonStyle::Secondary, assets, UiRect::default())),
             ])
        ]
    });
}

pub fn despawn(cmds: &mut Commands, q: &Query<Entity, With<ConfirmPopup>>) {
    for id in q.iter() {
        cmds.entity(id).despawn();
    }
}

pub fn restrict_navigation(
    q_popup: Query<Entity, With<ConfirmPopup>>,
    children: Query<&Children>,
    q_btn: Query<(Entity, Has<AutoDirectionalNavigation>), With<MenuButton>>,
    mut focus: ResMut<InputFocus>,
    mut cmds: Commands,
) {
    let Some(popup) = q_popup.iter().next() else {
        for (btn, has_nav) in &q_btn {
            if !has_nav {
                cmds.entity(btn).insert(AutoDirectionalNavigation::default());
            }
        }
        return;
    };

    let mut popup_btns = Vec::new();
    for d in children.iter_descendants(popup) {
        if q_btn.get(d).is_ok() {
            popup_btns.push(d);
        }
    }

    for (btn, has_nav) in &q_btn {
        let in_popup = popup_btns.contains(&btn);
        if in_popup && !has_nav {
            cmds.entity(btn).insert(AutoDirectionalNavigation::default());
        } else if !in_popup && has_nav {
            cmds.entity(btn).remove::<AutoDirectionalNavigation>();
        }
    }

    if !focus.get().is_some_and(|f| popup_btns.contains(&f))
        && let Some(first) = popup_btns.first()
    {
        focus.set(*first, FocusCause::Navigated);
    }
}
