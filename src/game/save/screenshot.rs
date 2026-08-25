use crate::menu::{MenuLayout, MenuState, SavedInPauseMenu};
use crate::prelude::*;
use bevy::render::view::screenshot::{Screenshot, ScreenshotCaptured};
use std::path::PathBuf;

pub const THUMB_W: u32 = 320;
pub const THUMB_H: u32 = 180;

#[derive(Resource)]
pub struct PendingScreenshot {
    pub save_path: PathBuf,
}

pub fn thumbnail_path(save_path: &std::path::Path) -> PathBuf {
    save_path.with_extension("png")
}

pub fn spawn_save_screenshot(cmds: &mut Commands, save_path: PathBuf) {
    cmds.insert_resource(PendingScreenshot { save_path });
    cmds.spawn(Screenshot::primary_window()).observe(on_captured);
}

fn on_captured(
    trigger: On<ScreenshotCaptured>,
    pending: Res<PendingScreenshot>,
    mut cmds: Commands,
    mut q_layout: Query<&mut Visibility, With<MenuLayout>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    let thumb_path = thumbnail_path(&pending.save_path);
    let image = trigger.image.clone();
    match image.try_into_dynamic() {
        Ok(dyn_img) => {
            let thumb = dyn_img.thumbnail(THUMB_W, THUMB_H);
            if let Err(e) = thumb.save_with_format(&thumb_path, image::ImageFormat::Png) {
                error!("Failed to save thumbnail to {}: {e}", thumb_path.display());
            } else {
                info!("Saved thumbnail to {}", thumb_path.display());
            }
        }
        Err(e) => error!("Failed to convert screenshot to dynamic image: {e}"),
    }
    for mut vis in &mut q_layout {
        *vis = Visibility::Inherited;
    }
    next_menu_state.set(MenuState::PauseMenu);
    cmds.insert_resource(SavedInPauseMenu);
    cmds.remove_resource::<PendingScreenshot>();
}