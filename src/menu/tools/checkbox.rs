use crate::menu::settings::SettingsMenuState;
use crate::prelude::*;
use crate::settings::{GraphicsSettings, SoundSettings};
use crate::utils::GameColor;
use crate::utils::reflect::set_field;
use bevy::ecs::observer::On;
use bevy::input_focus::tab_navigation::TabIndex;
use bevy::picking::hover::Hovered;
use bevy::ui::Checked;
use bevy::ui_widgets::{Checkbox, ValueChange, checkbox_self_update};

#[derive(Component, Clone, Default)]
pub struct CheckboxMark;

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
            };
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
