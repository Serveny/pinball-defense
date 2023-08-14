use super::ball_starter::BallStarterPlugin;
use super::events::collision::COLLIDE_ONLY_WITH_BALL;
use super::flipper::FlipperPlugin;
use super::pinball_menu::spawn_pinball_menu_glass;
use super::player_life::spawn_life_bar;
use super::road::spawn_road;
use super::tower::foundation::spawn_foundation;
use super::GameState;
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
    mut assets: ResMut<PinballDefenseAssets>,
    g_sett: Res<GraphicsSettings>,
) {
    cmds.spawn((
        SpatialBundle {
            transform: Transform::from_rotation(Quat::from_rotation_z(-0.25)),
            ..default()
        },
        PinballWorld,
        Name::new("Pinball World"),
    ))
    .with_children(|parent| {
        // Frame
        parent.spawn((
            PbrBundle {
                mesh: assets.world_1_frame.clone(),
                material: assets.world_1_frame_material.clone(),
                ..default()
            },
            Collider::from_bevy_mesh(
                meshes
                    .get(&assets.world_1_frame_collider)
                    .expect("Failed to find mesh"),
                &ComputedColliderShape::TriMesh,
            )
            .unwrap(),
            Friction::new(0.4),
            ColliderDebugColor(Color::GOLD),
            COLLIDE_ONLY_WITH_BALL,
            WorldGround,
        ));

        // Ground
        parent.spawn((
            PbrBundle {
                mesh: assets.world_1_ground.clone(),
                material: assets.world_1_ground_material.clone(),
                ..default()
            },
            //Ccd::enabled(),
            Collider::from_bevy_mesh(
                meshes
                    .get(&assets.world_1_ground_collider)
                    .expect("Failed to find mesh"),
                &ComputedColliderShape::TriMesh,
            )
            .unwrap(),
            Friction::new(0.2),
            ColliderDebugColor(Color::GOLD),
            COLLIDE_ONLY_WITH_BALL,
            WorldGround,
        ));

        // Top Glass (maybe with glass texture in future)
        parent.spawn((
            spatial_from_pos(Vec3::new(0., 0.12, 0.)),
            //PbrBundle {
            //mesh: assets.world_1_ground_collider.clone(),
            //transform: Transform::from_translation(Vec3::new(0., 0.12, 0.)),
            //material: mats.add(StandardMaterial {
            //base_color: Color::WHITE,
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
            Name::new("Pinball top glass"),
        ));

        // Rebound left
        parent.spawn((
            PbrBundle {
                mesh: assets.world_1_rebound_left.clone(),
                material: assets.world_1_rebound_left_material.clone(),
                ..default()
            },
            Name::new("Pinball Rebound Left"),
            Collider::from_bevy_mesh(
                meshes
                    .get(&assets.world_1_rebound_left_collider.clone())
                    .expect("Failed to find mesh"),
                &ComputedColliderShape::TriMesh,
            )
            .unwrap(),
            Friction::new(0.4),
            ColliderDebugColor(Color::GOLD),
            COLLIDE_ONLY_WITH_BALL,
        ));

        // Rebound right
        parent.spawn((
            PbrBundle {
                mesh: assets.world_1_rebound_right.clone(),
                material: assets.world_1_rebound_right_material.clone(),
                ..default()
            },
            Collider::from_bevy_mesh(
                meshes
                    .get(&assets.world_1_rebound_right_collider.clone())
                    .expect("Failed to find mesh"),
                &ComputedColliderShape::TriMesh,
            )
            .unwrap(),
            Friction::new(0.4),
            ColliderDebugColor(Color::GOLD),
            Name::new("Pinball Dodger Right"),
        ));

        // Ball starter
        let bs_pos = Vec3::new(1.175, -0.018, -0.657);
        super::ball_starter::spawn(parent, bs_pos, &mut meshes, &mut mats);

        // Flipper left
        let fl_pos = Transform::from_xyz(0.83, -0.043, 0.32);
        super::flipper::spawn_left(fl_pos, parent, &mut mats, &mut assets);

        // Flipper right
        let fr_pos = Transform::from_xyz(0.83, -0.043, -0.246);
        super::flipper::spawn_right(fr_pos, parent, &mut mats, &mut assets);

        spawn_foundations(parent, &mut mats, &assets, &g_sett);
        spawn_road(parent, &mut mats, &mut meshes, &assets);

        let lb_pos = Transform {
            translation: Vec3::new(1.15, -0.05, 0.035),
            scale: Vec3::new(4., 4., 4.),
            ..default()
        };
        spawn_life_bar(parent, &assets, &mut mats, lb_pos);
        spawn_pinball_menu_glass(parent, &assets, &mut mats);
    });
}

fn spawn_foundations(
    p: &mut ChildBuilder,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
) {
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.89, -0.04, 0.49));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.89, -0.04, 0.21));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.69, -0.04, -0.19));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.5, -0.04, 0.));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.3, -0.04, 0.51));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.1, -0.04, -0.01));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(0.1, -0.04, 0.51));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.904, -0.04, 0.0));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(0.01, -0.04, -0.4));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.275, -0.04, -0.4));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.5, -0.04, 0.26));
}
