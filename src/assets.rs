use crate::prelude::*;

#[derive(Resource, AssetCollection)]
pub struct PinballDefenseAssets {
    #[asset(path = "models/pinball_plate_1.gltf#Mesh0/Primitive0")]
    pub world_1_mesh: Handle<Mesh>,

    #[asset(path = "models/pinball_plate_1.gltf#Mesh1/Primitive0")]
    pub world_1_collision_mesh: Handle<Mesh>,

    #[asset(path = "textures/skybox.png")]
    pub skybox: Handle<Image>,

    #[asset(path = "fonts/Quicksand-Regular.ttf")]
    pub font: Handle<Font>,

    #[asset(path = "models/pinball_world_1_road.gltf#Mesh0/Primitive0")]
    pub road_mesh: Handle<Mesh>,

    #[asset(path = "models/pinball_plate_1.gltf#Animation0")]
    pub road_path: Handle<AnimationClip>,

    #[asset(path = "models/flippers.gltf#Mesh0/Primitive0")]
    pub flipper_right: Handle<Mesh>,

    #[asset(path = "models/flippers.gltf#Mesh1/Primitive0")]
    pub flipper_left: Handle<Mesh>,

    // Maybe different bases?
    #[asset(path = "models/towers/tower_microwave.gltf#Mesh0/Primitive0")]
    pub tower_base: Handle<Mesh>,

    #[asset(path = "models/towers/tower_microwave.gltf#Mesh1/Primitive0")]
    pub tower_microwave_top: Handle<Mesh>,

    #[asset(path = "models/towers/tower_machine_gun.gltf#Mesh1/Primitive0")]
    pub tower_mg_mounting: Handle<Mesh>,

    #[asset(path = "models/towers/tower_machine_gun.gltf#Mesh2/Primitive0")]
    pub tower_mg_head: Handle<Mesh>,

    #[asset(path = "models/towers/tower_machine_gun.gltf#Mesh3/Primitive0")]
    pub tower_mg_barrel: Handle<Mesh>,

    #[asset(path = "models/towers/tower_tesla.gltf#Mesh1/Primitive0")]
    pub tower_tesla_top: Handle<Mesh>,

    #[asset(path = "models/towers/tower_foundation.gltf#Mesh0/Primitive0")]
    pub tower_foundation_bottom: Handle<Mesh>,

    #[asset(path = "models/towers/tower_foundation.gltf#Mesh1/Primitive0")]
    pub tower_foundation_top: Handle<Mesh>,

    #[asset(path = "models/towers/tower_foundation.gltf#Mesh2/Primitive0")]
    pub tower_foundation_ring: Handle<Mesh>,

    #[asset(path = "models/towers/tower_foundation.gltf#Mesh3/Primitive0")]
    pub tower_foundation_progress_bar: Handle<Mesh>,
}
