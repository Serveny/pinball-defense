use crate::game::enemy::EnemyKind;
use crate::game::health::Health;
use crate::game::level::{Level, LevelUpEvent};
use crate::game::pinball_menu::{PinballMenu, UpgradeMenuExecuteEvent};
use crate::game::player_life::LifeBar;
use crate::game::tower::{TowerType, TowerUpgrade};
use crate::game::wave::WaveStartedEvent;
use crate::prelude::*;
use bevy::color::Hsva;
use bevy::ui::{BackgroundGradient, BoxShadow, ColorStop, Gradient, LinearGradient};

const BANNER_SECS: f32 = 2.;
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
    slot: u32,
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
    UpgradeReady,
    Upgraded(TowerUpgrade),
    BaseHit,
}

impl BannerType {
    fn title(self, wave: usize) -> String {
        match self {
            BannerType::LevelUp { .. } => "LEVEL UP".into(),
            BannerType::Wave | BannerType::SpecialWave(EnemyKind::Normal) => format!("WAVE {wave}"),
            BannerType::SpecialWave(_) => "SPECIAL WAVE".into(),
            BannerType::UpgradeReady => "UPGRADE READY".into(),
            BannerType::Upgraded(_) => "TOWER UPGRADED".into(),
            BannerType::BaseHit => "BASE UNDER ATTACK".into(),
        }
    }

    fn subtitle(self) -> Option<String> {
        match self {
            BannerType::LevelUp { level } => Some(level_up_unlocks(level)),
            BannerType::SpecialWave(EnemyKind::Tank) => Some("ARMORED TANKS INCOMING".into()),
            BannerType::SpecialWave(EnemyKind::Speeder) => Some("SPEEDERS INCOMING".into()),
            BannerType::Upgraded(TowerUpgrade::Damage) => Some("DAMAGE".into()),
            BannerType::Upgraded(TowerUpgrade::Range) => Some("RANGE".into()),
            BannerType::UpgradeReady => Some("HIT THE UPGRADE CARD WITH THE BALL".into()),
            _ => None,
        }
    }

    fn hue(self) -> f32 {
        match self {
            BannerType::LevelUp { .. } => 45.,
            BannerType::Wave | BannerType::SpecialWave(EnemyKind::Normal) => 35.,
            BannerType::SpecialWave(EnemyKind::Tank) => 15.,
            BannerType::SpecialWave(EnemyKind::Speeder) => 285.,
            BannerType::UpgradeReady => 140.,
            BannerType::Upgraded(TowerUpgrade::Damage) | BannerType::BaseHit => 0.,
            BannerType::Upgraded(TowerUpgrade::Range) => 195.,
        }
    }
}

fn hue_shifted(hue: f32, alpha: f32) -> Color {
    Color::from(Hsva::new(hue.rem_euclid(360.), 0.85, 1., alpha))
}

fn level_up_unlocks(level: Level) -> String {
    let mut unlocks: Vec<String> = vec!["NEW TOWER FOUNDATION".into()];
    match crate::game::pinball_menu::new_tower_unlock(level) {
        Some(TowerType::Tesla) => unlocks.push("TESLA TOWER".into()),
        Some(TowerType::Microwave) => unlocks.push("MICROWAVE TOWER".into()),
        Some(TowerType::Gun) | None => {}
    }
    match crate::game::pinball_menu::new_tower_upgrade_unlock(level) {
        Some(TowerUpgrade::Range) => unlocks.push("TOWER RANGE UPGRADE".into()),
        Some(TowerUpgrade::Damage) => unlocks.push("TOWER DAMAGE UPGRADE".into()),
        None => {}
    }
    format!("\nUnlocks\n{}", unlocks.join("\n"))
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
        ColorStop::auto(Hsva::new(hue, 0.8, 0.4, 0.14 * alpha)),
        ColorStop::auto(Hsva::new(hue + 30., 0.8, 0.5, 0.26 * alpha)),
        ColorStop::auto(Color::NONE),
    ]))])
}

