use super::{audio::SoundEvent, EventState, GameState};
use crate::prelude::*;
extern crate digits_iterator;
use crate::utils::RelEntity;
use digits_iterator::*;
use std::f32::consts::{PI, TAU};

pub struct AnalogCounterPlugin;

impl Plugin for AnalogCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnalogCounterSetEvent>()
            .add_systems(
                Update,
                (turn_digit_system).run_if(in_state(GameState::Ingame)),
            )
            .add_systems(Update, (on_set_system).run_if(in_state(EventState::Active)));
    }
}

#[derive(Component)]
struct AnalogCounter;

#[derive(Component)]
struct Digit {
    number: u8,
    position: u8,
    current_rot: f32,
    is_active: bool,
}

const TURN_SPEED_RADIANS_PER_SECOND: f32 = PI;
const NUMBER_ROT: f32 = TAU / 10.;
const ROT_TOLERANCE: f32 = 0.05;
const ROT_TOLERANCE_MAX: f32 = TAU - ROT_TOLERANCE;

impl Digit {
    fn new(place: u8) -> Self {
        Self {
            number: 0,
            position: place,
            current_rot: 0.,
            is_active: false,
        }
    }

    fn set_number(&mut self, number: u8) {
        self.number = number;
        self.is_active = true;
    }

    fn set_rot_to(&mut self, number: u8) -> f32 {
        let target_rot = (number as f32 * NUMBER_ROT).rem_euclid(TAU);
        self.current_rot = target_rot;
        self.is_active = false;
        TAU - target_rot
    }

    fn rotate(&mut self, delta_secs: f32) -> f32 {
        let rotation_to_add = TURN_SPEED_RADIANS_PER_SECOND * delta_secs;
        self.current_rot = (self.current_rot + rotation_to_add).rem_euclid(TAU);
        -rotation_to_add
    }

    fn is_on_number(&self) -> Option<u8> {
        let num_rot = self.current_rot.rem_euclid(NUMBER_ROT);
        if !(ROT_TOLERANCE..=ROT_TOLERANCE_MAX).contains(&num_rot) {
            return Some((self.current_rot / NUMBER_ROT).floor() as u8);
        }
        None
    }
}

#[derive(Event)]
pub struct AnalogCounterSetEvent {
    rel_id: Entity,
    number: u32,
}

impl AnalogCounterSetEvent {
    pub fn new(rel_id: Entity, number: u32) -> Self {
        Self { rel_id, number }
    }
}

fn on_set_system(
    mut on_set_ev: EventReader<AnalogCounterSetEvent>,
    mut q_digit: Query<(&mut Digit, &Parent)>,
    q_counter: Query<(Entity, &RelEntity), With<AnalogCounter>>,
) {
    for ev in on_set_ev.iter() {
        if let Some((counter_id, _)) = q_counter.iter().find(|(_, rel_id)| rel_id.0 == ev.rel_id) {
            for (i, number) in ev.number.digits().rev().enumerate() {
                if let Some((mut digit, _)) = q_digit
                    .iter_mut()
                    .find(|(digit_comp, p)| p.get() == counter_id && digit_comp.position == i as u8)
                {
                    digit.set_number(number);
                } else {
                    warn!("😥 No digit component for i ({i}) with given parent!");
                }
            }
        }
    }
}

pub fn spawn_10_digit(
    parent: &mut ChildBuilder,
    assets: &PinballDefenseGltfAssets,
    pos: Vec3,
    rel_id: Option<Entity>,
) -> Entity {
    let mut counter = parent.spawn((
        Name::new("Analog Counter Casing 10 Digit"),
        PbrBundle {
            mesh: assets.analog_counter_10_digit_casing.clone(),
            material: assets.analog_counter_casing_10_digit_material.clone(),
            transform: Transform::from_translation(pos),
            ..default()
        },
        AnalogCounter,
    ));
    counter.with_children(|parent| {
        for i in 0..9 {
            parent.spawn((
                Name::new("Analog Counter Digit"),
                PbrBundle {
                    mesh: assets.analog_counter_cylinder.clone(),
                    material: assets.analog_counter_cylinder_material.clone(),
                    transform: Transform::from_xyz(0., i as f32 * -0.0242 + 0.096, 0.),
                    ..default()
                },
                Digit::new(i),
            ));
        }
        parent.spawn((
            Name::new("Level Sign"),
            PbrBundle {
                mesh: assets.point_sign.clone(),
                material: assets.points_sign_material.clone(),
                transform: Transform::from_xyz(-0.055, 0., 0.047),
                ..default()
            },
        ));
    });

    let counter_id = counter.id();
    counter.insert(RelEntity(match rel_id {
        Some(rel_id) => rel_id,
        None => counter_id,
    }));
    counter_id
}

pub fn spawn_2_digit(
    parent: &mut ChildBuilder,
    assets: &PinballDefenseGltfAssets,
    transform: Transform,
    rel_id: Option<Entity>,
) -> Entity {
    let mut counter = parent.spawn((
        Name::new("Analog Counter Casing 2 Digit"),
        PbrBundle {
            mesh: assets.analog_counter_casing_2_digit.clone(),
            material: assets.analog_counter_casing_2_digit_material.clone(),
            transform,
            ..default()
        },
        AnalogCounter,
    ));
    counter.with_children(|parent| {
        for i in 0..2 {
            parent.spawn((
                Name::new("Analog Counter Digit"),
                PbrBundle {
                    mesh: assets.analog_counter_cylinder.clone(),
                    material: assets.analog_counter_cylinder_material.clone(),
                    transform: Transform::from_xyz(0., i as f32 * -0.022 + 0.012, 0.),
                    ..default()
                },
                Digit::new(i),
            ));
        }
        parent.spawn((
            Name::new("Level Sign"),
            PbrBundle {
                mesh: assets.level_sign.clone(),
                material: assets.level_sign_material.clone(),
                transform: Transform::from_xyz(-0.055, 0., 0.047),
                ..default()
            },
        ));
    });

    let counter_id = counter.id();
    counter.insert(RelEntity(match rel_id {
        Some(rel_id) => rel_id,
        None => counter_id,
    }));
    counter_id
}

fn turn_digit_system(
    mut q_digit: Query<(&mut Transform, &mut Digit)>,
    mut sound_ev: EventWriter<SoundEvent>,
    time: Res<Time>,
) {
    for (mut trans, mut digit) in q_digit.iter_mut() {
        if digit.is_active {
            if let Some(number) = digit.is_on_number() {
                if number == digit.number {
                    let target_rot = digit.set_rot_to(number);
                    *trans = trans.with_rotation(Quat::from_rotation_y(target_rot));
                    return;
                }
                sound_ev.send(SoundEvent::CounterTick);
            }

            trans.rotate_y(digit.rotate(time.delta_seconds()));
        }
    }
}
