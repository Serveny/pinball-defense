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
        DefaultPlugins.set(AssetPlugin {
            unapproved_path_mode: bevy::asset::UnapprovedPathMode::Allow,
            ..default()
        }),
        bevy_framepace::FramepacePlugin,
        FrameTimeDiagnosticsPlugin::default(),
        TweeningPlugin,
        WindowTitleLoggerDiagnosticsPlugin::default(),
        EguiPlugin {
            enable_multipass_for_primary_context: false,
        },
    ))
    .init_state::<AppState>()
    .add_systems(Startup, set_framerate);

    add_rapier(&mut app);

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

#[cfg(debug_assertions)]
fn add_debug_plugins(app: &mut App) {
    app.add_plugins((
        WorldInspectorPlugin::default(),
        RapierDebugRenderPlugin::default(),
    ));
}

fn add_rapier(app: &mut App) {
    app.insert_resource(TimestepMode::Variable {
        max_dt: 1. / 80.,
        time_scale: 1.0,
        substeps: 3,
    })
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_systems(OnEnter(AppState::Game), set_rapier_config);
}

fn set_rapier_config(mut q_config: Query<&mut RapierConfiguration>) {
    if let Ok(mut cfg) = q_config.single_mut() {
        *cfg = RapierConfiguration {
            gravity: Vec2::X * 2.,
            force_update_from_transform_changes: false,
            physics_pipeline_active: true,
            query_pipeline_active: true,
            scaled_shape_subdivision: 1,
        }
    }
}
