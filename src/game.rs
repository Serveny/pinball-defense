use ball::BallPlugin;
//#[cfg(debug_assertions)]
//use bevy_debug_grid::*;
use self::analog_counter::AnalogCounterPlugin;
use self::camera::PinballCameraPlugin;
use self::level::LevelPlugin;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::AppState;
use controls::ControlsPlugin;
use enemy::EnemyPlugin;
use events::PinballEventsPlugin;
use pinball_menu::PinballMenuPlugin;
use player_life::PlayerLifePlugin;
use progress_bar::ProgressBarPlugin;
use std::f32::consts::PI;
use tower::TowerPlugin;
use wave::WavePlugin;
use world::WorldPlugin;

mod analog_counter;
mod ball;
mod ball_starter;
mod camera;
mod colliders;
mod controls;
mod enemy;
mod events;
mod flipper;
mod level;
mod pinball_menu;
mod player_life;
mod progress_bar;
mod road;
mod tower;
mod wave;
mod world;

#[derive(States, PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    None,
    Ingame,
    Pause,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .init_resource::<IngameTime>()
            .add_plugins((
                WorldPlugin,
                BallPlugin,
                PinballCameraPlugin,
                TowerPlugin,
                ControlsPlugin,
                PinballMenuPlugin,
                PinballEventsPlugin,
                ProgressBarPlugin,
                EnemyPlugin,
                WavePlugin,
                PlayerLifePlugin,
                LevelPlugin,
                AnalogCounterPlugin,
            ))
            .add_systems(
                OnEnter(AppState::Game),
                (set_state_ingame, setup_ambient_lights),
            )
            .add_systems(
                Update,
                tick_ingame_timer_system.run_if(in_state(GameState::Ingame)),
            );
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct IngameTime(f32);

fn tick_ingame_timer_system(mut ig_time: ResMut<IngameTime>, time: Res<Time>) {
    **ig_time += time.delta_seconds();
}

#[derive(Component)]
struct Camera;

fn setup_ambient_lights(mut cmds: Commands, g_sett: Res<GraphicsSettings>) {
    cmds.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
    // directional 'sun' light
    cmds.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 16000.0,
            shadows_enabled: g_sett.is_shadows,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 2.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
        ..default()
    });
}

fn set_state_ingame(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Ingame);
}
