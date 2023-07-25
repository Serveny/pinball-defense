use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::tower::TowerType;
use crate::utils::collision_events::ActivatePinballMenuEvent;
use crate::world::PinballWorld;
use crate::GameState;
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotateYLens, TransformRotateZLens},
    Animator, Delay, EaseFunction, Sequence, Tween,
};
use std::time::Duration;

pub struct PinballMenuPlugin;

impl Plugin for PinballMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPinballMenuEvent>()
            .add_event::<ActivatePinballMenuEvent>()
            .add_systems(
                Update,
                (spawn_pinball_menu_system, activate_system).run_if(in_state(GameState::Ingame)),
            );
    }
}

#[derive(Component)]
pub struct PinballMenu;

#[derive(Event)]
pub struct SpawnPinballMenuEvent;

pub fn spawn_pinball_menu(
    parent: &mut ChildBuilder,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    use TowerType::*;
    parent
        .spawn(spatial_from_pos(pos))
        .insert(PinballMenu)
        .insert(Name::new("Tower Menu"))
        .with_children(|p| {
            spawn_menu_element(MachineGun, p, mats, assets, g_sett, -0.3, 0.1);
            spawn_menu_element(Microwave, p, mats, assets, g_sett, 0., 1.);
            spawn_menu_element(Tesla, p, mats, assets, g_sett, 0.3, 0.1);
        });
}

#[derive(Component)]
pub struct PinballMenuElement;

#[allow(clippy::too_many_arguments)]
fn spawn_menu_element(
    tower_type: TowerType,
    parent: &mut ChildBuilder,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    angle: f32,
    delay_secs: f32,
) {
    parent
        .spawn((PbrBundle {
            mesh: assets.menu_element.clone(),
            material: mats.add(StandardMaterial {
                base_color: Color::MIDNIGHT_BLUE,
                perceptual_roughness: 0.6,
                metallic: 0.2,
                reflectance: 0.8,
                ..default()
            }),
            transform: Transform::from_translation(elem_start_pos()),
            ..default()
        },))
        .insert(PinballMenuElement)
        .insert(Name::new("Tower Menu element"))
        .insert(tower_type)
        .insert(Animator::new(spawn_animation(angle, delay_secs)));
}

fn elem_start_pos() -> Vec3 {
    Vec3::new(0., -0.1, 0.)
}

fn spawn_pinball_menu_system(
    mut cmds: Commands,
    mut mats: ResMut<Assets<StandardMaterial>>,
    evr: EventReader<SpawnPinballMenuEvent>,
    assets: Res<PinballDefenseAssets>,
    q_pbw: Query<Entity, With<PinballWorld>>,
    q_pb_menu: Query<&PinballMenu>,
    g_sett: Res<GraphicsSettings>,
) {
    if !evr.is_empty() && q_pb_menu.is_empty() {
        cmds.entity(q_pbw.single()).with_children(|p| {
            spawn_pinball_menu(p, &mut mats, &assets, &g_sett, Vec3::new(1.2, 0.02, 0.05));
        });
    }
}

fn spawn_animation(angle: f32, delay_secs: f32) -> Sequence<Transform> {
    let wait = Delay::new(Duration::from_secs_f32(delay_secs));
    let slide_up = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_secs(1),
        TransformPositionLens {
            start: elem_start_pos(),
            end: Vec3::default(),
        },
    );
    let rotate = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_secs(1),
        TransformRotateYLens {
            start: 0.,
            end: angle,
        },
    );

    wait.then(slide_up).then(rotate)
}

fn activate_system(
    mut cmds: Commands,
    evr: EventReader<ActivatePinballMenuEvent>,
    meshes: Res<Assets<Mesh>>,
    q_pbm_el: Query<Entity, With<PinballMenuElement>>,
    assets: Res<PinballDefenseAssets>,
) {
    if !evr.is_empty() {
        //println!("😆 Activate Event");
        for entity in q_pbm_el.iter() {
            //println!("🔥Collider added");
            cmds.entity(entity).insert((
                ColliderDebugColor(Color::GREEN),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                Collider::from_bevy_mesh(
                    meshes
                        .get(&assets.menu_element.clone())
                        .expect("Failed to find mesh"),
                    &ComputedColliderShape::TriMesh,
                )
                .unwrap(),
            ));
        }
    }
}
