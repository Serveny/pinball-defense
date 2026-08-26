use crate::menu::MenuLayout;
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::ui_widgets::{ControlOrientation, Scrollbar, ScrollbarThumb};

pub const WIDTH: f32 = 10.;

pub fn spawn(cmds: &mut Commands, target: Entity) {
    cmds.spawn_scene(bsn! {
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(0.),
            top: Val::Px(0.),
            bottom: Val::Px(0.),
            width: Val::Px({WIDTH}),
        }
        MenuLayout
        GlobalZIndex(1)
        Scrollbar {
            target: {target},
            orientation: ControlOrientation::Vertical,
            min_thumb_length: {WIDTH},
        }
        Children [
            (ScrollbarThumb {
                border_radius: BorderRadius::all(Val::Px(WIDTH / 2.)),
                border: UiRect::all(Val::Px(1.)),
            }
             BackgroundColor(GameColor::GOLD)
             BorderColor::all(GameColor::GRAY))
        ]
    });
}

pub fn update_visibility(
    mut q_scrollbar: Query<(&Scrollbar, &mut Visibility)>,
    q_scroll_area: Query<&ComputedNode>,
) {
    for (scrollbar, mut vis) in &mut q_scrollbar {
        let scrollable = q_scroll_area
            .get(scrollbar.target)
            .map(|c| c.content_size().y > c.size().y)
            .unwrap_or(false);
        *vis = if scrollable {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}
