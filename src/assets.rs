use crate::prelude::*;
pub use bevy_asset_loader::prelude::*;

#[derive(Resource, AssetCollection)]
pub struct PinballDefenseAssets {
    // Other
    #[asset(path = "textures/skybox.png")]
    pub skybox: Handle<Image>,

    #[asset(path = "fonts/PressStart2P-Regular.ttf")]
    pub digital_font: Handle<Font>,

    // Road
    #[asset(path = "models/gltf/road.gltf#Mesh0/Primitive0")]
    pub road_mesh: Handle<Mesh>,
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
    pub points_sign_material: Handle<StandardMaterial>,
    pub analog_counter_casing_2_digit_material: Handle<StandardMaterial>,
    pub level_sign_material: Handle<StandardMaterial>,

    // Flipper
    pub flipper_left: Handle<Mesh>,
    pub flipper_right: Handle<Mesh>,

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
