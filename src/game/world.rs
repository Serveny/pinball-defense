use super::audio::SoundEvent;
use super::ball::CollisionWithBallEvent;
use super::ball_starter::BallStarterPlugin;
use super::events::collision::COLLIDE_ONLY_WITH_BALL;
use super::flipper::FlipperPlugin;
use super::lamp::spawn_lamp;
use super::level::{LevelCounterId, PointCounterId};
use super::light::{spawn_level_up_lights, LevelUpLamp};
use super::pinball_menu::pinball_menu_glass;
use super::player_life::spawn_life_bar;
use super::road::spawn_road;
use super::tower::foundation;
use super::{analog_counter, EventState, GameState};
use crate::assets::PinballDefenseGltfAssets;
use crate::generated::world_1::*;
use crate::prelude::*;
use crate::settings::GraphicsSettings;

pub type QueryWorld<'w, 's> = Query<'w, 's, Entity, With<PinballWorld>>;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FlipperPlugin)
            .add_plugins(BallStarterPlugin)
            .add_systems(OnEnter(GameState::Init), spawn_pinball_world)
            .add_systems(
                Update,
                on_ball_coll_wall_system.run_if(in_state(EventState::Active)),
            );
    }
}

#[derive(Component)]
pub struct PinballWorld;

#[derive(Component)]
struct WorldFrame;

fn spawn_pinball_world(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut pc_id: ResMut<PointCounterId>,
    mut lc_id: ResMut<LevelCounterId>,
    assets: Res<PinballDefenseGltfAssets>,
    g_sett: Res<GraphicsSettings>,
) {
    let assets = assets.as_ref();
    //let mut img_handle: Option<Handle<Image>> = None;
    cmds.spawn((
        SpatialBundle { ..default() },
        PinballWorld,
        Name::new("Pinball World"),
    ))
    .with_children(|p| {
        // World mesh
        p.spawn((PbrBundle {
            mesh: assets.world_1.clone(),
            material: assets.world_1_material.clone(),
            ..default()
        },));

        // Map colliders
        for coll in colliders::colliders() {
            p.spawn((
                Name::new("World Frame Collider"),
                WorldFrame,
                SpatialBundle::default(),
                coll,
                ColliderDebugColor(Color::RED),
                COLLIDE_ONLY_WITH_BALL,
                ActiveEvents::COLLISION_EVENTS,
            ));
        }
        // Ball starter
        let bs_pos = Vec3::new(1.175, 0.657, -0.018);
        super::ball_starter::spawn(p, bs_pos, &mut meshes, &mut mats);

        // Flipper left
        let fl_pos = Transform::from_xyz(0.83, -0.32, -0.043);
        super::flipper::spawn_left(fl_pos, p, assets);

        // Flipper right
        let fr_pos = Transform::from_xyz(0.83, 0.246, -0.043);
        super::flipper::spawn_right(fr_pos, p, assets);

        spawn_build_marks(p, assets);
        spawn_road(p, &mut mats, &mut meshes, assets);

        let life_bar_trans = Transform {
            translation: Vec3::new(1.15, -0.035, -0.05),
            scale: Vec3::new(4., 4., 4.),
            ..default()
        };
        spawn_life_bar(p, assets, &mut mats, life_bar_trans);
        p.spawn(pinball_menu_glass(assets, &mut mats));
        //img_handle = Some(spawn_point_display(p, &mut mats, &mut images, assets));
        pc_id.0 = analog_counter::spawn_10_digit(p, assets, Vec3::new(0.98, -0.563958, 0.01), None);
        lc_id.0 =
            analog_counter::spawn_2_digit(p, assets, Transform::from_xyz(0.98, 0.41, 0.01), None);
        spawn_level_up_lights(p, &g_sett);
        let level_lamp_pos = Vec3::new(1., 0.31, 0.06);
        spawn_lamp(
            p,
            &mut mats,
            assets,
            &g_sett,
            level_lamp_pos,
            Color::TOMATO,
            LevelUpLamp,
        );
    });
    //if let Some(img) = img_handle {
    //spawn_point_display_ui_and_cam(&mut cmds, assets, img);
    //}
}

#[cfg(not(debug_assertions))]
const TOWER_POSIS: [Vec3; 20] = [
    Vec3::new(-0.7, -0.49, -0.04),
    Vec3::new(-1.035, 0., -0.04),
    Vec3::new(-0.89, -0.49, -0.04),
    Vec3::new(-0.5, -0.4, -0.04),
    Vec3::new(-0.3, -0.51, -0.04),
    Vec3::new(-0.7, -0.2, -0.04),
    Vec3::new(-0.89, -0.21, -0.04),
    Vec3::new(-0.3, -0.17, -0.04),
    Vec3::new(0.1, -0.51, -0.04),
    Vec3::new(-0.5, 0.4, -0.04),
    Vec3::new(-0.904, -0.0, -0.04),
    Vec3::new(-0.1, 0.01, -0.04),
    Vec3::new(0.01, 0.4, -0.04),
    Vec3::new(-0.24, 0.4, -0.04),
    Vec3::new(-0.1, -0.26, -0.04),
    Vec3::new(-0.3, 0.17, -0.04),
    Vec3::new(-0.904, 0.24, -0.04),
    Vec3::new(-0.5, 0., -0.04),
    Vec3::new(-1.1, -0.55, -0.04),
    Vec3::new(0.11, 0.17, -0.04),
];

#[cfg(debug_assertions)]
const TOWER_POSIS: [Vec3; 3] = [
    Vec3::new(-0.3, -0.17, -0.04),
    Vec3::new(0.1, -0.51, -0.04),
    Vec3::new(-0.5, 0.4, -0.04),
];

fn spawn_build_marks(parent: &mut ChildBuilder, assets: &PinballDefenseGltfAssets) {
    for (i, pos) in TOWER_POSIS.iter().enumerate() {
        parent.spawn(foundation::build_mark(assets, *pos, i));
    }
}

fn on_ball_coll_wall_system(
    mut evr: EventReader<CollisionWithBallEvent>,
    mut sound_ev: EventWriter<SoundEvent>,
    q_wall: Query<With<WorldFrame>>,
) {
    for ev in evr.iter() {
        if q_wall.contains(ev.0) {
            sound_ev.send(SoundEvent::BallHitsWall);
        }
    }
}
