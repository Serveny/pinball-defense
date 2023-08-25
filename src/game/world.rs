use super::ball_starter::BallStarterPlugin;
use super::events::collision::COLLIDE_ONLY_WITH_BALL;
use super::flipper::FlipperPlugin;
use super::level::{LevelCounterId, PointCounterId};
use super::pinball_menu::pinball_menu_glass;
use super::player_life::spawn_life_bar;
use super::road::spawn_road;
use super::tower::foundation;
use super::{analog_counter, GameState};
use crate::assets::PinballDefenseGltfAssets;
use crate::prelude::*;
use crate::settings::GraphicsSettings;

pub type QueryWorld<'w, 's> = Query<'w, 's, Entity, With<PinballWorld>>;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FlipperPlugin)
            .add_plugins(BallStarterPlugin)
            .add_systems(OnEnter(GameState::Ingame), spawn_pinball_world);
    }
}

#[derive(Component)]
pub struct PinballWorld;

#[derive(Component)]
struct WorldGround;

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
        for coll in super::colliders::colliders() {
            p.spawn((
                SpatialBundle::default(),
                coll,
                ColliderDebugColor(Color::RED),
                COLLIDE_ONLY_WITH_BALL,
            ));
        }
        // Ball starter
        let bs_pos = Vec3::new(1.175, 0.657, -0.018);
        super::ball_starter::spawn(p, bs_pos, &mut meshes, &mut mats);

        // Flipper left
        let fl_pos = Transform::from_xyz(0.83, -0.32, -0.043);
        super::flipper::spawn_left(fl_pos, p, &mut mats, assets);

        // Flipper right
        let fr_pos = Transform::from_xyz(0.83, 0.246, -0.043);
        super::flipper::spawn_right(fr_pos, p, &mut mats, assets);

        spawn_foundations(p, &mut mats, assets, &g_sett);
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
    });
    //if let Some(img) = img_handle {
    //spawn_point_display_ui_and_cam(&mut cmds, assets, img);
    //}
}

const TOWER_POSIS: [Vec3; 12] = [
    Vec3::new(-0.89, -0.49, -0.04),
    Vec3::new(-0.71, -0.49, -0.04),
    Vec3::new(-0.89, -0.21, -0.04),
    Vec3::new(-0.904, 0.24, -0.04),
    Vec3::new(-0.5, 0., -0.04),
    Vec3::new(-0.3, -0.51, -0.04),
    Vec3::new(-0.1, 0.01, -0.04),
    Vec3::new(0.1, -0.51, -0.04),
    Vec3::new(-0.904, -0.0, -0.04),
    Vec3::new(0.01, 0.4, -0.04),
    Vec3::new(-0.275, 0.4, -0.04),
    Vec3::new(-0.5, -0.26, -0.04),
];

fn spawn_foundations(
    parent: &mut ChildBuilder,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
    g_sett: &GraphicsSettings,
) {
    for pos in TOWER_POSIS {
        foundation::spawn(parent, mats, assets, g_sett, pos);
    }
}
