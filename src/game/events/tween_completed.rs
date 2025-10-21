//use crate::game::pinball_menu::PinballMenuEvent;
use crate::game::pinball_menu::PinballMenuEvent;
use crate::prelude::*;
use bevy_tweening::AnimCompletedEvent;

#[derive(Component)]
pub enum AfterTween {
    DeleteEntity,
    ActivatePinballMenu,
    DespawnPinballMenu,
}

pub(super) fn on_tween_completed_system(
    mut cmds: Commands,
    mut evr: MessageReader<AnimCompletedEvent>,
    mut pm_status_ev: MessageWriter<PinballMenuEvent>,
    q_after_tween: Query<&AfterTween>,
) {
    for ev in evr.read() {
        if let bevy_tweening::AnimTargetKind::Component { entity } = ev.target {
            if let Ok(after_tween) = q_after_tween.get(entity) {
                match after_tween {
                    AfterTween::DeleteEntity => cmds.entity(ev.anim_entity).despawn(),
                    AfterTween::ActivatePinballMenu => {
                        pm_status_ev.write(PinballMenuEvent::SetReady);
                    }
                    AfterTween::DespawnPinballMenu => {
                        pm_status_ev.write(PinballMenuEvent::Disable);
                    }
                }
                cmds.entity(entity).remove::<AfterTween>();
            }
        }
    }
}
