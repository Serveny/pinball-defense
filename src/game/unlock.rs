use super::audio::SoundEvent;
use super::ball::CollisionWithBallEvent;
use super::camera::PinballCamera;
use super::events::collision::GameLayer;
use super::level::LevelHub;
use super::tower::TowerType;
use super::world::QueryWorld;
use super::{EventState, GameState};
use crate::AppState;
use crate::prelude::*;
use bevy::text::{FontSize, FontSource};

pub struct UnlockPlugin;

impl Plugin for UnlockPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TowerUnlocks>()
            .add_message::<TowerUnlockedEvent>()
            .add_systems(OnEnter(GameState::Init), |mut cmds: Commands| {
                cmds.insert_resource(TowerUnlocks::default());
            })
            .add_systems(
                Update,
                (spawn_fields_system, on_field_hit_system).run_if(in_state(EventState::Active)),
            )
            .add_systems(
                Update,
                update_labels_system.run_if(in_state(GameState::Ingame)),
            )
            .add_systems(OnExit(AppState::Game), clean_up_labels);
    }
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct TowerUnlocks {
    tesla: bool,
    microwave: bool,
}

#[derive(Message)]
pub struct TowerUnlockedEvent(pub TowerType);

impl TowerUnlocks {
    pub fn available(&self) -> impl Iterator<Item = TowerType> {
        [
            Some(TowerType::Gun),
            self.tesla.then_some(TowerType::Tesla),
            self.microwave.then_some(TowerType::Microwave),
        ]
        .into_iter()
        .flatten()
    }

    fn is_unlocked(&self, kind: TowerType) -> bool {
        match kind {
            TowerType::Gun => true,
            TowerType::Tesla => self.tesla,
            TowerType::Microwave => self.microwave,
        }
    }

    fn unlock(&mut self, kind: TowerType) {
        match kind {
            TowerType::Gun => {}
            TowerType::Tesla => self.tesla = true,
            TowerType::Microwave => self.microwave = true,
        }
    }
}

#[derive(Component)]
struct UnlockField(TowerType);

#[derive(Component)]
struct UnlockFieldLabel(Entity);

const FIELDS: [(TowerType, u32, Vec3, Color); 2] = [
    (
        TowerType::Tesla,
        2,
        Vec3::new(-1.15, 0.3, -0.048),
        Color::srgb_u8(35, 190, 245),
    ),
    (
        TowerType::Microwave,
        3,
        Vec3::new(-0.75, 0.54, -0.048),
        Color::srgb_u8(245, 110, 90),
    ),
];

fn spawn_fields_system(
    mut cmds: Commands,
    mut mats: ResMut<Assets<StandardMaterial>>,
    assets: Res<PinballDefenseGltfAssets>,
    level: Res<LevelHub>,
    unlocks: Res<TowerUnlocks>,
    q_fields: Query<&UnlockField>,
    q_world: QueryWorld,
) {
    if !level.is_changed() && !unlocks.is_changed() && !q_fields.is_empty() {
        return;
    }
    let Ok(world) = q_world.single() else { return };
    for (kind, required_level, pos, color) in FIELDS {
        if level.level() < required_level
            || unlocks.is_unlocked(kind)
            || q_fields.iter().any(|field| field.0 == kind)
        {
            continue;
        }
        cmds.entity(world).with_children(|p| {
            p.spawn((
                Name::new("Tower Unlock Field"),
                UnlockField(kind),
                Transform::from_translation(pos),
                Mesh3d(assets.extra_field.clone()),
                MeshMaterial3d(mats.add(StandardMaterial {
                    base_color: color,
                    emissive: color.to_linear() * 4.,
                    ..default()
                })),
                Sensor,
                Collider::circle(0.06),
                CollisionEventsEnabled,
                CollisionLayers::new(GameLayer::Map, GameLayer::Ball),
            ));
        });
    }
}

fn on_field_hit_system(
    mut cmds: Commands,
    mut hits: MessageReader<CollisionWithBallEvent>,
    q_fields: Query<&UnlockField>,
    mut unlocks: ResMut<TowerUnlocks>,
    mut events: MessageWriter<TowerUnlockedEvent>,
    mut sounds: MessageWriter<SoundEvent>,
) {
    for CollisionWithBallEvent(_, id) in hits.read() {
        if let Ok(field) = q_fields.get(*id) {
            if !unlocks.is_unlocked(field.0) {
                unlocks.unlock(field.0);
                events.write(TowerUnlockedEvent(field.0));
                sounds.write(SoundEvent::PbMenuActive);
            }
            cmds.entity(*id).despawn();
        }
    }
}

fn update_labels_system(
    mut cmds: Commands,
    fields: Query<(Entity, &Transform, &UnlockField)>,
    mut labels: Query<(Entity, &UnlockFieldLabel, &mut Node)>,
    camera: Query<(&GlobalTransform, &Camera), With<PinballCamera>>,
    assets: Res<PinballDefenseAssets>,
) {
    let Ok((camera_transform, camera)) = camera.single() else {
        return;
    };
    for (label_id, label, mut node) in &mut labels {
        if let Ok((_, transform, _)) = fields.get(label.0) {
            if let Ok(pos) = camera.world_to_viewport(camera_transform, transform.translation) {
                node.left = Val::Px(pos.x);
                node.top = Val::Px(pos.y - 30.);
            }
        } else {
            cmds.entity(label_id).despawn();
        }
    }
    for (field_id, transform, field) in &fields {
        if labels.iter().any(|(_, label, _)| label.0 == field_id) {
            continue;
        }
        let (label, color) = match field.0 {
            TowerType::Tesla => ("TESLA", FIELDS[0].3),
            TowerType::Microwave => ("MICROWAVE", FIELDS[1].3),
            TowerType::Gun => continue,
        };
        let Ok(pos) = camera.world_to_viewport(camera_transform, transform.translation) else {
            continue;
        };
        cmds.spawn((
            Name::new("Tower Unlock Label"),
            UnlockFieldLabel(field_id),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(pos.x),
                top: Val::Px(pos.y - 30.),
                ..default()
            },
            Text(label.into()),
            TextFont {
                font: FontSource::Handle(assets.menu_font.clone()),
                font_size: FontSize::Px(18.),
                ..default()
            },
            TextColor(color),
        ));
    }
}

fn clean_up_labels(mut cmds: Commands, labels: Query<Entity, With<UnlockFieldLabel>>) {
    for label in &labels {
        cmds.entity(label).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_hit_unlocks_once_and_removes_field() {
        let mut app = App::new();
        app.add_message::<CollisionWithBallEvent>()
            .add_message::<TowerUnlockedEvent>()
            .add_message::<SoundEvent>()
            .insert_resource(TowerUnlocks::default())
            .add_systems(Update, on_field_hit_system);
        let field = app.world_mut().spawn(UnlockField(TowerType::Tesla)).id();
        let ball = app.world_mut().spawn_empty().id();
        app.world_mut()
            .write_message(CollisionWithBallEvent(ball, field));
        app.update();
        assert!(app.world().resource::<TowerUnlocks>().tesla);
        assert!(!app.world().resource::<TowerUnlocks>().microwave);
        assert!(app.world().get_entity(field).is_err());
    }
}
