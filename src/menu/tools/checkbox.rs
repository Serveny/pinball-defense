use crate::menu::actions::MenuAction;
use crate::menu::{GOLD, WHITE};
use crate::prelude::*;

#[derive(Component)]
pub struct Checkbox;

#[derive(Component)]
pub struct CheckboxMark;

pub fn spawn_checkbox(action: MenuAction, p: &mut ChildBuilder) {
    p.spawn((
        Name::new("Checkbox"),
        Checkbox,
        action,
        ButtonBundle {
            style: Style {
                width: Val::Px(40.),
                height: Val::Px(40.),
                border: UiRect::all(Val::Px(5.0)),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            border_color: GOLD.into(),
            background_color: Color::NONE.into(),
            ..default()
        },
    ))
    .with_children(|p| {
        p.spawn((
            NodeBundle {
                background_color: GOLD.into(),
                style: Style {
                    width: Val::Px(20.),
                    height: Val::Px(20.),
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                visibility: match action.val_bool() {
                    true => Visibility::Inherited,
                    false => Visibility::Hidden,
                },
                ..default()
            },
            CheckboxMark,
        ));
    });
}

#[allow(clippy::type_complexity)]
pub fn checkbox_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BorderColor, &mut MenuAction),
        (Changed<Interaction>, With<Button>, With<Checkbox>),
    >,
    mut q_mark: Query<(&mut Visibility, &mut BackgroundColor, &Parent), With<CheckboxMark>>,
    mut action_ev: EventWriter<MenuAction>,
) {
    for (checkbox_id, interaction, mut border_color, mut action) in &mut interaction_query {
        if let Some((mut visi, mut bg_color, _)) = q_mark
            .iter_mut()
            .find(|(_, _, parent)| parent.get() == checkbox_id)
        {
            match *interaction {
                Interaction::Pressed => {
                    action.toggle_val_bool();
                    *visi = match action.val_bool() {
                        true => Visibility::Inherited,
                        false => Visibility::Hidden,
                    };
                    action_ev.send(*action);
                }
                Interaction::Hovered => {
                    *border_color = WHITE.into();
                    *bg_color = WHITE.into();
                }
                Interaction::None => {
                    *border_color = GOLD.into();
                    *bg_color = GOLD.into();
                }
            }
        }
    }
}
