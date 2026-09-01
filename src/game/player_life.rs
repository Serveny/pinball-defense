use super::health::{Health, HealthEmptyEvent, HealthRecovery};
use super::world::QueryWorld;
use super::{EventState, GameState};
use crate::prelude::*;
use moonshine_save::prelude::Save;

pub struct PlayerLifePlugin;

impl Plugin for PlayerLifePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LifeBar>()
            .init_resource::<GameOverDelay>()
            .add_systems(
                Update,
                reattach_life_bar_system.run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (on_game_over_system).run_if(in_state(EventState::Active)),
            )
            .add_systems(
                Update,
                delayed_game_over_system.run_if(in_state(GameState::Ingame)),
            );
    }
}

// Life bar animates at 0.5/s in progress::scale_system, so a full bar needs 2s to drain
const GAME_OVER_ANIMATION_SECS: f32 = 2.0;

#[derive(Resource, Default, Deref, DerefMut)]
pub(crate) struct GameOverDelay(Option<f32>);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(Save)]
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
            );
        });
}

fn reattach_life_bar_system(
    mut cmds: Commands,
    assets: Res<PinballDefenseGltfAssets>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    q_world: QueryWorld,
    q_life_bars: Query<(Entity, &Health), (With<LifeBar>, Without<Children>)>,
) {
    let Ok(world) = q_world.single() else { return };
    for (life_bar_id, health) in q_life_bars.iter() {
        cmds.entity(world).add_child(life_bar_id);
        cmds.entity(life_bar_id).with_children(|spawner| {
            let color = Color::srgb_u8(156, 217, 26);
            super::progress::spawn(
                spawner,
                &assets,
                &mut mats,
                life_bar_id,
                Transform::default(),
                color,
                health.fraction(),
            );
        });
    }
}

fn on_game_over_system(
    mut evr: MessageReader<HealthEmptyEvent>,
    mut game_over_delay: ResMut<GameOverDelay>,
    q_life_bar: Query<Entity, With<LifeBar>>,
) {
    for ev in evr.read() {
        if q_life_bar.contains(ev.0) {
            println!("Game Over");
            **game_over_delay = Some(GAME_OVER_ANIMATION_SECS);
        }
    }
}

fn delayed_game_over_system(
    time: Res<Time>,
    mut game_over_delay: ResMut<GameOverDelay>,
    mut game_state: ResMut<NextState<GameState>>,
    mut ev_state: ResMut<NextState<EventState>>,
) {
    let Some(remaining) = **game_over_delay else {
        return;
    };
    let remaining = remaining - time.delta_secs();
    **game_over_delay = Some(remaining);
    if remaining <= 0. {
        **game_over_delay = None;
        game_state.set(GameState::GameOver);
        ev_state.set(EventState::Inactive);
    }
}
