use super::project_3d_to_2d_screen;
use crate::game::camera::PinballCamera;
use crate::game::level::PointsEvent;
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::text::{FontSize, FontSource};

const FLOAT_DURATION_SECS: f32 = 1.2;
const RISE_PX: f32 = 40.;

#[derive(Component)]
pub struct FloatingPoints {
    world_pos: Vec3,
    timer: Timer,
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
        node.left = Val::Px(screen.x);
        node.top = Val::Px(screen.y - t * RISE_PX);
        color.0 = color.0.with_alpha(1. - t);
        if fp.timer.just_finished() {
            cmds.entity(id).despawn();
        }
    }
}
