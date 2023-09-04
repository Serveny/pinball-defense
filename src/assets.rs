use crate::prelude::*;
use crate::utils::reflect::get_field_mut;
use crate::utils::reflect::prop_name;
use crate::utils::reflect::set_field;
use crate::AppState;
use bevy::asset::Asset;
use bevy::asset::LoadState;
use bevy::gltf::{Gltf, GltfMesh};
use bevy::utils::HashMap;
pub use bevy_asset_loader::prelude::*;
use rand::seq::SliceRandom;
use std::env;
use std::path::PathBuf;

#[derive(Resource, AssetCollection, Default)]
pub struct PinballDefenseAssets {
    // Other
    #[asset(path = "textures/skybox.png")]
    pub skybox: Handle<Image>,

    #[asset(path = "fonts/PressStart2P-Regular.ttf")]
    pub digital_font: Handle<Font>,

    #[asset(path = "fonts/hemi_head.otf")]
    pub menu_font: Handle<Font>,
}

#[derive(Resource, Reflect, Default)]
pub struct PinballDefenseGltfAssets {
    pub analog_counter_10_digit_casing: Handle<Mesh>,
    pub analog_counter_cylinder: Handle<Mesh>,
    pub point_sign: Handle<Mesh>,
    pub analog_counter_casing_2_digit: Handle<Mesh>,
    pub level_sign: Handle<Mesh>,
    pub analog_counter_casing_10_digit_material: Handle<StandardMaterial>,
    pub analog_counter_cylinder_material: Handle<StandardMaterial>,
    pub analog_counter_casing_2_digit_material: Handle<StandardMaterial>,
    pub points_sign_material: Handle<StandardMaterial>,
    pub level_sign_material: Handle<StandardMaterial>,

    // Flipper
    pub flipper_left: Handle<Mesh>,
    pub flipper_right: Handle<Mesh>,
    pub flipper_material: Handle<StandardMaterial>,

    // Menu Elements
    pub pinball_menu_element: Handle<Mesh>,
    pub pinball_menu_element_collider: Handle<Mesh>,
    pub pinball_menu_element_gun_material: Handle<StandardMaterial>,
    pub pinball_menu_element_tesla_material: Handle<StandardMaterial>,
    pub pinball_menu_element_microwave_material: Handle<StandardMaterial>,
    pub pinball_menu_element_damage_upgrade_mat: Handle<StandardMaterial>,
    pub pinball_menu_element_range_upgrade_mat: Handle<StandardMaterial>,

    // Pinball Plate
    pub world_1: Handle<Mesh>,
    pub world_1_menu_glass: Handle<Mesh>,
    pub world_1_ground_collider: Handle<Mesh>,
    pub world_1_frame_collider: Handle<Mesh>,
    pub world_1_material: Handle<StandardMaterial>,

    // Progress bar
    pub progress_bar: Handle<Mesh>,
    pub progress_bar_frame: Handle<Mesh>,

    // Road
    pub road_mesh: Handle<Mesh>,
    pub road_material: Handle<StandardMaterial>,

    // Tower
    pub tower_base: Handle<Mesh>,
    pub tower_microwave_top: Handle<Mesh>,
    pub tower_mg_mounting: Handle<Mesh>,
    pub tower_mg_head: Handle<Mesh>,
    pub tower_mg_barrel: Handle<Mesh>,
    pub tower_tesla_top: Handle<Mesh>,

    // Foundation
    pub foundation_lid_bottom: Handle<Mesh>,
    pub foundation_lid_top: Handle<Mesh>,
    pub foundation_ring: Handle<Mesh>,
    pub foundation_lid_material: Handle<StandardMaterial>,
    pub foundation_ring_material: Handle<StandardMaterial>,
    pub build_mark: Handle<Mesh>,
    pub build_mark_material: Handle<StandardMaterial>,

    // lamp
    pub lamp_bulb: Handle<Mesh>,
    pub lamp_thread: Handle<Mesh>,
    pub lamp_thread_material: Handle<StandardMaterial>,
}

