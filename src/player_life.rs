use crate::prelude::*;
use crate::GameState;

pub struct PlayerLifePlugin;

impl Plugin for PlayerLifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (reload_life,).run_if(in_state(GameState::Ingame)));
    }
}

#[derive(Component)]
pub struct LifeBar;

pub fn spawn_life_bar(
    parent: &mut ChildBuilder,
    assets: &PinballDefenseAssets,
    materials: &mut Assets<StandardMaterial>,
    trans: Transform,
) {
    parent
        .spawn((
            SpatialBundle::from_transform(trans),
            LifeBar,
            Name::new("Life Bar"),
        ))
        .with_children(|parent| {
            crate::progress_bar::spawn(
                parent,
                assets,
                materials,
                parent.parent_entity(),
                Transform::default(),
                Color::GREEN,
                1.,
            )
        });
}

fn reload_life() {}
