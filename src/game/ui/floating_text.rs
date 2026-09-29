use super::project_3d_to_2d_screen;
use crate::game::camera::PinballCamera;
use crate::game::level::PointsEvent;
use crate::game::tower::TowerType;
use crate::game::tower::{Tower, TowerUpgradedEvent, level_numeral};
use crate::game::unlock::TowerUnlockedEvent;
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::text::{FontSize, FontSource};

const FLOAT_DURATION_SECS: f32 = 1.2;
const RISE_PX: f32 = 40.;

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
    mut unlocks: MessageReader<TowerUnlockedEvent>,
    mut upgrades: MessageReader<TowerUpgradedEvent>,
    towers: Query<&GlobalTransform, With<Tower>>,
    assets: Res<PinballDefenseAssets>,
) {
    for ev in evr.read() {
        spawn_text(
            &mut cmds,
            &assets,
            ev.pos,
            format!("+{}", ev.points),
            GameColor::GOLD,
            FLOAT_DURATION_SECS,
        );
    }
    for TowerUnlockedEvent(kind, pos) in unlocks.read() {
        let (label, color) = match kind {
            TowerType::Tesla => ("TESLA TOWER UNLOCKED", Color::srgb_u8(35, 190, 245)),
            TowerType::Microwave => ("MICROWAVE TOWER UNLOCKED", Color::srgb_u8(245, 110, 90)),
            TowerType::Gun => continue,
        };
        spawn_text(&mut cmds, &assets, *pos, label.into(), color, 2.);
    }
    for TowerUpgradedEvent(tower, level) in upgrades.read() {
        let Ok(transform) = towers.get(*tower) else {
            continue;
        };
        cmds.spawn((
            Name::new("Tower Level Hint"),
            FloatingPoints {
                world_pos: transform.translation(),
                timer: Timer::from_seconds(FLOAT_DURATION_SECS * 2., TimerMode::Once),
                offset: Vec2::new(-48., -56.),
            },
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(96.),
                ..default()
            },
            Text(level_numeral(*level).into()),
            TextLayout::justify(Justify::Center),
            TextFont {
                font: FontSource::Handle(assets.menu_font.clone()),
                font_size: FontSize::Px(27.),
                ..default()
            },
            TextColor(Color::WHITE),
            TextShadow::default(),
            GlobalZIndex(2),
            Pickable::IGNORE,
        ));
    }
}

fn spawn_text(
    cmds: &mut Commands,
    assets: &PinballDefenseAssets,
    pos: Vec3,
    text: String,
    color: Color,
    duration: f32,
) {
    cmds.spawn((
        Name::new("Floating Text"),
        FloatingPoints {
            world_pos: pos,
            timer: Timer::from_seconds(duration, TimerMode::Once),
            offset: Vec2::ZERO,
        },
        Node {
            position_type: PositionType::Absolute,
            ..default()
        },
        Text(text),
        TextFont {
            font: FontSource::Handle(assets.menu_font.clone()),
            font_size: FontSize::Px(32.),
            ..default()
        },
        TextColor(color),
    ));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tower_levels_float_only_on_upgrade_at_half_the_points_speed() {
        let mut app = App::new();
        app.add_message::<PointsEvent>()
            .add_message::<TowerUnlockedEvent>()
            .add_message::<TowerUpgradedEvent>()
            .init_resource::<Time>()
            .insert_resource(PinballDefenseAssets::default())
            .add_systems(Update, (spawn_system, update_system).chain());
        app.world_mut()
            .spawn((PinballCamera, Camera::default(), GlobalTransform::default()));
        let tower = app
            .world_mut()
            .spawn((Tower::new(Vec3::ZERO), GlobalTransform::default()))
            .id();
        app.update();
        let mut labels = app.world_mut().query::<(
            &FloatingPoints,
            &Text,
            &TextFont,
            &Node,
            &TextColor,
            Option<&BackgroundColor>,
        )>();
        assert_eq!(labels.iter(app.world()).count(), 0);
        for level in 1..=crate::game::tower::MAX_TOWER_LEVEL {
            app.world_mut()
                .write_message(TowerUpgradedEvent(tower, level));
        }
        app.update();
        let numerals = ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"];
        assert_eq!(labels.iter(app.world()).count(), numerals.len());
        for numeral in numerals {
            let (floating, _, font, node, color, background) = labels
                .iter(app.world())
                .find(|(_, text, _, _, _, _)| text.0 == numeral)
                .unwrap();
            assert_eq!(
                floating.timer.duration(),
                std::time::Duration::from_secs_f32(2.4)
            );
            assert_eq!(font.font_size, FontSize::Px(27.));
            assert_eq!(node.width, Val::Px(96.));
            assert_eq!(floating.offset, Vec2::new(-48., -56.));
            assert_eq!(color.0, Color::WHITE);
            assert!(background.is_none_or(|bg| bg.0.alpha() < 0.0001));
            assert_eq!(node.border, UiRect::default());
        }
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(std::time::Duration::from_secs_f32(0.6));
        app.update();
        for (floating, _, _, node, color, _) in labels.iter(app.world()) {
            assert!((floating.timer.fraction() - 0.25).abs() < 0.0001);
            assert_eq!(node.top, Val::Px(-66.));
            assert!((color.0.alpha() - 0.75).abs() < 0.0001);
        }
        for _ in 0..3 {
            app.update();
        }
        assert_eq!(labels.iter(app.world()).count(), 0);
        app.update();
        assert_eq!(labels.iter(app.world()).count(), 0);
    }

    #[test]
    fn tower_unlock_floats_for_two_seconds() {
        let mut app = App::new();
        app.add_message::<PointsEvent>()
            .add_message::<TowerUnlockedEvent>()
            .add_message::<TowerUpgradedEvent>()
            .insert_resource(PinballDefenseAssets::default())
            .add_systems(Update, spawn_system);
        let pos = Vec3::new(0.45, -0.18, -0.048);
        app.world_mut()
            .write_message(TowerUnlockedEvent(TowerType::Tesla, pos));
        app.update();
        let mut texts = app.world_mut().query::<(&FloatingPoints, &Text)>();
        let (floating, text) = texts.single(app.world()).unwrap();
        assert_eq!(floating.world_pos, pos);
        assert_eq!(floating.timer.duration(), std::time::Duration::from_secs(2));
        assert_eq!(text.0, "TESLA TOWER UNLOCKED");
    }
}
