---
name: playtest
description: Playtest, Vorschau, visual verification and in-game testing for Pinball-Defense. Use when testing gameplay, VFX, tower upgrades, enemy visuals or UI in the running Bevy game; create reproducible savegame scenes, inspect GPU logs and capture screenshots.
---

# Playtest

Test the feature in the actual game with a small, reproducible scene. Prefer the existing save/load path over adding a preview mode, debug plugin or test framework.

## 1. Choose a scene

- Run from the worktree containing the changes, following the `worktree-slot` skill. Reuse the current task's slot.
- State what should be observable before launching: source, target, range, timing or UI state.
- Reuse a scene in `scenes/` or create a temporary `.ron` save under `/tmp/opencode`. Preserve the user's save slots.
- Inspect a current save and the relevant logical components before constructing new scenes. Query graphify first for codebase questions; load `bsn` if adding spawning code.
- Keep only the entities and resources needed for the test. The game's restoration systems must build their usual visuals and physics.
- Include contrasting cases: affected/unaffected enemies, minimum/maximum upgrade levels, active/inactive states, or overlapping towers where relevant.
- Set both `speed` and `current_speed` to `0` for stationary enemy comparisons. Suppress automatic waves through the inspected wave state. This demonstrates appearance, not movement or entry/exit behavior; use a live scene for those checks.

The bundled Microwave scene contains level-1 and level-10 towers, stationary enemies and a deliberately out-of-range enemy:

```bash
cargo run -- --load .agents/skills/playtest/scenes/microwave.ron
```

Adapt the scene if the save schema changes. Do not alter gameplay code just to accommodate an outdated fixture.

## 2. Launch the real renderer

- Use `cargo run -- --load <scene.ron>` from the worktree root. Cargo supplies the dynamic-library and asset-root environment; invoking `target/debug/pinball-defense` directly can miss both.
- Give builds up to `612000` ms. When using a runtime timeout, finish the build first so compilation does not consume the playtest window.
- Capture stdout and stderr to a task-specific log. Wait for asset loading to finish and for the feature to be active before taking evidence.
- Inspect logs for panics, missing assets, shader errors and GPU validation failures. Shader-source generation tests alone do not validate the actual GPU pipeline.
- Check focus and pause state. A paused or unfocused game is insufficient evidence for animation. Resume the game and observe successive frames when motion matters.

Example bounded Linux run, after building:

```bash
cargo build
timeout 60s cargo run -- --load .agents/skills/playtest/scenes/microwave.ron > /tmp/opencode/playtest-microwave.log 2>&1
```

## 3. Capture and inspect

Use available application screenshot tools. On Linux/X11, `xdotool` plus ImageMagick worked for this project. Check tool availability first. If running under Wayland and an X11 capture is needed, launch this test process with `env -u WAYLAND_DISPLAY cargo run -- --load <scene.ron>`; leave the user's desktop configuration alone.

```bash
xdotool search --classname pinball-defense
import -window <verified-window-id> /tmp/opencode/playtest-microwave.png
```

- Verify the window belongs to your test process; multiple game instances may exist. Capture the application window rather than the whole desktop.
- Use a camera angle that exposes the feature. If the overview is too distant, use the existing camera controls for a close-up, then check readability at normal gameplay distance.
- Open the captured image with an image-capable tool. Saving an image without inspecting it is not visual verification.
- Check origin/attachment, aim, scale, actual affected targets, range boundaries, upgrade differences and visual clutter. Physics debug lines are not VFX; identify which objects belong to the feature before changing anything.
- For animation, inspect several frames or a recording. A single still cannot establish direction, cadence or the absence of flicker.
- If the result is wrong, make the smallest fix and repeat the same scene and camera comparison.

## 4. Finish

- Check restoration through loading and fresh spawning when adding child visuals. Use relevant unit tests for targeting and state transitions; a screenshot does not prove logical correctness.
- Run the checks appropriate to the changed code, normally `cargo clippy` and targeted tests. Update the code graph after source changes.
- Stop only the game process you started. Preserve screenshots, logs and the launch command until the user can review them; leave save slots untouched.
- Report the worktree, scene command, observed result and checks passed. Distinguish screenshot observations, live behavior, GPU validation and unit-test results. If a display or GPU is unavailable, report the blocker and which checks ran instead.
- Keep a new scene in `scenes/` only when it provides a reusable regression case; otherwise keep it temporary. Do not commit or merge unless requested.
