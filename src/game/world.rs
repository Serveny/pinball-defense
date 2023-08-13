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
struct Ground;

fn spawn_pinball_world(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
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
                    material: assets.world_1_material.clone(),
                    ..default()
                },
                //Ccd::enabled(),
                Collider::from_bevy_mesh(
                    meshes
                        .get(&assets.world_1_collision_mesh)
                        .expect("Failed to find mesh"),
                    &ComputedColliderShape::TriMesh,
                )
                .unwrap(),
                Friction::new(0.2),
                ColliderDebugColor(Color::GOLD),
                COLLIDE_ONLY_WITH_BALL,
            ))
            .insert(Ground);

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
        //spawn_enemy(parent, &assets, &mut meshes, &mut materials, &g_sett);
        parent
            .spawn(TransformBundle::default())
            .insert(Name::new("Colliders"))
            .with_children(spawn_colliders);
    });
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
