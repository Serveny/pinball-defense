use super::ball::CollisionWithBallEvent;
use super::events::collision::COLLIDE_ONLY_WITH_BALL;
use super::events::tween_completed::{ACTIVATE_PINBALL_MENU_EVENT_ID, DESPAWN_ENTITY_EVENT_ID};
use super::tower::foundation::{DespawnFoundationEvent, QuerySelected, SelectedTowerFoundation};
use super::tower::{SpawnTowerEvent, TowerType};
use super::world::QueryWorld;
use super::GameState;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;
use bevy_tweening::{lens::TransformRotateYLens, Animator, Delay, EaseFunction, Sequence, Tween};
use std::time::Duration;

pub struct PinballMenuPlugin;

impl Plugin for PinballMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PinballMenuEvent>().add_systems(
            Update,
            (menu_event_system, spawn_system, execute_system).run_if(in_state(GameState::Ingame)),
        );
    }
}

#[derive(Event, Debug, Clone, Copy)]
pub enum PinballMenuEvent {
    Disable,
    SetReady,
    Activate,
    Deactivate,
}

#[derive(Component, Debug, Clone, Copy, Default)]
enum PinballMenuStatus {
    #[default]
    Disabled,
    Ready,
    Activated,
}

fn menu_event_system(
    mut evr: EventReader<PinballMenuEvent>,
    mut q_pb_menu: Query<(Entity, &mut PinballMenuStatus), With<PinballMenu>>,
    cmds: Commands,
    q_pbm_el: QueryPinballMenuElements,
    q_lights: Query<&mut Visibility, With<PinballMenuElementLight>>,
    meshes: Res<Assets<Mesh>>,
    assets: Res<PinballDefenseAssets>,
) {
    if let Some(ev) = evr.iter().next() {
        if let Ok((menu_entity, mut status)) = q_pb_menu.get_single_mut() {
            use PinballMenuEvent::*;
            use PinballMenuStatus::*;
            if let Some(new_status) = match (ev, *status) {
                (Disable, Activated) => Some(despawn(cmds, q_lights, q_pbm_el, menu_entity)),
                (SetReady, Disabled) => Some(Ready),
                (Deactivate, Activated) => Some(deactivate(cmds, q_lights, q_pbm_el)), // TODO Status setzen
                (Activate, Ready) => Some(activate(cmds, q_lights, q_pbm_el, meshes, assets)),
                _ => None,
            } {
                *status = new_status;
            }
        }
    }
}

