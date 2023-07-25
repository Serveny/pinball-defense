use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::tower::TowerType;
use crate::world::PinballWorld;
use crate::GameState;

pub struct PinballMenuPlugin;

impl Plugin for PinballMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPinballMenuEvent>().add_systems(
            Update,
            spawn_pinball_menu_system.run_if(in_state(GameState::Ingame)),
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
    meshes: &mut Assets<Mesh>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    parent
        .spawn(SpatialBundle::from_transform(Transform::from_translation(
            pos,
        )))
        .insert(PinballMenu)
        .insert(Name::new("Tower Menu"))
        .with_children(|parent| {
            spawn_menu_element(
                TowerType::MachineGun,
                parent,
                mats,
                meshes,
                assets,
                g_sett,
                Transform::from_rotation(Quat::from_rotation_y(-0.3)),
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
                Transform::from_rotation(Quat::from_rotation_y(0.3)),
            );
        });
}

#[derive(Component)]
pub struct PinballMenuElement;

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
        ))
        .insert(PinballMenuElement)
        .insert(Name::new("Tower Menu element"))
        .insert(tower_type);
}

#[allow(clippy::too_many_arguments)]
fn spawn_pinball_menu_system(
    mut evr: EventReader<SpawnPinballMenuEvent>,
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    assets: Res<PinballDefenseAssets>,
    q_pbw: Query<Entity, With<PinballWorld>>,
    q_pb_menu: Query<&PinballMenu>,
    g_sett: Res<GraphicsSettings>,
) {
    for _ in evr.iter() {
        if q_pb_menu.is_empty() {
            cmds.entity(q_pbw.single()).with_children(|parent| {
                spawn_pinball_menu(
                    parent,
                    &mut mats,
                    &mut meshes,
                    &assets,
                    &g_sett,
                    Vec3::new(1.2, 0.02, 0.05),
                );
            });
        }
    }
}
