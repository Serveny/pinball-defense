//use crate::game::pinball_menu::PinballMenuEvent;
use crate::game::pinball_menu::PinballMenuEvent;
use crate::game::tower::TowerReady;
use crate::prelude::*;
use bevy_tweening::AnimCompletedEvent;

#[derive(Component)]
pub enum AfterTween {
    DeleteEntity,
    ActivatePinballMenu,
    DespawnPinballMenu,
    ActivateTower,
}

pub(super) fn on_tween_completed_system(
    mut cmds: Commands,
    mut evr: MessageReader<AnimCompletedEvent>,
    mut pm_status_ev: MessageWriter<PinballMenuEvent>,
    q_after_tween: Query<&AfterTween>,
) {
    for ev in evr.read() {
        if let bevy_tweening::AnimTargetKind::Component { entity } = ev.target
            && let Ok(after_tween) = q_after_tween.get(entity) {
                match after_tween {
                    AfterTween::DeleteEntity => {
                        if let Ok(mut ec) = cmds.get_entity(ev.anim_entity) {
                            ec.try_despawn();
                        }
                        // The tween target carries the `AfterTween` marker. When the
                        // anim entity *is* the tween target it has just been despawned
                        // above, so only clean up the marker when they differ.
                        if ev.anim_entity != entity
                            && let Ok(mut ec) = cmds.get_entity(entity) {
                                ec.remove::<AfterTween>();
                            }
                        continue;
                    }
                    AfterTween::ActivatePinballMenu => {
                        pm_status_ev.write(PinballMenuEvent::SetReady);
                    }
                    AfterTween::DespawnPinballMenu => {
                        pm_status_ev.write(PinballMenuEvent::Disable);
                    }
                    AfterTween::ActivateTower => {
                        if let Ok(mut ec) = cmds.get_entity(entity) {
                            ec.insert(TowerReady);
                        }
                    }
                }
                if let Ok(mut ec) = cmds.get_entity(entity) {
                    ec.remove::<AfterTween>();
                }
            }
    }
}
