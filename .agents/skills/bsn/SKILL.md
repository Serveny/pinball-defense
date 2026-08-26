---
name: bsn
description: Bevy Scene Notation (bsn!) reference for Bevy 0.19. Use when writing or editing UI, spawning entities, or any bsn! macro usage in this project — buttons, text, layouts, scene functions, spawn_scene/apply_scene/queue_apply_scene, ChildSpawnerCommands.
---

# BSN (Bevy Scene Notation) — Bevy 0.19

The `bsn!` macro is Bevy 0.19's scene DSL (see `bevy-0.19.1/_release-content/release-notes/next-generation-scenes.md`). Used throughout `src/` for UI and entity spawning. Facts verified against `bevy_scene-0.19.1` / `bevy_scene_macros-0.19.1` source.

## Syntax

- **`#Name` prefix** (e.g. `#Button`) = entity reference syntax: assigns `Name("Button")` AND defines a reference usable elsewhere in the same scene. NOT a required-component marker.
- **`Type::function(...)`** (e.g. `BorderColor::from(color)`) = template constructor; parsed as a function call whose return value becomes a component. Enum variants are plain Rust paths (`Team::Blue`, `Justify::Center`).
- **Any `Clone + Default` component is bsn-able** (blanket `FromTemplate`). Widgets like `Scrollbar`/`ScrollbarThumb`/`Slider` work directly in `bsn!` — no need to fall back to `cmds.spawn()` just because a field isn't `Default`; use `{expr}` for those fields.
- **`{expr}`** = raw Rust expression evaluated inline (e.g. `BackgroundColor({Color::srgba_u8(23,24,26,120)})`, `Text({my_string})`). Also the way to set a non-`Default` field value, e.g. an `Entity` (`target: {entity}`) or a `Handle`.
- **`Children [ ... ]`** = relationship list. `({scene_var})` embeds a previously-built `impl Scene` value; the parens are optional grouping. `(Node {...} Children [...])` groups one entity's components.
- **`Text`** accepts a `String`/`&str` directly (`From<String>`/`From<&str>` impls in `bevy_ui`); `\n` renders as line breaks (Parley layout). No `TextSpan` needed unless styling changes mid-text.

## Spawning API (in `bevy_scene-0.19.1/src/spawn.rs`)

- `Commands::spawn_scene(scene) -> EntityCommands` — new entity + apply scene.
- `EntityCommands::apply_scene(scene)` — apply to existing entity immediately.
- `EntityCommands::queue_apply_scene(scene)` — apply now, or queue until `.bsn` asset deps load.
- `ChildSpawnerCommands` = `RelatedSpawnerCommands<'w, ChildOf>` (what `.with_children(|p| …)` gives you).
- `Scene` trait = composable component "patch"; `fn foo() -> impl Scene { bsn! { … } }` is a reusable scene function.

## Common pattern in this repo

```rust
cmds.spawn_scene(bsn! { … }).with_children(|p| {
    p.spawn_empty().queue_apply_scene(bsn! { … });
});
```

## Reusable button helper

`src/menu/tools/menu_btn.rs` provides `menu_btn::scene(MenuAction, ButtonStyle, assets, margin) -> impl Scene` and `menu_btn::system` (writes `MenuAction` messages on press). Prefer this over hand-rolled buttons.

## Scrolling & scrollbars

All in `bevy::ui_widgets` (re-exported by `bevy`; `bevy_ui_widgets` is a default dep). `ScrollAreaPlugin` + `ScrollbarPlugin` are already registered via `DefaultPlugins` → `UiWidgetsPlugins` — no manual plugin wiring.

- **Scrollable container**: add `overflow: Overflow::scroll_y()` (or `scroll()`, `scroll_x()`) to the `Node`, plus the `ScrollArea` component. `ScrollArea` has `#[require(ScrollPosition)]`, so `ScrollPosition` is auto-inserted — do NOT add it manually. The container needs a bounded height (e.g. `top`/`bottom: 0` on an absolute node) or it won't overflow.
- **`Overflow` constructors** (`bevy::ui::Overflow`): `visible()`, `clip()`, `clip_x()`, `clip_y()`, `hidden()`, `hidden_x()`, `hidden_y()`, `scroll()`, `scroll_x()`, `scroll_y()`.
- **Scrollbar widget** (`bevy::ui_widgets::{Scrollbar, ScrollbarThumb, ControlOrientation}`): `Scrollbar { target: Entity, orientation, min_thumb_length }` controls the scroll area's `ScrollPosition` directly (drag + track-click page up/down, via observers). `ScrollbarThumb` is the moving child — it has **no `Node` component**; only `border_radius`/`border` styling, size/position set by `update_scrollbar_thumb`. `Scrollbar` needs the scroll-area entity id, so capture it: `let id = cmds.spawn_scene(bsn!{…}).id();`. Both widgets are bsn-able; set `target: {id}` via `{expr}`.
- **Z-order**: scrollbars must sit above the content to receive pointer events (drag). Add `GlobalZIndex(1)` to the scrollbar node; otherwise buttons underneath swallow the drag. `GlobalZIndex` also reorders rendering + picking together.
- **Show-only-when-scrollable**: compare `ComputedNode::content_size().y > size().y` on the target; toggle `Visibility` accordingly.

See `src/menu/tools/scrollbar.rs` for the working helper (`spawn(cmds, target)` + `update_visibility`).
