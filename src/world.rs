use crate::assets::PinballDefenseAssets;
use crate::ball::BallSpawn;
use crate::ball_starter::BallStarterPlugin;
use crate::flipper::FlipperPlugin;
use crate::prelude::*;
use crate::road::{add_road_path, animate_cube, spawn_road};
use crate::settings::GraphicsSettings;
use crate::tower::{
    spawn_tower_foundation, spawn_tower_machine_gun, spawn_tower_microwave, spawn_tower_tesla,
    TowerType,
};
use crate::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FlipperPlugin)
            .add_plugins(BallStarterPlugin)
            .add_systems(OnEnter(GameState::Ingame), setup_pinball_world)
            .add_systems(Update, animate_cube.run_if(in_state(GameState::Ingame)));
    }
}

#[derive(Component)]
pub struct PinballWorld;

#[derive(Component)]
struct Ground;

fn setup_pinball_world(
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

        //parent.spawn(PointLightBundle {
        //transform: Transform::from_xyz(1., 1., 0.5).looking_at(Vec3::ZERO, Vec3::Y),
        //point_light: PointLight {
        //intensity: 78.,
        //color: Color::WHITE,
        //shadows_enabled: g_sett.is_shadows,
        //radius: 0.1,
        //range: 4.,
        //..default()
        //},
        //..default()
        //});

        // Ball starter
        let bs_pos = Vec3::new(1.175, -0.018, -0.657);
        crate::ball_starter::spawn(parent, bs_pos, &mut meshes, &mut materials);

        // Flipper left
        let fl_pos = Transform::from_xyz(0.83, -0.043, 0.32);
        crate::flipper::spawn_left(fl_pos, parent, &mut materials, &mut assets);

        // Flipper right
        let fr_pos = Transform::from_xyz(0.83, -0.043, -0.246);
        crate::flipper::spawn_right(fr_pos, parent, &mut materials, &mut assets);

        test_tower(parent, &mut materials, &mut meshes, &assets, &g_sett);
        spawn_road(parent, &mut materials, &assets);
        add_road_path(parent, &assets, &mut meshes, &mut materials);
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

fn test_tower(
    parent: &mut ChildBuilder,
    mats: &mut Assets<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
) {
    spawn_tower_microwave(parent, mats, assets, g_sett, Vec3::new(0., -0.025, -0.2));
    spawn_tower_machine_gun(parent, mats, assets, g_sett, Vec3::new(0., -0.025, 0.2));
    spawn_tower_tesla(parent, mats, assets, g_sett, Vec3::new(0., -0.025, 0.));
    spawn_tower_foundation(parent, mats, assets, g_sett, Vec3::new(0.1, -0.04, 0.));
    spawn_menu(
        parent,
        mats,
        meshes,
        assets,
        g_sett,
        Vec3::new(1.2, 0., 0.05),
    );
}

#[derive(Component)]
struct Menu;

fn spawn_menu(
    parent: &mut ChildBuilder,
    mats: &mut Assets<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    parent
        .spawn(SpatialBundle::from_transform(Transform::from_translation(
            pos,
        )))
        .insert(Menu)
        .insert(Name::new("Tower Menu"))
        .with_children(|parent| {
            spawn_menu_element(
                TowerType::MachineGun,
                parent,
                mats,
                meshes,
                assets,
                g_sett,
                Transform::from_rotation(Quat::from_rotation_y(-0.2)),
            );
            spawn_menu_element(
                TowerType::Microwave,
                parent,
                mats,
                meshes,
                assets,
                g_sett,
                Transform::from_rotation(Quat::from_rotation_y(0.0)),
            );
            spawn_menu_element(
                TowerType::Tesla,
                parent,
                mats,
                meshes,
                assets,
                g_sett,
                Transform::from_rotation(Quat::from_rotation_y(0.2)),
            );
        });
}

#[derive(Component)]
pub struct MenuElement;

fn spawn_menu_element(
    tower_type: TowerType,
    parent: &mut ChildBuilder,
    mats: &mut Assets<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    transform: Transform,
) {
    parent
        .spawn((
            PbrBundle {
                mesh: assets.menu_element.clone(),
                material: mats.add(StandardMaterial {
                    base_color: Color::MIDNIGHT_BLUE,
                    perceptual_roughness: 0.6,
                    metallic: 0.2,
                    reflectance: 0.8,
                    ..default()
                }),
                transform,
                ..default()
            },
            ColliderDebugColor(Color::GOLD),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            Collider::from_bevy_mesh(
                meshes
                    .get(&assets.menu_element.clone())
                    .expect("Failed to find mesh"),
                &ComputedColliderShape::TriMesh,
            )
            .unwrap(),
        ))
        .insert(MenuElement)
        .insert(Name::new("Tower Menu element"))
        .insert(tower_type);
}
