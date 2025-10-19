use super::health::{Health, HealthEmptyEvent, HealthRecovery};
use super::{EventState, GameState};
use crate::prelude::*;

pub struct PlayerLifePlugin;

impl Plugin for PlayerLifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (on_game_over_system).run_if(in_state(EventState::Active)),
        );
    }
}

#[derive(Component)]
pub struct LifeBar;

pub fn spawn_life_bar(
    spawner: &mut ChildSpawnerCommands,
    assets: &PinballDefenseGltfAssets,
    materials: &mut Assets<StandardMaterial>,
    trans: Transform,
) {
    spawner
        .spawn((
            Name::new("Life Bar"),
            trans,
            LifeBar,
            Health::new(100.),
            HealthRecovery::new(4., 6.),
            Visibility::Inherited,
        ))
        .with_children(|spawner| {
            let color = Color::srgb_u8(156, 217, 26);
            super::progress::spawn(
                spawner,
                assets,
                materials,
                spawner.target_entity(),
                Transform::default(),
                color,
                1.,
            )
        });
}

fn on_game_over_system(
    mut evr: MessageReader<HealthEmptyEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    mut ev_state: ResMut<NextState<EventState>>,
    q_life_bar: Query<Entity, With<LifeBar>>,
) {
    for ev in evr.read() {
        let rel_id = ev.0;
        if q_life_bar.contains(rel_id) {
            println!("Game Over");
            game_state.set(GameState::GameOver);
            ev_state.set(EventState::Inactive);
        }
    }
}
