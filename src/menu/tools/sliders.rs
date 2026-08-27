use crate::menu::settings::SettingsMenuState;
use crate::prelude::*;
use crate::settings::{GraphicsSettings, SoundSettings};
use crate::utils::GameColor;
use crate::utils::reflect::set_field;
use bevy::ecs::observer::On;
use bevy::input_focus::InputFocus;
use bevy::input_focus::tab_navigation::TabIndex;
use bevy::picking::hover::Hovered;
use bevy::ui::Pressed;
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigation;
use bevy::ui_widgets::{
    Slider, SliderOrientation, SliderRange, SliderStep, SliderThumb, SliderValue, TrackClick,
    ValueChange, slider_self_update,
};

const THUMB_SIZE: f32 = 30.;

pub fn scene(prop_i: usize, init_val: f32) -> impl Scene {
    bsn! {
        Name::new("Slider")
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            min_width: Val::Px(120.),
        }
        Slider {
            track_click: TrackClick::Drag,
            orientation: SliderOrientation::Horizontal,
        }
        SliderValue(init_val)
        SliderRange::new(0., 1.)
        SliderStep(0.05)
        Hovered::default()
        TabIndex(0)
        AutoDirectionalNavigation
        on(slider_self_update)
        on(move |change: On<ValueChange<f32>>,
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
            (Name::new("Slider Track")
             Node {
                 height: Val::Px(10.),
                 width: Val::Percent(100.),
                 border_radius: BorderRadius::all(Val::Px(5.)),
             }
             BackgroundColor(GameColor::GRAY)),
            (Name::new("Slider Thumb Track")
             Node {
                 position_type: PositionType::Absolute,
                 left: Val::Px(0.),
                 right: Val::Px({THUMB_SIZE}),
                 top: Val::Px(0.),
                 bottom: Val::Px(0.),
             }
             Children [
                 (Name::new("Slider Thumb")
                  SliderThumb
                  Node {
                      position_type: PositionType::Absolute,
                      top: Val::Px(0.),
                      bottom: Val::Px(0.),
                      left: Val::Percent({init_val * 100.}),
                      width: Val::Px({THUMB_SIZE}),
                      height: Val::Px({THUMB_SIZE}),
                      margin: UiRect::vertical(Val::Auto),
                      border: UiRect::all(Val::Px(4.)),
                      border_radius: BorderRadius::MAX,
                  }
                  BorderColor::from(GameColor::GOLD)
                  BackgroundColor(GameColor::WHITE))
             ])
        ]
    }
}

pub fn spawn(p: &mut ChildSpawnerCommands, prop_i: usize, init_val: f32) {
    p.spawn_empty().apply_scene(scene(prop_i, init_val));
}

pub fn update_thumb_position(
    q_sliders: Query<(Entity, &SliderValue, &SliderRange), With<Slider>>,
    children: Query<&Children>,
    mut thumbs: Query<&mut Node, With<SliderThumb>>,
) {
    for (slider_ent, value, range) in q_sliders.iter() {
        let percent = range.thumb_position(value.0) * 100.;
        for child in children.iter_descendants(slider_ent) {
            if let Ok(mut node) = thumbs.get_mut(child) {
                node.left = Val::Percent(percent);
            }
        }
    }
}

pub fn update_thumb_style(
    focus: Res<InputFocus>,
    q_sliders: Query<(Entity, &Hovered, Has<Pressed>), With<Slider>>,
    children: Query<&Children>,
    mut thumbs: Query<&mut BorderColor, With<SliderThumb>>,
) {
    for (slider_ent, hovered, pressed) in q_sliders.iter() {
        let color = if pressed || hovered.0 || focus.get() == Some(slider_ent) {
            GameColor::WHITE
        } else {
            GameColor::GOLD
        };
        for child in children.iter_descendants(slider_ent) {
            if let Ok(mut border) = thumbs.get_mut(child) {
                border.set_all(color);
            }
        }
    }
}
