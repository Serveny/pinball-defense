---
name: bsn
description: Bevy Scene Notation (bsn!) reference for Bevy 0.20. Use when writing or editing UI, spawning entities, or any bsn!/bsn_list! macro usage in this project — buttons, text, layouts, scene functions, spawn_scene/apply_scene/queue_apply_scene, ChildSpawnerCommands.
---

# BSN (Bevy Scene Notation) — Bevy 0.20

The `bsn!` macro is Bevy's scene DSL (see `bevy-0.20.1/_release-content/release-notes/bsn_syntax_improvements.md` and the 0.19→0.20 migration guide). Used throughout `src/` for UI and entity spawning. Facts verified against `bevy_scene-0.20.1` / `bevy_scene_macros-0.20.1` source.

## Syntax

- **`#Name` prefix** (e.g. `#Button`) = entity reference syntax: assigns `Name("Button")` AND defines a reference usable elsewhere in the same scene. NOT a required-component marker.
- **`@` = scene reference (required since 0.20)**: `@scene_variable`, `@scene_function()`, `@{scene_expression}`, `@"asset.bsn"`. Without `@` the item is treated as a component value, not a scene. Use this to embed reusable scene functions, e.g. `@menu_btn::scene(...)`.
- **Component values are plain Rust** (no `template_value(...)` wrapper anymore — remove any you see): `component_variable`, `component_function()`, `Transform::from_xyz(0., 1., 2.).looking_at(Vec3::ZERO, Vec3::Y)`, `node.clone()`. Method chains (`.foo()`) after a type/function become a value.
- **`Type::function(...)`** (e.g. `BorderColor::from(color)`) = template constructor; its return value becomes a component.
- **Enums**: only need `Component + Default + Clone` (no `VariantDefaults`/`FromTemplate`). Enum variants with fields must specify **every** field (`Foo::A { x: 1, y: 0 }`) — no per-variant field defaults anymore.
- **Any `Clone + Default` component is bsn-able** (blanket `FromTemplate`). Widgets like `Scrollbar`/`ScrollbarThumb`/`Slider` work directly in `bsn!` — no need to fall back to `cmds.spawn()` just because a field isn't `Default`; use `{expr}` for those fields. Struct fields can be partially set (`Node { width: px(10.) }`), unset fields keep their defaults.
- **`{expr}`** as a field value = raw Rust expression (e.g. `BackgroundColor({Color::srgba_u8(23,24,26,120)})`, `Text({my_string})`, `target: {entity}`, a `Handle`).
- **`Children [ ... ]`** = relationship list. Separate entities with **`--`**; `()` grouping and `,` separators are deprecated:
  ```rust
  Children [
      #Ok @button("Ok")
      --
      #Cancel @button("Cancel")
      --
      @{some_scene_var}
  ]
  ```
- **`bsn_list! { ... }`** for standalone lists (same `--` separator). Use `{}` braces for `bsn!`/`bsn_list!` — `[]`/`()` are warned against (rustfmt mangles them).
- **`Text`** accepts a `String`/`&str` directly; `\n` renders as line breaks. No `TextSpan` needed unless styling changes mid-text. `TextFont::default()` is now `FontSize::Rem(1.)` (scales with `RemSize`); set `FontSize::Px(..)` for fixed sizes.
- **`Name`** from `&str` requires `&'static str`; for runtime strings use `Name::new(s.to_owned())`.

## Spawning API (in `bevy_scene-0.20.1/src/spawn.rs`, unchanged from 0.19)

- `Commands::spawn_scene(scene) -> EntityCommands` — new entity + apply scene. `queue_spawn_scene` waits for `.bsn` asset deps.
- `Commands::spawn_scene_list(bsn_list!{…})` / `queue_spawn_scene_list` — spawn several root entities.
- `EntityCommands::apply_scene(scene)` — apply to existing entity immediately.
- `EntityCommands::queue_apply_scene(scene)` — apply now, or queue until `.bsn` asset deps load.
- `EntityCommands::queue_spawn_related_scenes::<Children>(bsn_list!{…})` — spawn related entities from a list.
- `ChildSpawnerCommands` = `RelatedSpawnerCommands<'w, ChildOf>` (what `.with_children(|p| …)` gives you).
- `Scene` trait = composable component "patch"; `fn foo() -> impl Scene { bsn! { … } }` is a reusable scene function, embedded with `@foo()`.

