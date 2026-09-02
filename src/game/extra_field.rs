use super::EventState;
use super::GameState;
use super::IngameTime;
use super::audio::SoundEvent;
use super::ball::CollisionWithBallEvent;
use super::enemy::recover_speed_system;
use super::events::collision::GameLayer;
use super::extra_field_effects::{on_extra_field_fire_system, slow_reapply_system};
use super::level::{BallCollisionPoints, LevelHub, LevelUpEvent};
use super::light::{
    ContactLight, FlashLight, LightOnCollision, contact_light_bundle, disable_flash_light,
};
use super::progress::{
    ProgressBarCountUpEvent, ProgressBarFullEvent, ProgressBarResetEvent, self,
};
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use bevy::color::palettes::css::{BLUE, GOLD, ORANGE, RED};
use bevy::math::primitives::Cylinder;
use rand::RngExt;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use std::f32::consts::PI;

pub struct ExtraFieldPlugin;

impl Plugin for ExtraFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ExtraFieldFireEvent>()
            .init_resource::<ActiveEffects>()
            .add_systems(OnEnter(GameState::Init), init_resources)
            .add_systems(
                Update,
                (
                    on_level_up_field_system,
                    restore_fields_system.after(on_level_up_field_system),
                )
                    .run_if(in_state(EventState::Active)),
            )
            .add_systems(
                Update,
                (on_charge_system, on_fire_system).run_if(in_state(EventState::Active)),
            )
            .add_systems(
                Update,
                (on_extra_field_fire_system).run_if(in_state(EventState::Active)),
            )
            .add_systems(
                Update,
                (slow_reapply_system.after(recover_speed_system))
                    .run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (effect_flash_system).run_if(in_state(GameState::Ingame)),
            );
    }
}

fn init_resources(mut cmds: Commands) {
    cmds.insert_resource(ActiveEffects::default());
}

#[derive(Component, Reflect, Clone, Copy, PartialEq, Eq)]
pub enum ExtraFieldKind {
    ExtraBall,
    SlowDown,
    DoubleDamage,
    InstaKill,
}

pub const SLOW_DOWN_HITS: u32 = 2;
pub const DOUBLE_DAMAGE_HITS: u32 = 4;
pub const EXTRA_BALL_HITS: u32 = 6;
pub const INSTA_KILL_HITS: u32 = 8;

impl ExtraFieldKind {
    pub fn color(self) -> Color {
        match self {
            Self::ExtraBall => GOLD.into(),
            Self::SlowDown => BLUE.into(),
            Self::DoubleDamage => ORANGE.into(),
            Self::InstaKill => RED.into(),
        }
    }

    pub fn hits_needed(self) -> u32 {
        match self {
            Self::ExtraBall => EXTRA_BALL_HITS,
            Self::SlowDown => SLOW_DOWN_HITS,
            Self::DoubleDamage => DOUBLE_DAMAGE_HITS,
            Self::InstaKill => INSTA_KILL_HITS,
        }
    }
}

#[derive(Component)]
pub struct ExtraField {
    kind: ExtraFieldKind,
}

impl ExtraField {
    pub fn kind(&self) -> ExtraFieldKind {
        self.kind
    }
}

#[derive(Message)]
pub struct ExtraFieldFireEvent(pub ExtraFieldKind);

#[allow(clippy::struct_field_names)]
#[derive(Resource, Default)]
pub struct ActiveEffects {
    pub slow_until: f32,
    pub double_damage_until: f32,
    pub insta_kill_until: f32,
}

impl ActiveEffects {
    pub fn is_active(&self, now: f32, which: ExtraFieldKind) -> bool {
        match which {
            ExtraFieldKind::SlowDown => now < self.slow_until,
            ExtraFieldKind::DoubleDamage => now < self.double_damage_until,
            ExtraFieldKind::InstaKill => now < self.insta_kill_until,
            ExtraFieldKind::ExtraBall => false,
        }
    }
}

const FIELD_RADIUS: f32 = 0.05;

fn pick_random_inactive<R: RngExt>(
    rng: &mut R,
    kinds: &[Option<ExtraFieldKind>],
) -> Option<usize> {
    let inactive: Vec<usize> = kinds
        .iter()
        .enumerate()
        .filter_map(|(i, kind)| kind.is_none().then_some(i))
        .collect();
    (!inactive.is_empty())
        .then(|| rng.random_range(0..inactive.len()))
        .and_then(|i| inactive.get(i).copied())
}

