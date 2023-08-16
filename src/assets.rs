use crate::prelude::*;
pub use bevy_asset_loader::prelude::*;

#[derive(Resource, AssetCollection)]
pub struct PinballDefenseAssets {
    // Pinball World
    #[asset(path = "models/pinball_world_1.gltf#Mesh0/Primitive0")]
    pub world_1: Handle<Mesh>,

    #[asset(path = "models/pinball_world_1.gltf#Mesh1/Primitive0")]
    pub world_1_menu_glass: Handle<Mesh>,

    #[asset(path = "models/pinball_world_1.gltf#Mesh2/Primitive0")]
    pub world_1_point_display: Handle<Mesh>,

    #[asset(path = "models/pinball_world_1.gltf#Mesh3/Primitive0")]
    pub world_1_frame_collider: Handle<Mesh>,

    #[asset(path = "models/pinball_world_1.gltf#Mesh4/Primitive0")]
    pub world_1_ground_collider: Handle<Mesh>,

    #[asset(path = "models/pinball_world_1.gltf#Mesh5/Primitive0")]
    pub world_1_rebound_left_collider: Handle<Mesh>,

    #[asset(path = "models/pinball_world_1.gltf#Mesh6/Primitive0")]
    pub world_1_rebound_right_collider: Handle<Mesh>,

    #[asset(path = "models/pinball_world_1.gltf#Material0")]
    pub world_1_material: Handle<StandardMaterial>,

    // Other
    #[asset(path = "textures/skybox.png")]
    pub skybox: Handle<Image>,

    #[asset(path = "fonts/PressStart2P-Regular.ttf")]
    pub digital_font: Handle<Font>,

    // Road
    #[asset(path = "models/pinball_world_1_road.gltf#Mesh0/Primitive0")]
    pub road_mesh: Handle<Mesh>,

    #[asset(path = "models/pinball_world_1.gltf#Animation0")]
    pub road_path: Handle<AnimationClip>,

    // Flipper
    #[asset(path = "models/flippers.gltf#Mesh0/Primitive0")]
    pub flipper_right: Handle<Mesh>,

    #[asset(path = "models/flippers.gltf#Mesh1/Primitive0")]
    pub flipper_left: Handle<Mesh>,

    // Tower
    #[asset(path = "models/towers/tower_base.gltf#Mesh0/Primitive0")]
    pub tower_base: Handle<Mesh>,

    #[asset(path = "models/towers/tower_microwave.gltf#Mesh0/Primitive0")]
    pub tower_microwave_top: Handle<Mesh>,

    #[asset(path = "models/towers/tower_machine_gun.gltf#Mesh0/Primitive0")]
    pub tower_mg_mounting: Handle<Mesh>,

    #[asset(path = "models/towers/tower_machine_gun.gltf#Mesh1/Primitive0")]
    pub tower_mg_head: Handle<Mesh>,

    #[asset(path = "models/towers/tower_machine_gun.gltf#Mesh2/Primitive0")]
    pub tower_mg_barrel: Handle<Mesh>,

    #[asset(path = "models/towers/tower_tesla.gltf#Mesh1/Primitive0")]
    pub tower_tesla_top: Handle<Mesh>,

    #[asset(path = "models/towers/tower_foundation.gltf#Mesh0/Primitive0")]
    pub tower_foundation_bottom: Handle<Mesh>,

    #[asset(path = "models/towers/tower_foundation.gltf#Mesh1/Primitive0")]
    pub tower_foundation_top: Handle<Mesh>,

    #[asset(path = "models/towers/tower_foundation.gltf#Mesh2/Primitive0")]
    pub tower_foundation_ring: Handle<Mesh>,

    #[asset(path = "models/progress_bar.gltf#Mesh0/Primitive0")]
    pub progress_bar: Handle<Mesh>,

    #[asset(path = "models/progress_bar.gltf#Mesh1/Primitive0")]
    pub progress_bar_frame: Handle<Mesh>,

    #[asset(path = "models/pinball_world_menu.gltf#Mesh1/Primitive0")]
    pub pinball_menu_element_collider: Handle<Mesh>,

    #[asset(path = "models/pinball_world_menu.gltf#Mesh2/Primitive0")]
    pub pinball_menu_element: Handle<Mesh>,

    // Menu Elements
    #[asset(path = "models/pinball_world_menu.gltf#Material0")]
    pub pinball_menu_element_gun_material: Handle<StandardMaterial>,

    #[asset(path = "models/pinball_world_menu.gltf#Material1")]
    pub pinball_menu_element_tesla_material: Handle<StandardMaterial>,

    #[asset(path = "models/pinball_world_menu.gltf#Material2")]
    pub pinball_menu_element_microwave_material: Handle<StandardMaterial>,

    // Analog number counter
    #[asset(path = "models/analog_counter.gltf#Mesh0/Primitive0")]
    pub analog_counter_casing: Handle<Mesh>,

    #[asset(path = "models/analog_counter.gltf#Mesh1/Primitive0")]
    pub analog_counter_cylinder: Handle<Mesh>,

    #[asset(path = "models/analog_counter.gltf#Material0")]
    pub analog_counter_casing_material: Handle<StandardMaterial>,

    #[asset(path = "models/analog_counter.gltf#Material1")]
    pub analog_counter_cylinder_material: Handle<StandardMaterial>,
}
