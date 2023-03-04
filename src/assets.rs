use crate::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct PinballDefenseAssets {
    #[asset(path = "models/pinball_world_1.gltf#Mesh0/Primitive0")]
    //#[asset(path = "models/ape.gltf#Mesh0/Primitive0")]
    pub world_1_mesh: Handle<Mesh>,
}
