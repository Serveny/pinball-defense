#![allow(clippy::type_complexity)]

use avian2d::PhysicsPlugins;
#[cfg(debug_assertions)]
use bevy::camera::Hdr;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
#[cfg(debug_assertions)]
use bevy::input::common_conditions::input_toggle_active;
pub use bevy_asset_loader::prelude::*;
use bevy_framepace::Limiter;
#[cfg(debug_assertions)]
use bevy_inspector_egui::bevy_egui::{EguiPlugin, PrimaryEguiContext};
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use bevy_window_title_diagnostics::WindowTitleLoggerDiagnosticsPlugin;
use game::GamePlugin;
use loading::LoadingScreenPlugin;
use menu::MenuPlugin;
use prelude::*;
use settings::{GraphicsSettings, SoundSettings};
use std::path::PathBuf;

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
    MainMenu,
    Game,
}

#[derive(Resource, Default)]
pub struct CliArgs {
    pub load: Option<PathBuf>,
    pub save: Option<PathBuf>,
}

const MAX_FRAME_RATE: f32 = 144.;

fn parse_cli_args() -> CliArgs {
    let mut args = CliArgs::default();
    let mut iter = std::env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--load" => args.load = iter.next().map(PathBuf::from),
            "--save" => args.save = iter.next().map(PathBuf::from),
            _ => {}
        }
    }
    args
}

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

    #[cfg(debug_assertions)]
    app.insert_resource(SoundSettings {
        music_volume: 0.,
        fx_volume: 0.6,
    });

    #[cfg(not(debug_assertions))]
    app.insert_resource(SoundSettings {
        music_volume: 0.4,
        fx_volume: 0.8,
    });
    app.insert_resource(parse_cli_args());
    app.add_plugins((LoadingScreenPlugin, GamePlugin, MenuPlugin))
        .run();
}

fn set_framerate(mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    settings.limiter = Limiter::from_framerate(MAX_FRAME_RATE as f64);
}

fn add_pysics_settings(app: &mut App) {
    app.insert_resource(Gravity(Vec2::X * 2.))
        .insert_resource(Time::<Fixed>::from_hz(128.));
}

#[cfg(debug_assertions)]
fn add_debug_plugins(app: &mut App) {
    app.add_plugins((
        EguiPlugin::default(),
        WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F12)),
        PhysicsDebugPlugin,
    ))
    .add_systems(OnEnter(AppState::Game), spawn_egui_overlay_camera);
}

#[cfg(debug_assertions)]
fn spawn_egui_overlay_camera(mut cmds: Commands) {
    cmds.spawn((
        Name::new("Egui Overlay Camera"),
        Camera2d,
        Camera {
            order: 100,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Hdr,
        PrimaryEguiContext,
    ));
}
