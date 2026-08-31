mod config;
pub mod screenshot;
mod slots;

pub use config::save_world;
pub use screenshot::{spawn_save_screenshot, thumbnail_path};
pub use slots::{list_saves, next_save_path};

use super::GameState;
use crate::prelude::*;
use crate::utils::RelEntity;
use moonshine_save::prelude::*;
use std::path::PathBuf;

pub const SAVE_DIR: &str = "saves";

#[derive(Resource)]
pub struct PendingLoad(pub PathBuf);

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(save_on_default_event)
            .add_observer(load_on_default_event)
            .register_type::<RelEntity>()
            // Must run in Init, before the first Ingame frame: level_up_system
            // would otherwise fire on the default LevelHub (threshold 0) and
            // spawn a phantom foundation before the save is applied.
            .add_systems(Update, apply_pending_load.run_if(in_state(GameState::Init)));
    }
}

fn apply_pending_load(mut cmds: Commands, pending: Option<Res<PendingLoad>>) {
    let Some(pending) = pending else {
        return;
    };
    cmds.trigger_load(LoadWorld::default_from_file(&pending.0));
    cmds.remove_resource::<PendingLoad>();
}

pub fn save_game(cmds: &mut Commands, path: impl Into<PathBuf>) {
    let _ = std::fs::create_dir_all(SAVE_DIR);
    cmds.trigger_save(save_world(path));
}

pub fn load_game(cmds: &mut Commands, path: impl Into<PathBuf>) {
    cmds.insert_resource(PendingLoad(path.into()));
}
