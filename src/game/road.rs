use crate::prelude::*;

pub fn spawn_road(spawner: &mut ChildSpawnerCommands, assets: &PinballDefenseGltfAssets) {
    spawner.spawn((
        Name::new("Road Mesh"),
        Mesh3d(assets.road_mesh.clone()),
        MeshMaterial3d(assets.road_material.clone()),
    ));
}
