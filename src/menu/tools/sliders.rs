use crate::menu::settings::SettingsMenuState;
use crate::prelude::*;
use crate::settings::{GraphicsSettings, SoundSettings};
use crate::utils::GameColor;
use crate::utils::reflect::set_field;
use bevy::ecs::observer::On;
use bevy::input_focus::tab_navigation::TabIndex;
use bevy::picking::hover::Hovered;
use bevy::ui::Pressed;
use bevy::ui_widgets::{
    Slider, SliderOrientation, SliderRange, SliderThumb, SliderValue, TrackClick, ValueChange,
    slider_self_update,
};

/// Diameter of the slider thumb in pixels.
const THUMB_SIZE: f32 = 30.;

// init_val must be between 0 and 1
pub fn spawn(p: &mut ChildSpawnerCommands, prop_i: usize, init_val: f32) {
    p.spawn((
        Name::new("Slider"),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            min_width: Val::Px(120.),
            ..default()
        },
        Slider {
            track_click: TrackClick::Drag,
            orientation: SliderOrientation::Horizontal,
        },
        SliderValue(init_val),
        SliderRange::new(0., 1.),
        Hovered::default(),
        TabIndex(0),
    ))
    // Let the slider widget keep its own `SliderValue` in sync with `ValueChange` events.
    .observe(slider_self_update)
    .observe(
        move |change: On<ValueChange<f32>>,
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
        },
    )
    .with_children(|p| {
        // Static track (the rail the thumb slides along).
        p.spawn((
            Name::new("Slider Track"),
            Node {
                height: Val::Px(10.),
                width: Val::Percent(100.),
                border_radius: BorderRadius::all(Val::Px(5.)),
                ..default()
            },
            BackgroundColor(GameColor::GRAY),
        ));
        // Inner track that is shorter than the slider by exactly the thumb size, so the
        // thumb can be positioned with simple percentages without overhanging the ends.
        p.spawn((
            Name::new("Slider Thumb Track"),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                right: Val::Px(THUMB_SIZE),
                top: Val::Px(0.),
                bottom: Val::Px(0.),
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn((
                Name::new("Slider Thumb"),
                SliderThumb,
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    bottom: Val::Px(0.),
                    left: Val::Percent(init_val * 100.),
                    width: Val::Px(THUMB_SIZE),
                    height: Val::Px(THUMB_SIZE),
                    margin: UiRect::vertical(Val::Auto),
                    border: UiRect::all(Val::Px(4.)),
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                BorderColor::from(GameColor::GOLD),
                BackgroundColor(GameColor::WHITE),
            ));
        });
    });
}

/// Moves the thumb to match the slider's current value. The core [`Slider`] widget handles the
/// drag math but does not move the thumb visually; that is the stylist's responsibility.
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

/// Updates the thumb's border color based on hover/press state.
pub fn update_thumb_style(
    q_sliders: Query<(Entity, &Hovered, Has<Pressed>), With<Slider>>,
    children: Query<&Children>,
    mut thumbs: Query<&mut BorderColor, With<SliderThumb>>,
) {
    for (slider_ent, hovered, pressed) in q_sliders.iter() {
        let color = if pressed || hovered.0 {
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
