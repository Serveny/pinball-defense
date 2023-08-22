use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
//#[cfg(debug_assertions)]
//use bevy_debug_grid::*;
pub use bevy_asset_loader::prelude::*;
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

fn main() {
    let mut app = App::new();

    app.add_state::<AppState>()
        .add_loading_state(LoadingState::new(AppState::Loading).continue_to_state(AppState::Game))
        .add_collection_to_loading_state::<_, PinballDefenseAssets>(AppState::Loading)
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin,
            TweeningPlugin,
            WindowTitleLoggerDiagnosticsPlugin::default(),
        ));
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

#[cfg(debug_assertions)]
fn add_debug_plugins(app: &mut App) {
    app.add_plugins((RapierDebugRenderPlugin::default(), WorldInspectorPlugin::default()))
        //.add_plugin(DebugGridPlugin::with_floor_grid())
        ;
}

fn add_rapier(app: &mut App) {
    let rapier_cfg = RapierConfiguration {
        //timestep_mode: TimestepMode::Variable {
        //max_dt: 1. / 128.,
        //time_scale: 1.,
        //substeps: 1,
        //},
        //timestep_mode: TimestepMode::Fixed {
        //dt: 1. / 64.,
        //substeps: 4,
        //},
        ..default()
    };
    app.insert_resource(rapier_cfg)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
}
