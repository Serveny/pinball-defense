use avian2d::PhysicsPlugins;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
pub use bevy_asset_loader::prelude::*;
use bevy_framepace::Limiter;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use bevy_window_title_diagnostics::WindowTitleLoggerDiagnosticsPlugin;
use game::GamePlugin;
use loading::LoadingScreenPlugin;
use menu::MenuPlugin;
use prelude::*;
use settings::{GraphicsSettings, SoundSettings};

mod assets;
mod game;
mod generated;
mod loading;
mod menu;
mod prelude;
mod settings;
mod utils;

#[derive(States, PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    Game,
}

const MAX_FRAME_RATE: f32 = 144.;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        bevy_framepace::FramepacePlugin,
        FrameTimeDiagnosticsPlugin,
        TweeningPlugin,
        WindowTitleLoggerDiagnosticsPlugin::default(),
        EguiPlugin,
        PhysicsPlugins::default(),
    ))
    .init_state::<AppState>()
    .add_systems(Startup, set_framerate);

    add_pysics_settings(&mut app);

    // Only show debug data in debug mode
    #[cfg(debug_assertions)]
    add_debug_plugins(&mut app);

    #[cfg(debug_assertions)]
    app.insert_resource(GraphicsSettings::low());

    #[cfg(not(debug_assertions))]
    app.insert_resource(GraphicsSettings::high());

    app.insert_resource(SoundSettings::default());
    app.add_plugins((LoadingScreenPlugin, GamePlugin, MenuPlugin))
        .run();
}

fn set_framerate(mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    settings.limiter = Limiter::from_framerate(MAX_FRAME_RATE as f64);
}

fn add_pysics_settings(app: &mut App) {
    app.insert_resource(Gravity(Vec2::X * 9.81))
        .insert_resource(Time::<Fixed>::from_hz(128.));
}

#[cfg(debug_assertions)]
fn add_debug_plugins(app: &mut App) {
    app.add_plugins((
        WorldInspectorPlugin::default(),
        PhysicsDebugPlugin::default(),
    ));
}
