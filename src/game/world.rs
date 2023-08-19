use super::ball_starter::BallStarterPlugin;
use super::events::collision::COLLIDE_ONLY_WITH_BALL;
use super::flipper::FlipperPlugin;
use super::level::{LevelCounterId, PointCounterId};
use super::pinball_menu::spawn_pinball_menu_glass;
use super::player_life::spawn_life_bar;
use super::road::spawn_road;
use super::tower::foundation;
use super::{analog_counter, GameState};
use crate::assets::PinballDefenseAssets;
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
    assets: Res<PinballDefenseAssets>,
    g_sett: Res<GraphicsSettings>,
) {
    let assets = assets.as_ref();
    //let mut img_handle: Option<Handle<Image>> = None;
    cmds.spawn((
        SpatialBundle {
            transform: Transform::from_rotation(Quat::from_rotation_z(-0.25)),
            ..default()
        },
        PinballWorld,
        Name::new("Pinball World"),
    ))
    .with_children(|p| {
        // World mesh
        p.spawn((
            PbrBundle {
                mesh: assets.world_1.clone(),
                material: assets.world_1_material.clone(),
                ..default()
            },
            //super::colliders::create_collider(),
            //ColliderDebugColor(Color::RED),
        ));

        spawn_colliders(p, &mut meshes, assets);

        // Top Glass (maybe with glass texture in future)
        p.spawn((
            Name::new("Pinball top glass"),
            spatial_from_pos(Vec3::new(0., 0.12, 0.)),
            //PbrBundle {
            //mesh: assets.world_1_ground_collider.clone(),
            //transform: Transform::from_translation(Vec3::new(0., 0.12, 0.)),
            //material: mats.add(StandardMaterial {
            //base_color: Color::rgba_u8(255, 255, 255, 25),
            //perceptual_roughness: 0.,
            //metallic: 0.,
            //reflectance: 0.6,
            //alpha_mode: AlphaMode::Multiply,
            //..default()
            //}),
            //..default()
            //},
            Collider::from_bevy_mesh(
                meshes
                    .get(&assets.world_1_ground_collider)
                    .expect("Failed to find mesh"),
                &ComputedColliderShape::TriMesh,
            )
            .unwrap(),
            ColliderDebugColor(Color::NONE),
            COLLIDE_ONLY_WITH_BALL,
        ));

        // Ball starter
        let bs_pos = Vec3::new(1.175, -0.018, -0.657);
        super::ball_starter::spawn(p, bs_pos, &mut meshes, &mut mats);

        // Flipper left
        let fl_pos = Transform::from_xyz(0.83, -0.043, 0.32);
        super::flipper::spawn_left(fl_pos, p, &mut mats, assets);

        // Flipper right
        let fr_pos = Transform::from_xyz(0.83, -0.043, -0.246);
        super::flipper::spawn_right(fr_pos, p, &mut mats, assets);

        spawn_foundations(p, &mut mats, assets, &g_sett);
        spawn_road(p, &mut mats, &mut meshes, assets);

        let life_bar_trans = Transform {
            translation: Vec3::new(1.15, -0.05, 0.035),
            scale: Vec3::new(4., 4., 4.),
            ..default()
        };
        spawn_life_bar(p, assets, &mut mats, life_bar_trans);
        spawn_pinball_menu_glass(p, assets, &mut mats);
        //img_handle = Some(spawn_point_display(p, &mut mats, &mut images, assets));
        pc_id.0 = analog_counter::spawn_10_digit(p, assets, Vec3::new(0.98, 0.01, 0.563958), None);
        lc_id.0 =
            analog_counter::spawn_2_digit(p, assets, Transform::from_xyz(0.98, 0.01, -0.41), None);
    });
    //if let Some(img) = img_handle {
    //spawn_point_display_ui_and_cam(&mut cmds, assets, img);
    //}
}

fn spawn_colliders(p: &mut ChildBuilder, meshes: &mut Assets<Mesh>, assets: &PinballDefenseAssets) {
    let mesh = &assets.world_1_frame_collider;
    p.spawn(ball_coll("Frame Collider", meshes, mesh, 0.4));

    // Ground Collider
    let mesh = &assets.world_1_ground_collider;
    p.spawn(ball_coll("Ground Collider", meshes, mesh, 0.2));
}

fn ball_coll(
    name: &'static str,
    meshes: &Assets<Mesh>,
    handle: &Handle<Mesh>,
    friction: f32,
) -> impl Bundle {
    (
        Name::new(name),
        SpatialBundle::default(),
        Collider::from_bevy_mesh(
            meshes.get(handle).expect("Failed to find mesh"),
            &ComputedColliderShape::TriMesh,
        )
        .unwrap(),
        Friction::new(friction),
        ColliderDebugColor(Color::GOLD),
    )
}

const TOWER_POSIS: [Vec3; 12] = [
    Vec3::new(-0.89, -0.04, 0.49),
    Vec3::new(-0.71, -0.04, 0.49),
    Vec3::new(-0.89, -0.04, 0.21),
    Vec3::new(-0.904, -0.04, -0.24),
    Vec3::new(-0.5, -0.04, 0.),
    Vec3::new(-0.3, -0.04, 0.51),
    Vec3::new(-0.1, -0.04, -0.01),
    Vec3::new(0.1, -0.04, 0.51),
    Vec3::new(-0.904, -0.04, 0.0),
    Vec3::new(0.01, -0.04, -0.4),
    Vec3::new(-0.275, -0.04, -0.4),
    Vec3::new(-0.5, -0.04, 0.26),
];

fn spawn_foundations(
    parent: &mut ChildBuilder,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
) {
    for pos in TOWER_POSIS {
        foundation::spawn(parent, mats, assets, g_sett, pos);
    }
}
