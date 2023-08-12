use super::enemy::Enemy;
use super::progress_bar::ProgressBarCountUpEvent;
use super::world::QueryWorld;
use super::GameState;
use crate::prelude::*;
use crate::utils::RelEntity;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                damage_system,
                count_down_life_time_system,
                delete_damage_system,
                hit_marker_system,
                delete_hit_marker_system,
            )
                .run_if(in_state(GameState::Ingame)),
        );
    }
}

#[derive(Component, Default)]
pub struct DamageList(pub Vec<RadiusDamage>);

pub struct RadiusDamage {
    pub pos: Vec3,
    radius: f32,
    amount_per_sec: f32,
    life_time: Option<f32>,
}

impl RadiusDamage {
    pub fn new(pos: Vec3, radius: f32, amount_per_sec: f32, life_time: Option<f32>) -> Self {
        Self {
            pos,
            radius,
            amount_per_sec,
            life_time,
        }
    }
}

fn damage_system(
    time: Res<Time>,
    q_damage: Query<&DamageList>,
    q_enemy: Query<(Entity, &Transform), With<Enemy>>,
    mut life_bar_ev: EventWriter<ProgressBarCountUpEvent>,
) {
    for dmg_list in q_damage.iter() {
        if let Some(dmg) = dmg_list.0.first() {
            for enemy_id in enemies_in_radius(dmg.pos, dmg.radius, &q_enemy) {
                life_bar_ev.send(ProgressBarCountUpEvent(
                    enemy_id,
                    dmg.amount_per_sec * time.delta_seconds(),
                ));
            }
        }
    }
}

fn delete_damage_system(mut q_damage: Query<&mut DamageList>) {
    for mut dmg_list in q_damage.iter_mut() {
        let mut to_delete = vec![];
        for (i, dmg) in dmg_list.0.iter().enumerate() {
            if let Some(life_time) = dmg.life_time {
                if life_time <= 0. {
                    to_delete.push(i);
                }
            }
        }
        for i in to_delete {
            dmg_list.0.remove(i);
        }
    }
}

fn count_down_life_time_system(time: Res<Time>, mut q_damage: Query<&mut DamageList>) {
    for mut dmg_list in q_damage.iter_mut() {
        for dmg in dmg_list.0.iter_mut() {
            if let Some(life_time) = dmg.life_time.as_mut() {
                *life_time -= time.delta_seconds();
            }
        }
    }
}

fn hit_marker_system(
    mut cmds: Commands,
    mut q_hitmarker: Query<(&mut Transform, &RelEntity), With<HitMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    q_damage: Query<(Entity, &DamageList)>,
    q_pbw: QueryWorld,
) {
    for (dmg_list_id, dmg_list) in q_damage.iter() {
        for dmg in dmg_list.0.iter() {
            if let Some((mut hm_trans, _)) = q_hitmarker
                .iter_mut()
                .find(|(_, rel_id)| rel_id.0 == dmg_list_id)
            {
                hm_trans.translation = dmg.pos;
            } else {
                cmds.entity(q_pbw.single()).with_children(|parent| {
                    spawn_hit_marker(
                        parent,
                        &mut meshes,
                        &mut mats,
                        dmg.pos,
                        dmg_list_id,
                        dmg.radius,
                    );
                });
            }
        }
    }
}

fn delete_hit_marker_system(
    mut cmds: Commands,
    q_hitmarker: Query<(Entity, &RelEntity), With<HitMarker>>,
    q_damage: Query<&DamageList>,
) {
    for (hm_id, rel_id) in q_hitmarker.iter() {
        if let Ok(dmg_list) = q_damage.get(rel_id.0) {
            if !dmg_list.0.is_empty() {
                continue;
            }
        }
        cmds.entity(hm_id).despawn();
    }
}

#[derive(Component)]
struct HitMarker;

fn spawn_hit_marker(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    mats: &mut Assets<StandardMaterial>,
    pos: Vec3,
    rel_id: Entity,
    radius: f32,
) {
    parent.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius,
                ..default()
            })),
            material: mats.add(StandardMaterial {
                base_color: Color::YELLOW,
                ..default()
            }),
            transform: Transform::from_translation(pos),
            ..default()
        },
        RelEntity(rel_id),
    ));
}

fn enemies_in_radius(
    pos: Vec3,
    radius: f32,
    q_enemy: &Query<(Entity, &Transform), With<Enemy>>,
) -> Vec<Entity> {
    q_enemy
        .iter()
        .filter(|(_, enemy_pos)| distance_squared(enemy_pos.translation, pos) <= radius.powi(2))
        .map(|(entity, _)| entity)
        .collect()
}

fn enemy_pos_in_radius(
    pos: Vec3,
    radius: f32,
    q_enemy: &Query<&Transform, With<Enemy>>,
) -> Vec<Vec3> {
    q_enemy
        .iter()
        .filter(|enemy_pos| distance_squared(enemy_pos.translation, pos) <= radius.powi(2))
        .map(|pos| pos.translation)
        .collect()
}
fn distance_squared(pos1: Vec3, pos2: Vec3) -> f32 {
    pos1.distance_squared(pos2)
}