fn charge_amount(hits_needed: u32) -> f32 {
    1. / f32::from(u8::try_from(hits_needed.max(1)).unwrap_or(u8::MAX))
}

const KINDS: [ExtraFieldKind; 4] = [
    ExtraFieldKind::ExtraBall,
    ExtraFieldKind::SlowDown,
    ExtraFieldKind::DoubleDamage,
    ExtraFieldKind::InstaKill,
];

pub fn spawn_fields(
    p: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    mats: &mut Assets<StandardMaterial>,
    g_sett: &GraphicsSettings,
    posis: [Vec3; 4],
) {
    for (kind, pos) in KINDS.iter().zip(posis) {
        p.spawn(field_bundle(meshes, mats, *kind, pos)).with_children(|p| {
            p.spawn(contact_light_bundle(g_sett, kind.color()));
        });
    }
}

fn field_bundle(
    meshes: &mut Assets<Mesh>,
    mats: &mut Assets<StandardMaterial>,
    kind: ExtraFieldKind,
    pos: Vec3,
) -> impl Bundle {
    (
        Name::new("Extra Field"),
        Mesh3d(meshes.add(Mesh::from(Cylinder {
            radius: FIELD_RADIUS,
            half_height: 0.004,
        }))),
        MeshMaterial3d(mats.add(StandardMaterial {
            base_color: kind.color(),
            perceptual_roughness: 0.4,
            metallic: 0.2,
            reflectance: 0.3,
            ..default()
        })),
        Transform::from_translation(pos).with_rotation(Quat::from_rotation_x(PI / 2.)),
        ExtraField { kind },
        Visibility::Hidden,
    )
}

fn activate_field(
    cmds: &mut Commands,
    field_id: Entity,
    kind: ExtraFieldKind,
    assets: &PinballDefenseGltfAssets,
    mats: &mut Assets<StandardMaterial>,
) {
    cmds.entity(field_id)
        .insert((
            Sensor,
            Collider::circle(FIELD_RADIUS),
            CollisionEventsEnabled,
            CollisionLayers::new(GameLayer::Map, GameLayer::Ball),
            BallCollisionPoints(50),
            LightOnCollision,
            Visibility::Inherited,
        ))
        .with_children(|p| {
            progress::spawn(
                p,
                assets,
                mats,
                field_id,
                Transform::default(),
                kind.color(),
                0.,
            );
        });
}

fn on_level_up_field_system(
    mut cmds: Commands,
    evr: MessageReader<LevelUpEvent>,
    q_field: Query<(Entity, &ExtraField, Has<Collider>)>,
    assets: Res<PinballDefenseGltfAssets>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    if evr.is_empty() {
        return;
    }
    let mut rng = SmallRng::from_rng(&mut rand::rng());
    let fields: Vec<(Entity, ExtraFieldKind, bool)> = q_field
        .iter()
        .map(|(id, field, active)| (id, field.kind(), active))
        .collect();
    let kinds: Vec<Option<ExtraFieldKind>> = fields
        .iter()
        .map(|(_, kind, active)| active.then_some(*kind))
        .collect();
    if let Some(i) = pick_random_inactive(&mut rng, &kinds)
        && let Some(&(field_id, kind, _)) = fields.get(i)
    {
        activate_field(&mut cmds, field_id, kind, &assets, &mut mats);
    }
}

fn restore_fields_system(
    mut cmds: Commands,
    level: Res<LevelHub>,
    evr: MessageReader<LevelUpEvent>,
    q_field: Query<(Entity, &ExtraField, Has<Collider>)>,
    assets: Res<PinballDefenseGltfAssets>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    if !level.is_changed() || !evr.is_empty() {
        return;
    }
    let target = (u32::from(level.level()) / 3).min(4) as usize;
    let active = q_field.iter().filter(|(_, _, active)| *active).count();
    let mut missing = target.saturating_sub(active);
    if missing == 0 {
        return;
    }
    let mut rng = SmallRng::from_rng(&mut rand::rng());
    let mut kinds: Vec<Option<ExtraFieldKind>> = q_field
        .iter()
        .map(|(_, field, is_active)| is_active.then_some(field.kind()))
        .collect();
    let entities: Vec<Entity> = q_field.iter().map(|(id, _, _)| id).collect();
    while missing > 0 {
        let Some(i) = pick_random_inactive(&mut rng, &kinds) else {
            break;
        };
        let Some(&field_id) = entities.get(i) else {
            break;
        };
        if let Ok((_, field, _)) = q_field.get(field_id) {
            if let Some(kind_slot) = kinds.get_mut(i) {
                *kind_slot = Some(field.kind());
            }
            activate_field(&mut cmds, field_id, field.kind(), &assets, &mut mats);
        }
        missing -= 1;
    }
}

