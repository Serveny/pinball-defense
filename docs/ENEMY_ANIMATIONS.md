# Enemy Animations

Normal enemies use `assets/models/normal_iron_cage.glb`. Tank and Speeder keep
their primitive visuals until animated models are configured for them.

The controller lives in `src/game/enemy/animation.rs`. It instantiates a GLTF
world beneath the enemy, waits for `WorldInstanceReady`, and binds the model's
named clips to its animation player. Graphs and source assets are shared;
phase, heading, blend weights and material overrides belong to each instance.
The primitive stays visible until the model is ready.

## Motion

Gameplay and Avian2D retain ownership of the actor transform. The visual child
rotates toward observed movement in the XY plane, with +Z up. The model's
orientation maps its GLTF +Z forward and +Y up to the game's axes. Its ground
offset places the feet on the table without moving the actor or health bar.

Walking phase advances from actual distance travelled, using the model's
reference speed, uniform scale and walk duration. At scale 1.2 the Iron Cage
reference speed is 0.0432 world units/second. Faster movement advances the same
cycle faster; slowdown reduces the rate. Zero displacement preserves heading,
and paused virtual time preserves phase. No root motion is applied.

Left/right curve clips blend according to signed visual yaw speed. All clips
sample the same normalized phase, including when their durations differ.
Weights ease over approximately 0.08 seconds. Optional idle/turn clips are
selected when their corresponding motion is supplied; the current path walker
turns while moving and does not issue stationary turn commands.

## Another Enemy Type

1. Add its GLB handle to `PinballDefenseAssets` so the loading screen waits for it.
2. In `prepare_models`, build and register an `EnemyModel` under its `EnemyKind`.
3. Supply a `ModelConfig`: named clips, scale, ground offset, orientation,
   authored walk speed, curve yaw speed and stationary turn yaw speed.

Only `Clip::Walk` is required. Omit unavailable optional clips from the config;
missing curve clips fall back to walking, and a missing idle clip holds the
walk pose. Configured names must exist and clips must have positive duration.
Named clips avoid dependence on GLTF animation ordering.

Visual entities and controller state are transient and have no `Save` marker.
Saved enemies recreate their visual when loaded. Model materials are copied
once per material per instance, allowing freeze tint to restore the original
colors without modifying the source asset or other instances.

The GLB contains PBR base colors and sensor emission. The original procedural
Blender rust shader has not been baked into this export.
