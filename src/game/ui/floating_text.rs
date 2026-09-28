use super::project_3d_to_2d_screen;
use crate::game::camera::PinballCamera;
use crate::game::level::PointsEvent;
use crate::game::tower::{Tower, TowerUpgradedEvent, level_color};
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::text::{FontSize, FontSource};

const FLOAT_DURATION_SECS: f32 = 1.2;
const RISE_PX: f32 = 40.;
const LEVEL_HINT_SECS: f32 = 1.5;

#[derive(Component)]
pub struct TowerLevelHint {
    tower: Entity,
    timer: Timer,
}

pub(super) fn spawn_level_hint_system(
    mut cmds: Commands,
    mut events: MessageReader<TowerUpgradedEvent>,
    assets: Res<PinballDefenseAssets>,
) {
    for TowerUpgradedEvent(tower, level) in events.read() {
        let numeral = match level {
            2 => "II",
            3 => "III",
            4 => "IV",
            _ => "V",
        };
        cmds.spawn((
            Name::new("Tower Level Hint"),
            TowerLevelHint {
                tower: *tower,
                timer: Timer::from_seconds(LEVEL_HINT_SECS, TimerMode::Once),
            },
            Node {
                position_type: PositionType::Absolute,
                ..default()
            },
            Text(numeral.into()),
            TextFont {
                font: FontSource::Handle(assets.menu_font.clone()),
                font_size: FontSize::Px(32.),
                ..default()
            },
            TextColor(level_color(*level)),
        ));
    }
}

pub(super) fn update_level_hint_system(
    mut cmds: Commands,
    time: Res<Time>,
    mut hints: Query<(Entity, &mut Node, &mut TextColor, &mut TowerLevelHint)>,
    towers: Query<&GlobalTransform, With<Tower>>,
    q_cam: Query<(&GlobalTransform, &Camera), With<PinballCamera>>,
) {
    let Ok((cam_trans, cam)) = q_cam.single() else {
        return;
    };
    for (id, mut node, mut color, mut hint) in &mut hints {
        let Ok(transform) = towers.get(hint.tower) else {
            cmds.entity(id).despawn();
            continue;
        };
        hint.timer.tick(time.delta());
        let screen = project_3d_to_2d_screen(transform.translation(), cam_trans, cam);
        node.left = Val::Px(screen.x);
        node.top = Val::Px(screen.y - 48. - hint.timer.fraction() * 20.);
        color.0 = color.0.with_alpha(1. - hint.timer.fraction());
        if hint.timer.is_finished() {
            cmds.entity(id).despawn();
        }
    }
}

#[derive(Component)]
pub struct FloatingPoints {
    world_pos: Vec3,
    timer: Timer,
    offset: Vec2,
}

pub(crate) fn spawn_tower_build(
    cmds: &mut Commands,
    pos: Vec3,
    tower_type: crate::game::tower::TowerType,
    assets: &PinballDefenseAssets,
) {
    let name = match tower_type {
        crate::game::tower::TowerType::Gun => "Gun",
        crate::game::tower::TowerType::Tesla => "Tesla",
        crate::game::tower::TowerType::Microwave => "Microwave",
    };
    cmds.spawn((
        Name::new("Tower Build Label"),
        FloatingPoints {
            world_pos: pos,
            timer: Timer::from_seconds(2., TimerMode::Once),
            offset: Vec2::new(-110., -48.),
        },
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(220.),
            ..default()
        },
        Text(format!("New {name} Tower")),
        TextLayout::justify(Justify::Center),
        TextFont {
            font: FontSource::Handle(assets.menu_font.clone()),
            font_size: FontSize::Px(22.),
            ..default()
        },
        TextColor(Color::WHITE),
        TextShadow::default(),
        Pickable::IGNORE,
    ));
}

pub(super) fn spawn_system(
    mut cmds: Commands,
    mut evr: MessageReader<PointsEvent>,
    assets: Res<PinballDefenseAssets>,
) {
    for ev in evr.read() {
        let font = FontSource::Handle(assets.menu_font.clone());
        cmds.spawn((
            Name::new("Floating Points"),
            FloatingPoints {
                world_pos: ev.pos,
                timer: Timer::from_seconds(FLOAT_DURATION_SECS, TimerMode::Once),
                offset: Vec2::ZERO,
            },
            Node {
                position_type: PositionType::Absolute,
                ..default()
            },
            Text(format!("+{}", ev.points)),
            TextFont {
                font,
                font_size: FontSize::Px(32.0),
                ..default()
            },
            TextColor(GameColor::GOLD),
        ));
    }
}

pub(super) fn update_system(
    mut cmds: Commands,
    time: Res<Time>,
    mut q: Query<(Entity, &mut Node, &mut TextColor, &mut FloatingPoints)>,
    q_cam: Query<(&GlobalTransform, &Camera), With<PinballCamera>>,
) {
    let Ok((cam_trans, cam)) = q_cam.single() else {
        return;
    };
    for (id, mut node, mut color, mut fp) in q.iter_mut() {
        fp.timer.tick(time.delta());
        let t = fp.timer.fraction();
        let screen = project_3d_to_2d_screen(fp.world_pos, cam_trans, cam);
        node.left = Val::Px(screen.x + fp.offset.x);
        node.top = Val::Px(screen.y + fp.offset.y - t * RISE_PX);
        color.0 = color.0.with_alpha(1. - t);
        if fp.timer.just_finished() {
            cmds.entity(id).despawn();
        }
    }
}
