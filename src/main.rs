#![allow(clippy::type_complexity)]

use avian2d::PhysicsPlugins;
#[cfg(debug_assertions)]
use bevy::camera::Hdr;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::window::{MonitorSelection, PrimaryWindow, Window, WindowMode};
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
pub mod menu;
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
        bevy_hanabi::prelude::HanabiPlugin,
        FrameTimeDiagnosticsPlugin::default(),
        TweeningPlugin,
        WindowTitleLoggerDiagnosticsPlugin::default(),
        PhysicsPlugins::default(),
    ))
    .init_state::<AppState>()
    .add_systems(Startup, set_framerate)
    .add_systems(Update, toggle_fullscreen);

    add_pysics_settings(&mut app);

    // Only show debug data in debug mode
    #[cfg(debug_assertions)]
    add_debug_plugins(&mut app);

    app.insert_resource(GraphicsSettings::low());

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
    settings.limiter = Limiter::from_framerate(f64::from(MAX_FRAME_RATE));
}

fn toggle_fullscreen(
    keys: Res<ButtonInput<KeyCode>>,
    mut g_sett: ResMut<GraphicsSettings>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::F11)
        && let Ok(mut window) = q_window.single_mut()
    {
        g_sett.is_fullscreen = !matches!(
            window.mode,
            WindowMode::Fullscreen(..) | WindowMode::BorderlessFullscreen(..)
        );
        window.mode = if g_sett.is_fullscreen {
            WindowMode::BorderlessFullscreen(MonitorSelection::Current)
        } else {
            WindowMode::Windowed
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(debug_assertions)]
    #[test]
    fn f12_toggles_debug_rendering_once_per_press() {
        let mut app = App::new();
        let mut config_store = GizmoConfigStore::default();
        config_store.insert(
            GizmoConfig {
                enabled: false,
                ..default()
            },
            PhysicsGizmos::default(),
        );
        app.insert_resource(config_store)
            .init_resource::<ButtonInput<KeyCode>>()
            .add_systems(Update, toggle_debug_visibility);

        for (press_f12, visible) in [
            (false, false),
            (true, true),
            (false, true),
            (true, false),
            (true, true),
        ] {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.clear();
            if press_f12 {
                keys.release(KeyCode::F12);
                keys.press(KeyCode::F12);
            }
            app.update();

            assert_eq!(
                app.world()
                    .resource::<GizmoConfigStore>()
                    .config::<PhysicsGizmos>()
                    .0
                    .enabled,
                visible
            );
        }
    }

    #[test]
    fn f11_keeps_window_and_graphics_settings_in_sync() {
        let mut app = App::new();
        app.insert_resource(GraphicsSettings::low())
            .add_systems(Update, toggle_fullscreen);
        let window = app
            .world_mut()
            .spawn((Window::default(), PrimaryWindow))
            .id();

        for (press_f11, fullscreen) in [(true, true), (false, true), (true, false)] {
            let mut keys = ButtonInput::<KeyCode>::default();
            if press_f11 {
                keys.press(KeyCode::F11);
            }
            app.insert_resource(keys);
            app.update();

            assert_eq!(
                app.world().resource::<GraphicsSettings>().is_fullscreen,
                fullscreen
            );
            let mode = app.world().get::<Window>(window).unwrap().mode;
            assert_eq!(
                mode,
                if fullscreen {
                    WindowMode::BorderlessFullscreen(MonitorSelection::Current)
                } else {
                    WindowMode::Windowed
                }
            );
        }
    }
}

fn add_pysics_settings(app: &mut App) {
    app.insert_resource(Gravity(Vec2::X * 2.))
        .insert_resource(Time::<Fixed>::from_hz(128.));
}

#[cfg(debug_assertions)]
fn add_debug_plugins(app: &mut App) {
    app.add_plugins((
        EguiPlugin::default(),
        WorldInspectorPlugin::new().run_if(|config_store: Res<GizmoConfigStore>| {
            config_store.config::<PhysicsGizmos>().0.enabled
        }),
        PhysicsDebugPlugin,
    ))
    .insert_gizmo_config(
        PhysicsGizmos::default(),
        GizmoConfig {
            enabled: false,
            ..default()
        },
    )
    .add_systems(Update, toggle_debug_visibility)
    .add_systems(OnEnter(AppState::Game), spawn_egui_overlay_camera);
}

#[cfg(debug_assertions)]
fn toggle_debug_visibility(
    keys: Res<ButtonInput<KeyCode>>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    if keys.just_pressed(KeyCode::F12) {
        let (config, _) = config_store.config_mut::<PhysicsGizmos>();
        config.enabled = !config.enabled;
    }
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
