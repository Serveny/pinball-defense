use super::health::{Health, HealthEmptyEvent, HealthRecovery};
use super::GameState;
use crate::prelude::*;

pub struct PlayerLifePlugin;

impl Plugin for PlayerLifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (game_over_system).run_if(in_state(GameState::Ingame)),
        );
    }
}

#[derive(Component)]
pub struct LifeBar;

pub fn spawn_life_bar(
    parent: &mut ChildBuilder,
    assets: &PinballDefenseGltfAssets,
    materials: &mut Assets<StandardMaterial>,
    trans: Transform,
) {
    parent
        .spawn((
            Name::new("Life Bar"),
            SpatialBundle::from_transform(trans),
            LifeBar,
            Health::new(100.),
            HealthRecovery::new(4., 6.),
        ))
        .with_children(|parent| {
            super::progress_bar::spawn(
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

fn game_over_system(mut evr: EventReader<HealthEmptyEvent>, q_life_bar: Query<With<LifeBar>>) {
    for ev in evr.iter() {
        let rel_id = ev.0;
        if q_life_bar.contains(rel_id) {
            println!("Game Over");
        }
    }
}
