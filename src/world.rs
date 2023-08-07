use crate::assets::PinballDefenseAssets;
use crate::ball::BallSpawn;
use crate::ball_starter::BallStarterPlugin;
use crate::enemy::spawn_enemy;
use crate::flipper::FlipperPlugin;
use crate::prelude::*;
use crate::road::spawn_road;
use crate::settings::GraphicsSettings;
use crate::tower::foundation::spawn_foundation;
use crate::GameState;

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
struct Ground;

fn spawn_pinball_world(
    mut cmds: Commands,
    mut ball_spawn: ResMut<BallSpawn>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut assets: ResMut<PinballDefenseAssets>,
    g_sett: Res<GraphicsSettings>,
) {
    cmds.spawn(SpatialBundle {
        transform: Transform::from_rotation(Quat::from_rotation_z(-0.25)),
        ..default()
    })
    .insert(PinballWorld)
    .insert(Name::new("Pinball World"))
    .with_children(|parent| {
        parent
            .spawn((
                PbrBundle {
                    mesh: assets.world_1_mesh.clone(),
                    material: materials.add(StandardMaterial {
                        base_color: Color::BLUE,
                        perceptual_roughness: 0.6,
                        metallic: 0.2,
                        reflectance: 0.4,
                        ..default()
                    }),
                    ..default()
                },
                //Ccd::enabled(),
                ColliderDebugColor(Color::GOLD),
                Collider::from_bevy_mesh(
                    meshes
                        .get(&assets.world_1_collision_mesh)
                        .expect("Failed to find mesh"),
                    &ComputedColliderShape::TriMesh,
                )
                .unwrap(),
            ))
            .insert(Ground);

        // Ball starter
        let bs_pos = Vec3::new(1.175, -0.018, -0.657);
        crate::ball_starter::spawn(parent, bs_pos, &mut meshes, &mut materials);

        // Flipper left
        let fl_pos = Transform::from_xyz(0.83, -0.043, 0.32);
        crate::flipper::spawn_left(fl_pos, parent, &mut materials, &mut assets);

        // Flipper right
        let fr_pos = Transform::from_xyz(0.83, -0.043, -0.246);
        crate::flipper::spawn_right(fr_pos, parent, &mut materials, &mut assets);

        spawn_foundations(parent, &mut materials, &assets, &g_sett);
        spawn_road(parent, &mut materials, &assets, &mut meshes);
        spawn_enemy(parent, &assets, &mut meshes, &mut materials, &g_sett);
        parent
            .spawn(TransformBundle::default())
            .insert(Name::new("Colliders"))
            .with_children(spawn_colliders);
    });
    ball_spawn.0 = Vec3::new(0.96, -0.26, -0.6);
}

fn spawn_colliders(p: &mut ChildBuilder) {
    let size = Vec3::new(1.30, 0.04, 0.70);
    spawn_cube_collider("Top Glass", p, size, Vec3::new(0., 0.09, 0.));
}

fn spawn_cube_collider(name: &'static str, parent: &mut ChildBuilder, size: Vec3, pos: Vec3) {
    parent
        .spawn((
            TransformBundle::from_transform(Transform::from_translation(pos)),
            ColliderDebugColor(Color::RED),
            Collider::cuboid(size.x, size.y, size.z),
        ))
        .insert(Name::new(name));
}

fn spawn_foundations(
    p: &mut ChildBuilder,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
) {
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.89, -0.04, 0.48));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.89, -0.04, 0.2));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.69, -0.04, -0.2));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.5, -0.04, -0.01));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.3, -0.04, 0.5));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.1, -0.04, -0.02));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(0.1, -0.04, 0.5));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.904, -0.04, -0.01));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(0.01, -0.04, -0.41));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.275, -0.04, -0.41));
    spawn_foundation(p, mats, assets, g_sett, Vec3::new(-0.5, -0.04, 0.25));
}
