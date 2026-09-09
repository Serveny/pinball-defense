mod charge;
pub(crate) mod effects;
mod field;

pub use effects::ActiveEffects;
pub use field::spawn_fields;

use super::{EventState, GameState};
use super::enemy::recover_speed_system;
use crate::prelude::*;
use bevy::color::palettes::css::{BLUE, GOLD, ORANGE, RED};

pub struct ExtraFieldPlugin;

impl Plugin for ExtraFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ExtraFieldFireEvent>()
            .add_message::<ExtraFieldUnlockEvent>()
            .init_resource::<ActiveEffects>()
            .add_systems(OnEnter(GameState::Init), init_resources)
            .add_systems(
                Update,
                field::update_fields_system.run_if(in_state(EventState::Active)),
            )
            .add_systems(
                Update,
                (charge::on_charge_system, charge::on_fire_system, field::button_press_system)
                    .run_if(in_state(EventState::Active)),
            )
            .add_systems(
                Update,
                effects::on_extra_field_fire_system.run_if(in_state(EventState::Active)),
            )
            .add_systems(
                Update,
                (
                    effects::slow_reapply_system.after(recover_speed_system),
                    charge::rewind_on_effect_end_system,
                )
                    .run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                field::effect_flash_system.run_if(in_state(GameState::Ingame)),
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

pub const SLOW_DOWN_HITS: u32 = 4;
pub const DOUBLE_DAMAGE_HITS: u32 = 8;
pub const EXTRA_BALL_HITS: u32 = 12;
pub const INSTA_KILL_HITS: u32 = 16;

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

    pub fn label(self) -> &'static str {
        match self {
            Self::ExtraBall => "EXTRA BALL FIELD",
            Self::SlowDown => "SLOW-DOWN FIELD",
            Self::DoubleDamage => "DOUBLE-DAMAGE FIELD",
            Self::InstaKill => "INSTA-KILL FIELD",
        }
    }

    pub fn icon(self, tex: &PinballDefenseAssets) -> &Handle<Image> {
        match self {
            Self::ExtraBall => &tex.extra_ball_icon,
            Self::SlowDown => &tex.slow_down_icon,
            Self::DoubleDamage => &tex.double_damage_icon,
            Self::InstaKill => &tex.insta_kill_icon,
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

#[derive(Message)]
pub struct ExtraFieldUnlockEvent(pub ExtraFieldKind);

const KINDS: [ExtraFieldKind; 4] = [
    ExtraFieldKind::ExtraBall,
    ExtraFieldKind::SlowDown,
    ExtraFieldKind::DoubleDamage,
    ExtraFieldKind::InstaKill,
];