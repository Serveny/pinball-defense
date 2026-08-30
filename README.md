:warning: **Work in progress**

# Pinball Defense

<p align="center">
  <img src="assets/demo-animation.gif"/>
</p>

A tower denfense game, but you can only interact with the world by hitting things with the pinball
Story: You want to steal resources on lava planet, but native monsters come in waves to stop you.

## Development

- Run `git lfs checkout` once, before you can use `cargo run` (Otherwise missing assets)
- **Export world collider**: Open blender and select `world_1_frame_collider` object, then run `bpy_export_mesh_as_polyline.py`
- **Export 3D models**: Open blender and run `bpy_export_all_gltf.py`
