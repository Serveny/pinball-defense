//use crate::game::pinball_menu::PinballMenuEvent;
use crate::prelude::*;
use bevy_tweening::AnimCompletedEvent;

#[derive(Component)]
pub struct DeleteAfterTween;

//pub const DESPAWN_ENTITY_EVENT_ID: u64 = 0;
//pub const ACTIVATE_PINBALL_MENU_EVENT_ID: u64 = 1;
//pub const DESPAWN_PINBALL_MENU_EVENT_ID: u64 = 2;

pub(super) fn on_tween_completed_system(
    mut evr: MessageReader<AnimCompletedEvent>,
    mut cmds: Commands,
    //mut pm_status_ev: MessageWriter<PinballMenuEvent>,
    q_delete: Query<Entity, With<DeleteAfterTween>>,
) {
    for ev in evr.read() {
        if let bevy_tweening::AnimTargetKind::Component { entity } = ev.target {
            if q_delete.get(entity).is_ok() {
                cmds.entity(ev.anim_entity).despawn();
            }
        }

        //match ev.anim_entity.to_bits() {
        //DESPAWN_ENTITY_EVENT_ID => cmds.entity(ev.anim_entity).despawn(),
        //ACTIVATE_PINBALL_MENU_EVENT_ID => {
        //pm_status_ev.write(PinballMenuEvent::SetReady);
        //}
        //DESPAWN_PINBALL_MENU_EVENT_ID => {
        //pm_status_ev.write(PinballMenuEvent::Disable);
        //}
        //_ => panic!("😭 Unkown tween user event: {}", ev.anim_entity),
        //}
    }
}
