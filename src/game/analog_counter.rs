use super::GameState;
use crate::prelude::*;
extern crate digits_iterator;
use digits_iterator::*;
use std::f32::consts::{PI, TAU};

pub struct AnalogCounterPlugin;

impl Plugin for AnalogCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnalogCounterSetToEvent>().add_systems(
            Update,
            (on_set_system, turn_digit_system).run_if(in_state(GameState::Ingame)),
        );
    }
}

#[derive(Component)]
struct AnalogCounter;

#[derive(Component)]
struct Digit {
    number: u8,
    position: u8,
    current_rot_rad10: f32,
    is_active: bool,
}

impl Digit {
    fn new(place: u8) -> Self {
        Self {
            number: 0,
            position: place,
            current_rot_rad10: 0.,
            is_active: false,
        }
    }

    fn set_number(&mut self, number: u8) {
        self.number = number;
        self.is_active = true;
    }
}

#[derive(Event)]
pub struct AnalogCounterSetToEvent(pub u32);

fn on_set_system(
    mut on_set_ev: EventReader<AnalogCounterSetToEvent>,
    mut q_digit: Query<&mut Digit>,
) {
    for ev in on_set_ev.iter() {
        for (i, number) in ev.0.digits().rev().enumerate() {
            q_digit
                .iter_mut()
                .find(|digit_comp| digit_comp.position == i as u8)
                .unwrap_or_else(|| panic!("No digit component for i ({i})"))
                .set_number(number);
        }
    }
}

pub fn spawn(parent: &mut ChildBuilder, assets: &PinballDefenseAssets, pos: Vec3) {
    parent
        .spawn((
            Name::new("Analog Counter Casing"),
            PbrBundle {
                mesh: assets.analog_counter_casing.clone(),
                material: assets.analog_counter_casing_material.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            },
            AnalogCounter,
        ))
        .with_children(|parent| {
            for i in 0..9 {
                parent.spawn((
                    Name::new("Analog Counter Digit"),
                    PbrBundle {
                        mesh: assets.analog_counter_cylinder.clone(),
                        material: assets.analog_counter_cylinder_material.clone(),
                        transform: Transform::from_xyz(0., 0., i as f32 * 0.0242 - 0.096),
                        ..default()
                    },
                    Digit::new(i),
                ));
            }
        });
}

const TURN_SPEED_RADIANS_PER_SECOND: f32 = PI;

fn turn_digit_system(mut q_digit: Query<(&mut Transform, &mut Digit)>, time: Res<Time>) {
    for (mut trans, mut digit) in q_digit.iter_mut() {
        if digit.is_active {
            let current_rot = digit.current_rot_rad10.floor();
            let target_rot = (TAU * digit.number as f32).floor();
            if target_rot != current_rot {
                let rotation_to_add = TURN_SPEED_RADIANS_PER_SECOND * time.delta_seconds();
                trans.rotate_z(rotation_to_add);
                digit.current_rot_rad10 += rotation_to_add * 10.;
                digit.current_rot_rad10 %= TAU * 10.;
            } else {
                let angle = TAU * digit.number as f32;
                *trans = trans.with_rotation(Quat::from_rotation_z(angle / 10.));
                digit.current_rot_rad10 = angle;
                digit.is_active = false;
            }
        }
    }
}
