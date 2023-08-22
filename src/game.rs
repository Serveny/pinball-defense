use ball::BallPlugin;
//#[cfg(debug_assertions)]
//use bevy_debug_grid::*;
use self::analog_counter::AnalogCounterPlugin;
use self::camera::PinballCameraPlugin;
use self::level::LevelPlugin;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::AppState;
use bevy::gltf::{Gltf, GltfMesh};
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
//mod colliders;
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
            .add_state::<TempAppState>()
            .init_resource::<GltfHandle>()
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
            .add_systems(Update, check_assets_ready.run_if(in_state(GameState::None)))
            .add_systems(OnEnter(TempAppState::Loaded), load_gltf_content)
            .add_systems(
                OnEnter(AppState::Game),
                (setup_ambient_lights, init_gltf_load),
            )
            .add_systems(
                Update,
                tick_ingame_timer_system.run_if(in_state(GameState::Ingame)),
            );
    }
}

#[derive(Resource, Default)]
struct GltfHandle(Handle<Gltf>);

fn init_gltf_load(mut cmds: Commands, ass: Res<AssetServer>) {
    let handle = ass.load("models/gltf/world.gltf");
    cmds.insert_resource(GltfHandle(handle));
}

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum TempAppState {
    #[default]
    Loading,
    Loaded,
}

fn check_assets_ready(
    mut state: ResMut<NextState<TempAppState>>,
    server: Res<AssetServer>,
    loading: Res<GltfHandle>,
) {
    use bevy::asset::LoadState;

    match server.get_group_load_state(vec![loading.0.id()]) {
        LoadState::Failed => {
            // one of our assets had an error
            panic!("😭 Failed loading asset");
        }
        LoadState::Loaded => state.set(TempAppState::Loaded),
        _ => (),
    }
}

fn load_gltf_content(
    mut cmds: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    gltfs: Res<Assets<Gltf>>,
    gltf_handle: Res<GltfHandle>,
) {
    let gltf = gltfs
        .get(&gltf_handle.0)
        .expect("😭 Can not load world gltf file!");

    let mut my_assets = PinballDefenseGltfAssets::default();
    for (i, field) in PinballDefenseGltfAssets::default()
        .iter_fields()
        .enumerate()
    {
        let name = my_assets
            .name_at(i)
            .unwrap_or_else(|| panic!("😭 No name at index {i}"))
            .to_string();
        match field.type_name() {
            "bevy_asset::handle::Handle<bevy_render::mesh::mesh::Mesh>" => {
                let mesh = gltf_meshes
                    .get(
                        gltf.named_meshes
                            .get(&name)
                            .unwrap_or_else(|| panic!("😭 No mesh with name {name}")),
                    )
                    .unwrap_or_else(|| panic!("😭 Can not load mesh with name {name}"))
                    .primitives[0]
                    .mesh
                    .clone();
                my_assets
                    .field_at_mut(i)
                    .unwrap_or_else(|| panic!("😭 No mesh at position {i}"))
                    .set(Box::new(mesh))
                    .unwrap_or_else(|error| {
                        panic!("😭 Not able to set mesh at position {i}: {error:?}")
                    });
            }
            "bevy_asset::handle::Handle<bevy_pbr::pbr_material::StandardMaterial>" => {
                let material = gltf
                    .named_materials
                    .get(&name)
                    .unwrap_or_else(|| panic!("😭 No material with name {name}"))
                    .clone();
                my_assets
                    .field_at_mut(i)
                    .unwrap_or_else(|| panic!("😭 No material at position {i}"))
                    .set(Box::new(material))
                    .unwrap_or_else(|error| {
                        panic!("😭 Not able to set material at position {i}: {error:?}")
                    });
            }
            type_name => log!("🐱 Unknown type in asset struct: {}", type_name),
        }
    }
    cmds.insert_resource(my_assets);
    game_state.set(GameState::Ingame);
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
