use crate::prelude::*;
use crate::AppState;
use bevy::asset::LoadState;
use bevy::gltf::{Gltf, GltfMesh};
pub use bevy_asset_loader::prelude::*;

#[derive(Resource, AssetCollection, Default)]
pub struct PinballDefenseAssets {
    // Other
    #[asset(path = "textures/skybox.png")]
    pub skybox: Handle<Image>,

    #[asset(path = "fonts/PressStart2P-Regular.ttf")]
    pub digital_font: Handle<Font>,
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
    pub foundation_lid_bottom: Handle<Mesh>,
    pub foundation_lid_top: Handle<Mesh>,
    pub foundation_ring: Handle<Mesh>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GltfLoadState>()
            .add_state::<AssetsLoadState>()
            .init_resource::<GltfHandle>()
            .add_loading_state(
                LoadingState::new(AssetsLoadState::Loading)
                    .continue_to_state(AssetsLoadState::Finished),
            )
            .add_collection_to_loading_state::<_, PinballDefenseAssets>(AssetsLoadState::Loading)
            .add_systems(Startup, init_gltf_load)
            .add_systems(
                Update,
                check_assets_ready.run_if(in_state(GltfLoadState::Loading)),
            )
            .add_systems(OnEnter(GltfLoadState::GltfLoaded), load_gltf_content)
            .add_systems(OnEnter(AssetsLoadState::Finished), set_appstate_if_finished)
            .add_systems(OnEnter(GltfLoadState::Finished), set_appstate_if_finished);
    }
}

fn set_appstate_if_finished(
    mut app_state: ResMut<NextState<AppState>>,
    gltf_load_state: Res<State<GltfLoadState>>,
    load_state: Res<State<AssetsLoadState>>,
) {
    if *gltf_load_state == GltfLoadState::Finished && *load_state == AssetsLoadState::Finished {
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
enum GltfLoadState {
    #[default]
    Loading,
    GltfLoaded,
    Finished,
}

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum AssetsLoadState {
    #[default]
    Loading,
    Finished,
}

fn check_assets_ready(
    mut state: ResMut<NextState<GltfLoadState>>,
    server: Res<AssetServer>,
    loading: Res<GltfHandle>,
) {
    match server.get_group_load_state(vec![loading.0.id()]) {
        LoadState::Failed => panic!("😭 Failed loading asset"),
        LoadState::Loaded => state.set(GltfLoadState::GltfLoaded),
        _ => (),
    }
}

fn load_gltf_content(
    mut cmds: Commands,
    mut state: ResMut<NextState<GltfLoadState>>,
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
    state.set(GltfLoadState::Finished);
}

fn prop_name(assets: &PinballDefenseGltfAssets, i: usize) -> String {
    assets
        .name_at(i)
        .unwrap_or_else(|| panic!("😭 No name at index {i}"))
        .to_string()
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

fn set_field(assets: &mut PinballDefenseGltfAssets, i: usize, obj: Box<dyn Reflect>) {
    assets
        .field_at_mut(i)
        .unwrap_or_else(|| panic!("😭 No object at position {i}"))
        .set(obj)
        .unwrap_or_else(|error| panic!("😭 Not able to set object at position {i}: {error:?}"));
}
