use crate::game::pinball_menu::PinballMenuEvent;
use crate::prelude::*;
use bevy_tweening::TweenCompleted;

pub const DESPAWN_ENTITY_EVENT_ID: u64 = 0;
pub const ACTIVATE_PINBALL_MENU_EVENT_ID: u64 = 1;
pub const DESPAWN_PINBALL_MENU_EVENT_ID: u64 = 2;

pub(super) fn on_tween_completed_system(
    mut evr: EventReader<TweenCompleted>,
    mut cmds: Commands,
    mut pm_status_ev: EventWriter<PinballMenuEvent>,
) {
    for ev in evr.read() {
        match ev.user_data {
            DESPAWN_ENTITY_EVENT_ID => cmds.entity(ev.entity).despawn(),
            ACTIVATE_PINBALL_MENU_EVENT_ID => {
                pm_status_ev.write(PinballMenuEvent::SetReady);
            }
            DESPAWN_PINBALL_MENU_EVENT_ID => {
                pm_status_ev.write(PinballMenuEvent::Disable);
            }
            _ => panic!("😭 Unkown tween user event: {}", ev.user_data),
        }
    }
}