type QueryPinballMenuElements<'w, 's, 'a> =
    Query<'w, 's, (Entity, &'a Transform), With<PinballMenuElement>>;

fn spawn_system(
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
    q_pbw: QueryWorld,
    q_pb_menu: Query<&PinballMenu>,
    g_sett: Res<GraphicsSettings>,
    q_selected: Query<Entity, With<SelectedTowerFoundation>>,
) {
    if !q_selected.is_empty() && q_pb_menu.is_empty() {
        let selected_foundation_id = q_pbw.single();
        log!("🐢 Spawn tower menu for: {:?}", selected_foundation_id);
        cmds.entity(selected_foundation_id).with_children(|p| {
            spawn(p, &assets, &g_sett, MENU_POS);
        });
    }
}

#[derive(Component)]
struct PinballMenu;

fn spawn(
    parent: &mut ChildBuilder,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    use TowerType::*;
    parent
        .spawn((
            spatial_from_pos(pos),
            PinballMenu,
            PinballMenuStatus::Disabled,
            Name::new("Tower Menu"),
        ))
        .with_children(|p| {
            let el = assets.pinball_menu_element.clone();
            let gun_mat = assets.pinball_menu_element_gun_material.clone();
            let tesla_mat = assets.pinball_menu_element_tesla_material.clone();
            let microwave_mat = assets.pinball_menu_element_microwave_material.clone();
            spawn_menu_element(Gun, p, el.clone(), gun_mat, g_sett, -0.25, 1.25);
            spawn_menu_element(Microwave, p, el.clone(), microwave_mat, g_sett, 0., 0.75);
            spawn_menu_element(Tesla, p, el, tesla_mat, g_sett, 0.25, 0.25);
        });
}

// Only pub(crate)for collision events
#[derive(Component)]
struct PinballMenuElement;

#[derive(Component)]
struct PinballMenuElementLight;

fn spawn_menu_element(
    tower_type: TowerType,
    parent: &mut ChildBuilder,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    g_sett: &GraphicsSettings,
    angle: f32,
    delay_secs: f32,
) {
    parent
        .spawn((
            PbrBundle {
                mesh,
                material,
                transform: Transform {
                    rotation: Quat::from_rotation_y(ELEM_START_ANGLE),
                    ..default()
                },
                ..default()
            },
            // Game components
            PinballMenuElement,
            Name::new("Tower Menu element"),
            tower_type,
            // Spawn animation
            Animator::new(spawn_animation(angle, delay_secs)),
        ))
        .with_children(|parent| {
            // Active status light
            parent.spawn((
                SpotLightBundle {
                    transform: Transform::from_translation(Vec3::new(-0.79, 0., 0.))
                        .looking_at(Vec3::new(-1.0, 0.0, 0.0), Vec3::Z),
                    spot_light: SpotLight {
                        intensity: 28., // lumens - roughly a 100W non-halogen incandescent bulb
                        color: Color::BEIGE,
                        shadows_enabled: g_sett.is_shadows,
                        range: 0.2,
                        inner_angle: 0.2,
                        outer_angle: 0.8,
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                },
                //PointLightBundle {
                //visibility: Visibility::Hidden,
                //point_light: PointLight {
                //color: Color::BEIGE,
                //intensity: 0.2,
                //shadows_enabled: g_sett.is_shadows,
                //..default()
                //},
                //transform: Transform::from_translation(Vec3::new(-0.84, 0., 0.)),
                //..default()
                //},
                PinballMenuElementLight,
            ));
        });
}

fn despawn(
    mut cmds: Commands,
    q_lights: Query<&mut Visibility, With<PinballMenuElementLight>>,
    q_pbm_el: QueryPinballMenuElements,
    menu_entity: Entity,
) -> PinballMenuStatus {
    // Despawn menu
    let delay: Delay<Transform> =
        Delay::new(Duration::from_secs(2)).with_completed_event(DESPAWN_ENTITY_EVENT_ID);
    cmds.entity(menu_entity).insert(Animator::new(delay));
    // Despawn animation
    q_pbm_el.for_each(|(entity, trans)| {
        let secs = (trans.rotation.y + 0.2) * 2.;
        log!("Durationn for {}: {}", trans.rotation.y, secs);
        cmds.entity(entity).insert(Animator::new(despawn_animation(
            trans.rotation.y,
            Duration::from_secs_f32(secs),
        )));
    });
    deactivate(cmds, q_lights, q_pbm_el);
    PinballMenuStatus::Disabled
}

fn activate(
    mut cmds: Commands,
    mut q_lights: Query<&mut Visibility, With<PinballMenuElementLight>>,
    q_pbm_el: QueryPinballMenuElements,
    meshes: Res<Assets<Mesh>>,
    assets: Res<PinballDefenseAssets>,
) -> PinballMenuStatus {
    q_pbm_el.for_each(|(entity, _)| {
        cmds.entity(entity).insert((
            // Active status collider
            ColliderDebugColor(Color::GREEN),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            Collider::from_bevy_mesh(
                meshes
                    .get(&assets.pinball_menu_element_collider.clone())
                    .expect("Failed to find mesh"),
                &ComputedColliderShape::TriMesh,
            )
            .unwrap(),
            COLLIDE_ONLY_WITH_BALL,
        ));
    });
    q_lights.for_each_mut(|mut visi| *visi = Visibility::Inherited);
    PinballMenuStatus::Activated
}

fn deactivate(
    mut cmds: Commands,
    mut q_lights: Query<&mut Visibility, With<PinballMenuElementLight>>,
    q_pbm_el: QueryPinballMenuElements,
) -> PinballMenuStatus {
    q_pbm_el.for_each(|(entity, _)| {
        cmds.entity(entity).remove::<Collider>();
    });
    q_lights.for_each_mut(|mut visi| *visi = Visibility::Hidden);
    PinballMenuStatus::Ready
}

const ELEM_START_ANGLE: f32 = -0.58;

fn spawn_animation(angle: f32, delay_secs: f32) -> Sequence<Transform> {
    let wait = Delay::new(Duration::from_secs_f32(delay_secs));
    let rotate = Tween::new(
        EaseFunction::ElasticOut,
        Duration::from_secs_f32(2.),
        TransformRotateYLens {
            start: ELEM_START_ANGLE + 0.2,
            end: angle,
        },
    );

    wait.then(rotate.with_completed_event(ACTIVATE_PINBALL_MENU_EVENT_ID))
}

fn despawn_animation(angle: f32, duration: Duration) -> Sequence<Transform> {
    let wait = Delay::new(duration);
    let rotate = Tween::new(
        EaseFunction::ExponentialInOut,
        duration,
        TransformRotateYLens {
            start: angle,
            end: ELEM_START_ANGLE,
        },
    );
    wait.then(rotate)
}

fn execute_system(
    mut ball_coll_ev: EventReader<CollisionWithBallEvent>,
    mut despawn_foundation_ev: EventWriter<DespawnFoundationEvent>,
    mut pb_menu_ev: EventWriter<PinballMenuEvent>,
    mut spawn_tower_ev: EventWriter<SpawnTowerEvent>,
    q_menu_els: Query<(Entity, &TowerType), With<PinballMenuElement>>,
    q_selected: QuerySelected,
) {
    for CollisionWithBallEvent(id, flag) in ball_coll_ev.iter() {
        if *flag == CollisionEventFlags::SENSOR {
            if let Some((_, tower_type)) = q_menu_els.iter().find(|(el_id, _)| *el_id == *id) {
                if let Ok((_, sel_trans)) = q_selected.get_single() {
                    despawn_foundation_ev.send(DespawnFoundationEvent);

                    // Despawn menu
                    pb_menu_ev.send(PinballMenuEvent::Disable);

                    // Spawn new tower
                    let pos = sel_trans.translation;
                    spawn_tower_ev.send(SpawnTowerEvent(
                        *tower_type,
                        Vec3::new(pos.x, -0.025, pos.z),
                    ));

                    return;
                }
            }
        }
    }
}

const MENU_POS: Vec3 = Vec3::new(1.3, 0., 0.038);

pub fn spawn_pinball_menu_glass(
    parent: &mut ChildBuilder,
    assets: &PinballDefenseAssets,
    mats: &mut Assets<StandardMaterial>,
) {
    parent.spawn((
        PbrBundle {
            mesh: assets.world_1_menu_glass.clone(),
            material: mats.add(StandardMaterial {
                base_color: Color::ALICE_BLUE,
                perceptual_roughness: 0.,
                metallic: 0.,
                reflectance: 0.6,
                alpha_mode: AlphaMode::Multiply,
                ..default()
            }),
            transform: Transform::from_translation(MENU_POS),
            ..default()
        },
        Name::new("Pinball menu glass"),
    ));
}
