use super::events::collision::GameLayer;

use super::analog_counter;
use super::level::{LevelCounterId, PointCounterId};
use super::light::LevelUpLamp;
use super::light::spawn_lamp;
use super::pinball_menu::pinball_menu_glass;
use super::player_life::spawn_life_bar;
use super::road::spawn_road;
use super::tower::foundation;
use super::wave::WaveCounterId;
use crate::assets::PinballDefenseGltfAssets;
use crate::generated::world_1::colliders;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use bevy::color::palettes::css::{RED, TOMATO};

pub type QueryWorld<'w, 's> = Query<'w, 's, Entity, With<PinballWorld>>;

#[derive(Component)]
pub struct PinballWorld;

#[derive(Component)]
pub struct WorldFrame;

pub fn spawn_pinball_world(
    mut cmds: Commands,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    gltf_assets: Res<PinballDefenseGltfAssets>,
    tex_assets: Res<PinballDefenseAssets>,
    g_sett: Res<GraphicsSettings>,
) {
    let assets = gltf_assets.as_ref();
    cmds.spawn((
        PinballWorld,
        Name::new("Pinball World"),
        Transform::default(),
        Visibility::default(),
    ))
    .with_children(|p| {
        // World mesh
        p.spawn((
            Mesh3d(assets.world_1.clone()),
            MeshMaterial3d(assets.world_1_material.clone()),
        ));

        // Map colliders
        p.spawn((
            Name::new("World Frame Collider"),
            WorldFrame,
            CollisionLayers::new(GameLayer::Map, GameLayer::Ball),
            RigidBody::Static,
        ))
        .with_children(|p| {
            for coll in colliders::colliders() {
                p.spawn((
                    coll,
                    WorldFrame,
                    DebugRender::default().with_collider_color(RED.into()),
                    CollisionMargin(0.008),
                ));
            }
        });

        // Ball starter
        let bs_pos = Vec3::new(1.289, 0.67, -0.018);
        super::ball_starter::spawn(p, bs_pos, assets);

        // Flipper left
        let f_left_pos = Transform::from_xyz(0.83, -0.32, -0.043);
        super::flipper::spawn_left(f_left_pos, p, assets);

        // Flipper right
        let f_right_pos = Transform::from_xyz(0.83, 0.246, -0.043);
        super::flipper::spawn_right(f_right_pos, p, assets);

        spawn_build_marks(p, assets);
        super::extra_field::spawn_fields(p, &mut meshes, &mut mats, &g_sett, EXTRA_FIELD_POSIS);
        spawn_road(p, assets);

        let life_bar_trans = Transform {
            translation: Vec3::new(1.15, -0.035, -0.05),
            scale: Vec3::new(4., 4., 4.),
            ..default()
        };
        spawn_life_bar(p, assets, &mut mats, life_bar_trans);
        p.spawn(pinball_menu_glass(assets, &mut mats));
        let pc = analog_counter::spawn_10_digit(p, assets, Vec3::new(0.98, -0.563_958, 0.01), None);
        p.commands_mut().insert_resource(PointCounterId(pc));
        let lc = analog_counter::spawn_2_digit(
            p,
            assets,
            Transform::from_xyz(0.98, 0.41, 0.01),
            None,
            &assets.level_sign_material,
        );
        p.commands_mut().insert_resource(LevelCounterId(lc));
        let wave_sign_mat = mats.add(StandardMaterial {
            base_color_texture: Some(tex_assets.mini_sign_wave.clone()),
            perceptual_roughness: 0.2,
            ..default()
        });
        let wc = analog_counter::spawn_2_digit(
            p,
            assets,
            Transform::from_xyz(0.98, 0.48, 0.01),
            None,
            &wave_sign_mat,
        );
        p.commands_mut().insert_resource(WaveCounterId(wc));
        let level_lamp_pos = Vec3::new(1., 0.31, 0.06);
        spawn_lamp(
            p,
            &mut mats,
            assets,
            &g_sett,
            level_lamp_pos,
            TOMATO.into(),
            LevelUpLamp,
        );
    });
}

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

const EXTRA_FIELD_POSIS: [Vec3; 4] = [
    Vec3::new(-1.05, 0.45, -0.04),
    Vec3::new(-0.55, 0.62, -0.04),
    Vec3::new(0.45, 0.62, -0.04),
    Vec3::new(-1.03, -0.38, -0.04),
];

fn spawn_build_marks(spawner: &mut ChildSpawnerCommands, assets: &PinballDefenseGltfAssets) {
    for (i, pos) in TOWER_POSIS.iter().enumerate() {
        spawner.spawn(foundation::build_mark(assets, *pos, i));
    }
}