fn spawn_banner(
    cmds: &mut Commands,
    kind: BannerType,
    wave: usize,
    slot: u32,
    assets: &PinballDefenseAssets,
) {
    let hue = kind.hue();
    cmds.spawn((
        Name::new("Event Banner"),
        EventBanner {
            timer: Timer::from_seconds(BANNER_SECS, TimerMode::Once),
            slot,
        },
        BannerKind(kind),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(TOP_MARGIN_PX),
            left: Val::Percent(30.),
            width: Val::Percent(40.),
            height: Val::Auto,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(2.),
            border: UiRect::all(Val::Px(1.)),
            border_radius: BorderRadius::all(Val::Px(8.)),
            padding: UiRect::new(Val::Px(6.), Val::Px(18.), Val::Px(8.), Val::Px(8.)),
            ..default()
        },
        banner_background(hue, 1.),
        UiTransform::from_scale(Vec2::splat(1.)),
        Pickable::IGNORE,
        BoxShadow::new(
            hue_shifted(hue, 0.35),
            Val::ZERO,
            Val::ZERO,
            Val::Px(10.),
            Val::Px(18.),
        ),
        GlobalZIndex(5),
    ))
    .with_children(|p| {
        p.spawn((
            Text(kind.title(wave)),
            TextLayout::justify(Justify::Center),
            TextFont {
                font: assets.menu_font.clone().into(),
                font_size: FontSize::Px(36.),
                ..default()
            },
            TextColor(hue_shifted(hue, 1.)),
            TextShadow::default(),
        ));
        if let Some(sub) = kind.subtitle() {
            p.spawn((
                Text(sub),
                TextLayout::justify(Justify::Center),
                TextFont {
                    font: assets.menu_font.clone().into(),
                    font_size: FontSize::Px(16.),
                    ..default()
                },
                TextColor(Color::from(Hsva::new(hue, 0.3, 1., 1.))),
                TextShadow::default(),
            ));
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
    q_banner: Query<(), With<EventBanner>>,
) {
    for ev in evr.read() {
        if ev.0 < 2 {
            continue;
        }
        let slot = u32::try_from(q_banner.iter().count()).unwrap_or(0);
        spawn_banner(
            &mut cmds,
            BannerType::LevelUp { level: ev.0 },
            0,
            slot,
            &assets,
        );
    }
}

pub(super) fn on_wave_started_system(
    mut evr: MessageReader<WaveStartedEvent>,
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
    q_banner: Query<(), With<EventBanner>>,
) {
    for ev in evr.read() {
        let kind = match ev.kind {
            EnemyKind::Normal => BannerType::Wave,
            other => BannerType::SpecialWave(other),
        };
        let slot = u32::try_from(q_banner.iter().count()).unwrap_or(0);
        spawn_banner(&mut cmds, kind, ev.number, slot, &assets);
    }
}

pub(super) fn on_upgrade_ready_system(
    q_menu: Query<(Entity, &PinballMenu), Added<PinballMenu>>,
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
    q_banner: Query<(), With<EventBanner>>,
) {
    if let Some((_, PinballMenu::Upgrade)) = q_menu.iter().next() {
        let slot = u32::try_from(q_banner.iter().count()).unwrap_or(0);
        spawn_banner(&mut cmds, BannerType::UpgradeReady, 0, slot, &assets);
    }
}

pub(super) fn on_tower_upgraded_system(
    mut evr: MessageReader<UpgradeMenuExecuteEvent>,
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
    q_banner: Query<(), With<EventBanner>>,
) {
    for ev in evr.read() {
        let slot = u32::try_from(q_banner.iter().count()).unwrap_or(0);
        spawn_banner(&mut cmds, BannerType::Upgraded(ev.upgrade), 0, slot, &assets);
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
            spawn_banner(&mut cmds, BannerType::BaseHit, 0, 0, &assets);
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
    mut q_root_gradient: Query<
        &mut BackgroundGradient,
        (With<EventBanner>, Without<BannerLine>),
    >,
    mut q_line_gradient: Query<(&ChildOf, &mut BackgroundGradient), With<BannerLine>>,
    mut q_text: Query<(&ChildOf, &mut TextColor)>,
) {
    let mut order: Vec<(u32, Entity, f32, f32, f32)> = Vec::new();
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
        order.push((banner.slot, id, computed.unrounded_size.y, slide, alpha));
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

    order.sort_by_key(|(slot, ..)| *slot);
    let mut next_top = TOP_MARGIN_PX;
    for (_slot, id, height, slide, _alpha) in order {
        let top = (next_top - slide).min(MAX_TOP_PX);
        next_top = top + slide + height + GAP_PX;
        if let Ok((_, _, mut node, _, _, _, _)) = q_banner.get_mut(id) {
            node.top = Val::Px(top);
        }
    }
}