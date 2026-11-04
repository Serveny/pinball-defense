use crate::game::enemy::EnemyKind;
use crate::game::health::Health;
use crate::game::level::{Level, LevelUpEvent};
use crate::game::player_life::LifeBar;
use crate::game::tower::{TowerUpgradedEvent, level_numeral};
use crate::game::wave::WaveStartedEvent;
use crate::prelude::*;
use bevy::color::Hsva;
use bevy::text::{FontSize, FontSourceTemplate};
use bevy::ui::{BackgroundGradient, BoxShadow, ColorStop, Gradient, LinearGradient};

const BANNER_SECS: f32 = 2.;
const LEVEL_UP_EXTRA_SECS: f32 = 3.;
const SLIDE_DIST_PX: f32 = 24.;
const SLIDE_IN_SECS: f32 = 0.25;
const FADE_OUT_FRACTION: f32 = 0.8;
const TOP_MARGIN_PX: f32 = 16.;
const MAX_TOP_PX: f32 = 220.;
const GAP_PX: f32 = 8.;
const BASE_HIT_THRESHOLD: f32 = 0.3;

#[derive(Component)]
pub(super) struct EventBanner {
    timer: Timer,
}

impl EventBanner {
    fn new(kind: BannerType) -> Self {
        let secs = match kind {
            BannerType::LevelUp { .. } => BANNER_SECS + LEVEL_UP_EXTRA_SECS,
            _ => BANNER_SECS,
        };
        Self {
            timer: Timer::from_seconds(secs, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub(super) struct BannerLine;

#[derive(Component, Clone, Copy)]
pub(super) struct BannerKind(BannerType);

#[derive(Clone, Copy, PartialEq)]
pub(super) enum BannerType {
    LevelUp { level: Level },
    Wave,
    SpecialWave(EnemyKind),
    Upgraded(u8),
    BaseHit,
    Extra(crate::game::extra::ExtraFieldKind),
}

impl BannerType {
    fn title(self, wave: usize) -> String {
        match self {
            BannerType::LevelUp { .. } => "LEVEL UP".into(),
            BannerType::Wave | BannerType::SpecialWave(EnemyKind::Normal) => format!("WAVE {wave}"),
            BannerType::SpecialWave(_) => "SPECIAL WAVE".into(),
            BannerType::Upgraded(level) => format!("TOWER LEVEL {}", level_numeral(level)),
            BannerType::BaseHit => "BASE UNDER ATTACK".into(),
            BannerType::Extra(kind) => kind.label().into(),
        }
    }

    fn subtitle(self) -> Option<String> {
        match self {
            BannerType::LevelUp { level } => Some(level_up_unlocks(level)),
            BannerType::SpecialWave(EnemyKind::Tank) => Some("ARMORED TANKS INCOMING".into()),
            BannerType::SpecialWave(EnemyKind::Speeder) => Some("SPEEDERS INCOMING".into()),
            BannerType::Extra(kind) => {
                Some(format!("EXTRA TRIGGERED\n{}", extra_effect_text(kind)))
            }
            _ => None,
        }
    }

    fn hue(self) -> f32 {
        match self {
            BannerType::LevelUp { .. } => 45.,
            BannerType::Wave | BannerType::SpecialWave(EnemyKind::Normal) => 35.,
            BannerType::SpecialWave(EnemyKind::Tank) => 15.,
            BannerType::SpecialWave(EnemyKind::Speeder) => 285.,
            BannerType::Upgraded(_) | BannerType::BaseHit => 0.,
            BannerType::Extra(kind) => extra_hue(kind),
        }
    }
}

fn extra_hue(kind: crate::game::extra::ExtraFieldKind) -> f32 {
    match kind {
        crate::game::extra::ExtraFieldKind::ExtraBall => 50.,
        crate::game::extra::ExtraFieldKind::SlowDown => 210.,
        crate::game::extra::ExtraFieldKind::DoubleUpgradePoints => 30.,
        crate::game::extra::ExtraFieldKind::InstaKill => 0.,
    }
}

fn extra_effect_text(kind: crate::game::extra::ExtraFieldKind) -> String {
    match kind {
        crate::game::extra::ExtraFieldKind::ExtraBall => "BONUS BALL SPAWNED".into(),
        crate::game::extra::ExtraFieldKind::SlowDown => "ENEMIES SLOWED FOR 5s".into(),
        crate::game::extra::ExtraFieldKind::DoubleUpgradePoints => {
            "DOUBLE UPGRADE POINTS FOR 5s".into()
        }
        crate::game::extra::ExtraFieldKind::InstaKill => "INSTA-KILL FOR 5s".into(),
    }
}

fn hue_shifted(hue: f32, alpha: f32) -> Color {
    Color::from(Hsva::new(hue.rem_euclid(360.), 0.85, 1., alpha))
}

fn level_up_unlocks(level: Level) -> String {
    let mut unlocks: Vec<String> = Vec::new();
    if crate::game::tower::foundation::spawns_on_level_up(level) {
        unlocks.push("NEW TOWER FOUNDATION".into());
    }
    if level == 2 {
        unlocks.push("TESLA UNLOCK FIELD AVAILABLE".into());
    } else if level == 3 {
        unlocks.push("MICROWAVE UNLOCK FIELD AVAILABLE".into());
    }
    format!("\nUnlocks\n{}", unlocks.join("\n"))
}

fn extra_field_unlock_line(kind: crate::game::extra::ExtraFieldKind) -> String {
    format!("EXTRA FIELD: {}", kind.label())
}

fn hue_gradient_line(hue: f32, alpha: f32) -> BackgroundGradient {
    BackgroundGradient(vec![Gradient::Linear(LinearGradient::to_right(vec![
        ColorStop::auto(Color::NONE),
        ColorStop::auto(hue_shifted(hue, alpha)),
        ColorStop::auto(hue_shifted(hue + 40., alpha)),
        ColorStop::auto(Color::NONE),
    ]))])
}

fn banner_background(hue: f32, alpha: f32) -> BackgroundGradient {
    BackgroundGradient(vec![Gradient::Linear(LinearGradient::to_right(vec![
        ColorStop::auto(Color::NONE),
        ColorStop::auto(Hsva::new(hue, 0.8, 0.4, 0.07 * alpha)),
        ColorStop::auto(Hsva::new(hue + 30., 0.8, 0.5, 0.13 * alpha)),
        ColorStop::auto(Color::NONE),
    ]))])
}

fn banner_text(text: String, size: f32, color: Color, font: Handle<Font>) -> impl Scene {
    let font = FontSourceTemplate::Handle(font.into());
    bsn! {
        Text({text})
        TextLayout::justify(Justify::Center)
        TextFont { font: {font}, font_size: FontSize::Px({size}) }
        TextColor({color})
        TextShadow
    }
}

fn spawn_banner(cmds: &mut Commands, kind: BannerType, wave: usize, assets: &PinballDefenseAssets) {
    let hue = kind.hue();
    let font = assets.menu_font.clone();
    let title = banner_text(kind.title(wave), 36., hue_shifted(hue, 1.), font.clone());
    let subtitle = kind
        .subtitle()
        .map(|sub| banner_text(sub, 16., Color::from(Hsva::new(hue, 0.3, 1., 1.)), font));
    cmds.spawn_scene(bsn! {
        Name::new("Event Banner")
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(TOP_MARGIN_PX),
            left: Val::Percent(30.),
            width: Val::Percent(40.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(2.),
            border: UiRect::all(Val::Px(1.)),
            border_radius: BorderRadius::all(Val::Px(8.)),
            padding: UiRect::new(Val::Px(6.), Val::Px(18.), Val::Px(8.), Val::Px(8.)),
        }
        GlobalZIndex(5)
        Children [ @title ]
    })
    .insert((
        EventBanner::new(kind),
        BannerKind(kind),
        banner_background(hue, 1.),
        Pickable::IGNORE,
        BoxShadow::new(
            hue_shifted(hue, 0.35),
            Val::ZERO,
            Val::ZERO,
            Val::Px(10.),
            Val::Px(18.),
        ),
    ))
    .with_children(|p| {
        if let Some(subtitle) = subtitle {
            p.spawn_empty().apply_scene(subtitle);
        }
        p.spawn((
            BannerLine,
            Node {
                width: Val::Percent(70.),
                height: Val::Px(2.),
                ..default()
            },
            hue_gradient_line(hue, 1.),
            Pickable::IGNORE,
        ));
    });
}

pub(super) fn on_level_up_system(
    mut evr: MessageReader<LevelUpEvent>,
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
) {
    for ev in evr.read() {
        if ev.0 < 2 {
            continue;
        }
        spawn_banner(&mut cmds, BannerType::LevelUp { level: ev.0 }, 0, &assets);
    }
}

pub(super) fn on_extra_field_unlock_system(
    mut evr: MessageReader<crate::game::extra::ExtraFieldUnlockEvent>,
    q_banner: Query<(&BannerKind, &Children)>,
    mut q_text: Query<&mut Text>,
) {
    for crate::game::extra::ExtraFieldUnlockEvent(kind) in evr.read() {
        let line = extra_field_unlock_line(*kind);
        for (banner_kind, children) in &q_banner {
            let BannerType::LevelUp { .. } = banner_kind.0 else {
                continue;
            };
            for &child in children {
                let Ok(mut text) = q_text.get_mut(child) else {
                    continue;
                };
                if !text.0.contains("Unlocks") || text.0.contains(&line) {
                    continue;
                }
                if !text.0.ends_with('\n') {
                    text.0.push('\n');
                }
                text.0.push_str(&line);
            }
        }
    }
}

pub(super) fn on_wave_started_system(
    mut evr: MessageReader<WaveStartedEvent>,
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
) {
    for ev in evr.read() {
        let kind = match ev.kind {
            EnemyKind::Normal => BannerType::Wave,
            other => BannerType::SpecialWave(other),
        };
        spawn_banner(&mut cmds, kind, ev.number, &assets);
    }
}

pub(super) fn on_tower_upgraded_system(
    mut evr: MessageReader<TowerUpgradedEvent>,
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
) {
    for TowerUpgradedEvent(_, level) in evr.read() {
        spawn_banner(&mut cmds, BannerType::Upgraded(*level), 0, &assets);
    }
}

pub(super) fn on_extra_fire_system(
    mut evr: MessageReader<crate::game::extra::ExtraFieldFireEvent>,
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
) {
    for crate::game::extra::ExtraFieldFireEvent(kind) in evr.read() {
        spawn_banner(&mut cmds, BannerType::Extra(*kind), 0, &assets);
    }
}

pub(super) fn on_base_hit_system(
    q_life_bar: Query<&Health, (With<LifeBar>, Changed<Health>)>,
    mut warned: Local<bool>,
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
    q_banner: Query<(), With<EventBanner>>,
) {
    for health in q_life_bar.iter() {
        let fraction = health.fraction();
        if fraction > BASE_HIT_THRESHOLD + 0.05 {
            *warned = false;
        } else if fraction <= BASE_HIT_THRESHOLD && !*warned && q_banner.is_empty() {
            *warned = true;
            spawn_banner(&mut cmds, BannerType::BaseHit, 0, &assets);
        }
    }
}

type QBanner<'w, 's, 'a> = Query<
    'w,
    's,
    (
        Entity,
        &'a mut EventBanner,
        &'a mut Node,
        &'a mut UiTransform,
        &'a ComputedNode,
        &'a BannerKind,
        &'a mut BoxShadow,
    ),
>;

pub(super) fn banner_update_system(
    mut cmds: Commands,
    time: Res<Time>,
    mut q_banner: QBanner,
    mut q_root_gradient: Query<&mut BackgroundGradient, (With<EventBanner>, Without<BannerLine>)>,
    mut q_line_gradient: Query<(&ChildOf, &mut BackgroundGradient), With<BannerLine>>,
    mut q_text: Query<(&ChildOf, &mut TextColor)>,
) {
    let mut order: Vec<(f32, Entity, f32, f32)> = Vec::new();
    for (id, mut banner, _node, mut transform, computed, kind, mut glow) in q_banner.iter_mut() {
        banner.timer.tick(time.delta());
        if banner.timer.is_finished() {
            cmds.entity(id).despawn();
            continue;
        }
        let t = banner.timer.elapsed_secs();
        let fraction = banner.timer.fraction();

        let (alpha, scale, slide) = if t < SLIDE_IN_SECS {
            let k = t / SLIDE_IN_SECS;
            (k, 1.15 - 0.15 * k, (1. - k) * SLIDE_DIST_PX)
        } else if fraction > FADE_OUT_FRACTION {
            let k = (fraction - FADE_OUT_FRACTION) / (1. - FADE_OUT_FRACTION);
            (1. - k, 1., k * SLIDE_DIST_PX)
        } else {
            (1., 1., 0.)
        };

        let hue = kind.0.hue() + t * 60.;
        order.push((t, id, computed.unrounded_size.y, slide));
        if let Ok(mut gradient) = q_root_gradient.get_mut(id) {
            *gradient = banner_background(hue, alpha);
        }
        if let Some(glow_style) = glow.0.first_mut() {
            glow_style.color = hue_shifted(hue, 0.5 * alpha);
        }

        for (parent, mut color) in q_text.iter_mut() {
            if parent.0 == id {
                color.0 = hue_shifted(hue, alpha);
            }
        }
        for (parent, mut gradient) in q_line_gradient.iter_mut() {
            if parent.0 == id {
                *gradient = hue_gradient_line(hue, alpha);
            }
        }
        transform.scale = Vec2::splat(scale);
    }

    order.sort_by(|a, b| b.0.total_cmp(&a.0));
    let mut next_top = TOP_MARGIN_PX;
    for (_, id, height, slide) in order {
        let top = (next_top - slide).min(MAX_TOP_PX);
        next_top = top + slide + height + GAP_PX;
        if let Ok((_, _, mut node, _, _, _, _)) = q_banner.get_mut(id) {
            node.top = Val::Px(top);
        }
    }
}