## Common pattern in this repo

```rust
cmds.spawn_scene(bsn! { … }).with_children(|p| {
    p.spawn_empty().queue_apply_scene(bsn! { … });
});
```

## Related 0.20 UI changes

- **Buttons**: `bevy::ui::widget::Button` (prelude) and `bevy::ui::Interaction` are deprecated → use `bevy::ui_widgets::Button` plus `Hovered` (`bevy::picking::hover`) / `Pressed` (`bevy::ui`) components (see Bevy's `button.rs` example).
- **Pointer events are flat**: `On<Pointer<Press>>` → `On<PointerPress>`; fields via `event.pointer.id` / `event.pointer.position` (a `Vec2`).
- **`BorderRadius` fields are `CornerRadius`** (2D): `top_left: px(10.).into()` or `CornerRadius::circular(px(10.))`; constructors take `impl Into<CornerRadius>` and are no longer `const`.
- **`Val::Em` / `Val::Rem`** (`em()`, `rem()` helpers) size relative to font size; `Node` now requires `EmSize`.

## Reusable button helper

`src/menu/tools/menu_btn.rs` provides `menu_btn::scene(MenuAction, ButtonStyle, assets, margin) -> impl Scene` and `menu_btn::system` (writes `MenuAction` messages on press). Prefer this over hand-rolled buttons. Note: it still uses the deprecated `ui::Button` + `Interaction`; migrate it to `ui_widgets::Button` + `Hovered`/`Pressed` when touching it.

## Scrolling & scrollbars

All in `bevy::ui_widgets` (re-exported by `bevy`; `bevy_ui_widgets` is a default dep). `ScrollAreaPlugin` + `ScrollbarPlugin` are already registered via `DefaultPlugins` → `UiWidgetsPlugins` — no manual plugin wiring.

- **Scrollable container**: add `overflow: Overflow::scroll_y()` (or `scroll()`, `scroll_x()`) to the `Node`, plus the `ScrollArea` component. `ScrollArea` has `#[require(ScrollPosition)]`, so `ScrollPosition` is auto-inserted — do NOT add it manually. The container needs a bounded height (e.g. `top`/`bottom: 0` on an absolute node) or it won't overflow.
- **`Overflow` constructors** (`bevy::ui::Overflow`): `visible()`, `clip()`, `clip_x()`, `clip_y()`, `hidden()`, `hidden_x()`, `hidden_y()`, `scroll()`, `scroll_x()`, `scroll_y()`.
- **Scrollbar widget** (`bevy::ui_widgets::{Scrollbar, ScrollbarThumb, ControlOrientation}`): `Scrollbar { target: Entity, orientation, min_thumb_length }` controls the scroll area's `ScrollPosition` directly (drag + track-click page up/down, via observers). `ScrollbarThumb` is the moving child — it has **no `Node` component**; only `border_radius`/`border` styling, size/position set by `update_scrollbar_thumb`. `Scrollbar` needs the scroll-area entity id, so capture it: `let id = cmds.spawn_scene(bsn!{…}).id();`. Both widgets are bsn-able; set `target: {id}` via `{expr}`.
- **Z-order**: scrollbars must sit above the content to receive pointer events (drag). Add `GlobalZIndex(1)` to the scrollbar node; otherwise buttons underneath swallow the drag. `GlobalZIndex` also reorders rendering + picking together.
- **Show-only-when-scrollable**: compare `ComputedNode::content_size().y > size().y` on the target; toggle `Visibility` accordingly.

See `src/menu/tools/scrollbar.rs` for the working helper (`spawn(cmds, target)` + `update_visibility`).
