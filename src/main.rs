use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
//#[cfg(debug_assertions)]
//use bevy_debug_grid::*;
pub use bevy_asset_loader::prelude::*;
use bevy_framepace::Limiter;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use bevy_window_title_diagnostics::WindowTitleLoggerDiagnosticsPlugin;
use game::GamePlugin;
use loading::LoadingScreenPlugin;
use prelude::*;
use settings::GraphicsSettings;

mod assets;
mod game;
mod loading;
mod prelude;
mod settings;
mod utils;

#[derive(States, PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    Game,
}

pub const TICK_TIME: f32 = 1. / 120.;

fn main() {
    let mut app = App::new();

    app.add_state::<AppState>()
        .add_plugins((
            DefaultPlugins,
            bevy_framepace::FramepacePlugin,
            FrameTimeDiagnosticsPlugin,
            TweeningPlugin,
            WindowTitleLoggerDiagnosticsPlugin::default(),
        ))
        .insert_resource(FixedTime::new_from_secs(TICK_TIME))
        .add_systems(Startup, set_framerate);

    add_rapier(&mut app);

    // Only show debug data in debug mode
    #[cfg(debug_assertions)]
    add_debug_plugins(&mut app);

    #[cfg(debug_assertions)]
    app.insert_resource(GraphicsSettings::low());

    #[cfg(not(debug_assertions))]
    app.insert_resource(GraphicsSettings::high());

    app.add_plugins((LoadingScreenPlugin, GamePlugin)).run();
}

fn set_framerate(mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    settings.limiter = Limiter::from_framerate(1. / TICK_TIME as f64);
}

#[cfg(debug_assertions)]
fn add_debug_plugins(app: &mut App) {
    app.add_plugins((RapierDebugRenderPlugin::default(), WorldInspectorPlugin::default()))
        //.add_plugin(DebugGridPlugin::with_floor_grid())
        ;
}

fn add_rapier(app: &mut App) {
    let rapier_cfg = RapierConfiguration {
        //timestep_mode: TimestepMode::Variable {
        //max_dt: TICK_TIME,
        //time_scale: 1.,
        //substeps: 2,
        //},
        //timestep_mode: TimestepMode::Fixed {
        //dt: TICK_TIME,
        //substeps: 2,
        //},
        //timestep_mode: TimestepMode::Interpolated {
        //dt: TICK_TIME,
        //time_scale: 1.,
        //substeps: 1,
        //},
        timestep_mode: TimestepMode::Variable {
            max_dt: TICK_TIME,
            time_scale: 1.0,
            substeps: 2,
        },
        gravity: Vec2::X,
        ..default()
    };
    app.insert_resource(rapier_cfg)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
}