#[derive(Resource, Reflect, Default)]
pub struct PinballDefenseAudioAssets {
    pub flipper_press: Handles<AudioSource>,
    pub flipper_release: Handles<AudioSource>,
    pub background_music: Handle<AudioSource>,
    pub ball_release: Handle<AudioSource>,
    pub tower_hit: Handles<AudioSource>,
    pub ball_hits_end: Handle<AudioSource>,
    pub ball_hits_enemy: Handle<AudioSource>,
    pub enemy_reach_end: Handle<AudioSource>,
    pub tower_build: Handle<AudioSource>,
    pub tower_upgrade_range: Handle<AudioSource>,
    pub tower_upgrade_damage: Handle<AudioSource>,
}

#[derive(Resource, Reflect, Default)]
pub struct AudioAssetId(HashMap<Entity, String>);

#[derive(Reflect, Clone)]
pub struct Handles<T: Asset>(pub Vec<Handle<T>>);

impl<T: Asset> Default for Handles<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: Asset> Handles<T> {
    pub fn choose(&self) -> &Handle<T> {
        self.0
            .choose(&mut rand::thread_rng())
            .expect("😥 Vector empty, no sound to choose")
    }

    fn from(value: Handle<T>) -> Self {
        Self(vec![value])
    }
}
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AssetsInternalLoadState>()
            .add_state::<AssetsLoadState>()
            .init_resource::<GltfHandle>()
            .add_loading_state(
                LoadingState::new(AssetsLoadState::Loading)
                    .continue_to_state(AssetsLoadState::Finished),
            )
            .add_collection_to_loading_state::<_, PinballDefenseAssets>(AssetsLoadState::Loading)
            .add_systems(Startup, (init_gltf_load, add_audio_resource))
            .add_systems(
                Update,
                check_assets_ready.run_if(in_state(AssetsInternalLoadState::Loading)),
            )
            .add_systems(
                OnEnter(AssetsInternalLoadState::AssetServerFinished),
                (add_gltf_resource, add_audio_resource),
            )
            .add_systems(OnEnter(AssetsLoadState::Finished), set_appstate_if_finished)
            .add_systems(
                OnEnter(AssetsInternalLoadState::Finished),
                set_appstate_if_finished,
            );
    }
}

fn set_appstate_if_finished(
    mut app_state: ResMut<NextState<AppState>>,
    gltf_load_state: Res<State<AssetsInternalLoadState>>,
    load_state: Res<State<AssetsLoadState>>,
) {
    if *gltf_load_state == AssetsInternalLoadState::Finished
        && *load_state == AssetsLoadState::Finished
    {
        app_state.set(AppState::Game);
    }
}

#[derive(Resource, Default)]
struct GltfHandle(Handle<Gltf>);

fn init_gltf_load(mut cmds: Commands, ass: Res<AssetServer>) {
    let handle = ass.load("models/gltf/world.glb");
    cmds.insert_resource(GltfHandle(handle));
}

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum AssetsInternalLoadState {
    #[default]
    Loading,
    AssetServerFinished,
    Finished,
}

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum AssetsLoadState {
    #[default]
    Loading,
    Finished,
}

fn check_assets_ready(
    mut state: ResMut<NextState<AssetsInternalLoadState>>,
    server: Res<AssetServer>,
    loading: Res<GltfHandle>,
) {
    match server.get_group_load_state(vec![loading.0.id()]) {
        LoadState::Failed => panic!("😭 Failed loading asset"),
        LoadState::Loaded => state.set(AssetsInternalLoadState::AssetServerFinished),
        _ => (),
    }
}

fn add_gltf_resource(
    mut cmds: Commands,
    mut state: ResMut<NextState<AssetsInternalLoadState>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    gltfs: Res<Assets<Gltf>>,
    gltf_handle: Res<GltfHandle>,
) {
    let gltf = gltfs
        .get(&gltf_handle.0)
        .expect("😭 Can not load world gltf file!");

    let mut assets = PinballDefenseGltfAssets::default();
    for (i, field) in PinballDefenseGltfAssets::default()
        .iter_fields()
        .enumerate()
    {
        let prop_name = prop_name(&assets, i);
        match field.type_name() {
            "bevy_asset::handle::Handle<bevy_render::mesh::mesh::Mesh>" => {
                let mesh = mesh(&prop_name, gltf, &gltf_meshes);
                set_field(&mut assets, i, Box::new(mesh));
            }
            "bevy_asset::handle::Handle<bevy_pbr::pbr_material::StandardMaterial>" => {
                let material = material(&prop_name, gltf);
                set_field(&mut assets, i, Box::new(material));
            }
            type_name => println!("🐱 Unknown type in asset struct: {}", type_name),
        }
    }
    cmds.insert_resource(assets);
    state.set(AssetsInternalLoadState::Finished);
}

