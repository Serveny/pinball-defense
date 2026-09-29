use super::Focusable;
use crate::menu::settings::SettingsMenuState;
use crate::prelude::*;
use crate::settings::{GraphicsSettings, SoundSettings};
use crate::utils::GameColor;
use crate::utils::reflect::set_field;
use bevy::ecs::observer::On;
use bevy::input_focus::tab_navigation::TabIndex;
use bevy::picking::hover::Hovered;
use bevy::reflect::structs::Struct;
use bevy::ui::Checked;
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigation;
use bevy::ui_widgets::{Checkbox, ValueChange, checkbox_self_update};

#[derive(Component, Clone, Default)]
pub struct CheckboxMark;

#[derive(Component, Clone, Copy, Deref)]
pub struct CheckboxField(pub usize);

pub fn scene(prop_i: usize) -> impl Scene {
    bsn! {
        Name::new("Checkbox")
        Checkbox
        Node {
            width: Val::Px(40.),
            height: Val::Px(40.),
            border: UiRect::all(Val::Px(5.)),
            margin: UiRect::all(Val::Auto),
            border_radius: BorderRadius::all(Val::Px(4.)),
        }
        BorderColor::from(GameColor::GOLD)
        BackgroundColor(Color::NONE)
        Hovered::default()
        TabIndex(0)
        AutoDirectionalNavigation
        Focusable
        on(checkbox_self_update)
        on(move |change: On<ValueChange<bool>>,
              menu_state: Res<State<SettingsMenuState>>,
              mut g_sett: ResMut<GraphicsSettings>,
              mut s_sett: ResMut<SoundSettings>| {
            let val = change.value;
            match **menu_state {
                SettingsMenuState::Sound => {
                    set_field(&mut *s_sett, prop_i, Box::new(val));
                }
                SettingsMenuState::Graphics => {
                    set_field(&mut *g_sett, prop_i, Box::new(val));
                }
                _ => (),
            }
        })
        Children [
            (Name::new("Checkbox Mark")
             Node {
                 width: Val::Px(20.),
                 height: Val::Px(20.),
                 margin: UiRect::all(Val::Auto),
             }
             BackgroundColor(GameColor::GOLD)
             CheckboxMark)
        ]
    }
}

pub fn spawn(p: &mut ChildSpawnerCommands, prop_i: usize, init_val: bool) {
    let mut entity = p.spawn_empty();
    entity.apply_scene(scene(prop_i));
    entity.insert(CheckboxField(prop_i));
    if init_val {
        entity.insert(Checked);
    }
}

pub fn update_mark_visibility(
    q_checkboxes: Query<(Entity, Has<Checked>), With<Checkbox>>,
    children: Query<&Children>,
    mut marks: Query<&mut Visibility, With<CheckboxMark>>,
) {
    for (checkbox_ent, checked) in q_checkboxes.iter() {
        let target = if checked {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        for child in children.iter_descendants(checkbox_ent) {
            if let Ok(mut visi) = marks.get_mut(child) {
                *visi = target;
            }
        }
    }
}

pub fn sync_from_settings(
    g_sett: Res<GraphicsSettings>,
    s_sett: Res<SoundSettings>,
    menu_state: Res<State<SettingsMenuState>>,
    mut cmds: Commands,
    q_checkboxes: Query<(Entity, &CheckboxField, Has<Checked>), With<Checkbox>>,
) {
    let changed = g_sett.is_changed() || s_sett.is_changed();
    let active = matches!(
        **menu_state,
        SettingsMenuState::Graphics | SettingsMenuState::Sound
    );
    if !changed || !active {
        return;
    }
    for (ent, CheckboxField(prop_i), checked) in q_checkboxes.iter() {
        let source: &dyn Struct = if **menu_state == SettingsMenuState::Sound {
            &*s_sett
        } else {
            &*g_sett
        };
        let Some(field) = source.field_at(*prop_i).and_then(|f| f.try_as_reflect()) else {
            continue;
        };
        let Some(val) = crate::utils::reflect::cast::<bool>(field) else {
            continue;
        };
        if val != checked {
            if val {
                cmds.entity(ent).insert(Checked);
            } else {
                cmds.entity(ent).remove::<Checked>();
            }
        }
    }
}