fn on_charge_system(
    mut prog_bar_ev: MessageWriter<ProgressBarCountUpEvent>,
    mut sound_ev: MessageWriter<SoundEvent>,
    mut evr: MessageReader<CollisionWithBallEvent>,
    q_field: Query<&ExtraField>,
) {
    for CollisionWithBallEvent(id) in evr.read() {
        if let Ok(field) = q_field.get(*id) {
            prog_bar_ev.write(ProgressBarCountUpEvent::new(
                *id,
                charge_amount(field.kind().hits_needed()),
            ));
            sound_ev.write(SoundEvent::ExtraFieldHit);
        }
    }
}

fn on_fire_system(
    mut fire_ev: MessageWriter<ExtraFieldFireEvent>,
    mut reset_ev: MessageWriter<ProgressBarResetEvent>,
    mut evr: MessageReader<ProgressBarFullEvent>,
    q_field: Query<&ExtraField>,
) {
    for ProgressBarFullEvent(id) in evr.read() {
        if let Ok(field) = q_field.get(*id) {
            fire_ev.write(ExtraFieldFireEvent(field.kind()));
            reset_ev.write(ProgressBarResetEvent::new(*id));
        }
    }
}

fn effect_flash_system(
    mut cmds: Commands,
    q_field: Query<(Entity, &ExtraField)>,
    mut q_light_off: Query<(Entity, &ChildOf, &mut Visibility), (With<ContactLight>, Without<FlashLight>)>,
    mut q_light_on: Query<(Entity, &ChildOf, &mut Visibility), With<FlashLight>>,
    effects: Res<ActiveEffects>,
    ig_time: Res<IngameTime>,
    mut prev_active: Local<[bool; 3]>,
) {
    for (i, kind) in [
        ExtraFieldKind::SlowDown,
        ExtraFieldKind::DoubleDamage,
        ExtraFieldKind::InstaKill,
    ]
    .into_iter()
    .enumerate()
    {
        let now_active = effects.is_active(**ig_time, kind);
        let Some(prev) = prev_active.get_mut(i) else {
            continue;
        };
        if now_active && !*prev {
            for (field_id, field) in q_field.iter() {
                if field.kind() != kind {
                    continue;
                }
                if let Some((light_id, _, mut visi)) = q_light_off
                    .iter_mut()
                    .find(|(_, child_of, _)| child_of.parent() == field_id)
                {
                    cmds.entity(light_id).insert(FlashLight);
                    *visi = Visibility::Inherited;
                }
            }
        } else if *prev && !now_active {
            for (field_id, field) in q_field.iter() {
                if field.kind() != kind {
                    continue;
                }
                disable_flash_light(&mut cmds, &mut q_light_on, field_id);
            }
        }
        *prev = now_active;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn extra_field_pick_returns_none_when_all_active() {
        let mut rng = StdRng::seed_from_u64(42);
        let kinds = [
            Some(ExtraFieldKind::ExtraBall),
            Some(ExtraFieldKind::SlowDown),
            Some(ExtraFieldKind::DoubleDamage),
            Some(ExtraFieldKind::InstaKill),
        ];
        assert_eq!(pick_random_inactive(&mut rng, &kinds), None);
    }

    #[test]
    fn extra_field_pick_returns_some_when_inactive_exist() {
        let mut rng = StdRng::seed_from_u64(42);
        let kinds = [Some(ExtraFieldKind::ExtraBall), None, None, None];
        assert!(pick_random_inactive(&mut rng, &kinds).is_some());
    }

    #[test]
    fn extra_field_pick_never_returns_active_kind() {
        let mut rng = StdRng::seed_from_u64(7);
        let kinds = [
            Some(ExtraFieldKind::ExtraBall),
            None,
            Some(ExtraFieldKind::InstaKill),
            None,
        ];
        for _ in 0..100 {
            let picked = pick_random_inactive(&mut rng, &kinds).unwrap();
            assert!(kinds[picked].is_none());
        }
    }

    #[test]
    fn extra_field_charge_amount_is_inverse_of_hits() {
        assert_eq!(charge_amount(2), 0.5);
        assert_eq!(charge_amount(4), 0.25);
        assert_eq!(charge_amount(8), 0.125);
        assert!(charge_amount(0).is_finite());
    }
}