fn mesh(mesh_name: &str, gltf: &Gltf, gltf_meshes: &Assets<GltfMesh>) -> Handle<Mesh> {
    gltf_meshes
        .get(
            gltf.named_meshes
                .get(mesh_name)
                .unwrap_or_else(|| panic!("😭 No mesh with name {mesh_name}")),
        )
        .unwrap_or_else(|| panic!("😭 Can not load mesh with name {mesh_name}"))
        .primitives[0]
        .mesh
        .clone()
}

fn material(material_name: &str, gltf: &Gltf) -> Handle<StandardMaterial> {
    gltf.named_materials
        .get(material_name)
        .unwrap_or_else(|| panic!("😭 No material with name {material_name}"))
        .clone()
}

fn audio_assets_path(sub_dir: Option<&str>) -> PathBuf {
    env::current_exe()
        .expect("😥 No current exe")
        .parent()
        .expect("😥 No parent folder of current exe")
        .join(PathBuf::from(format!(
            "../../assets/audio/{}",
            if let Some(sub_dir) = sub_dir {
                sub_dir
            } else {
                ""
            }
        )))
}

#[derive(Resource, Default)]
pub struct PinballDefenseAudioSources(pub HashMap<String, Handles<AudioSource>>);

fn add_audio_resource(mut cmds: Commands, ass: Res<AssetServer>) {
    let audio_dir = audio_assets_path(None);
    let file_name_paths: Vec<(String, PathBuf)> = file_paths(audio_dir);

    let mut audio_assets = PinballDefenseAudioAssets::default();
    let mut handles_map = PinballDefenseAudioSources::default();
    for (i, field) in PinballDefenseAudioAssets::default()
        .iter_fields()
        .enumerate()
    {
        let prop_name = prop_name(&audio_assets, i);
        match field.type_name() {
            "pinball_defense::assets::Handles<bevy_audio::audio_source::AudioSource>" => {
                let audio_dir = audio_assets_path(Some(&prop_name));
                let field: &mut Handles<AudioSource> = get_field_mut(&mut audio_assets, i)
                    .downcast_mut()
                    .expect("😥 Unexpected: Handles type is no handles type.");
                for (_, path) in file_paths(audio_dir) {
                    let handle = ass.load(path);
                    field.0.push(handle);
                }
                handles_map.0.insert(prop_name.clone(), field.clone());
            }
            "bevy_asset::handle::Handle<bevy_audio::audio_source::AudioSource>" => {
                let file_path = path_by_name(&prop_name, &file_name_paths);
                let handle: Handle<AudioSource> = ass.load(file_path);
                handles_map
                    .0
                    .insert(prop_name.clone(), Handles::from(handle.clone()));
                set_field(&mut audio_assets, i, Box::new(handle));
            }
            type_name => println!("🔊 Unknown type in audio asset struct: {}", type_name),
        }
    }
    cmds.insert_resource(audio_assets);
    cmds.insert_resource(handles_map);
}

fn path_by_name(name: &str, files: &[(String, PathBuf)]) -> PathBuf {
    files
        .iter()
        .find(|file| file.0 == name)
        .unwrap_or_else(|| panic!("😥 No file with name {name} in audio folder."))
        .1
        .clone()
}

fn file_paths(dir: PathBuf) -> Vec<(String, PathBuf)> {
    dir.read_dir()
        .unwrap_or_else(|err| panic!("😥 Can not read audio directory {dir:?} with error {err}"))
        .map(|file| {
            let file = file.as_ref().expect("😥 Can not read file");
            (
                file.path()
                    .file_stem()
                    .expect("😥 Can not stem file")
                    .to_str()
                    .expect("😥 Can not convert os string to string")
                    .to_string(),
                file.path(),
            )
        })
        .collect()
}
