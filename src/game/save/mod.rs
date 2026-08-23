use super::GameState;
use crate::prelude::*;
use crate::utils::RelEntity;
use moonshine_save::prelude::*;
use std::path::PathBuf;

pub const SAVE_DIR: &str = "saves";

#[derive(Resource)]
pub struct PendingLoad(pub String);

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(save_on_default_event)
            .add_observer(load_on_default_event)
            .register_type::<RelEntity>()
            .add_systems(
                Update,
                apply_pending_load.run_if(in_state(GameState::Ingame)),
            );
    }
}

fn apply_pending_load(mut cmds: Commands, pending: Option<Res<PendingLoad>>) {
    let Some(pending) = pending else {
        return;
    };
    cmds.trigger_load(LoadWorld::default_from_file(&pending.0));
    cmds.remove_resource::<PendingLoad>();
}

pub fn list_saves() -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(SAVE_DIR) else {
        return Vec::new();
    };
    let mut saves: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "ron"))
        .collect();
    saves.sort();
    saves
}

pub fn next_save_path() -> String {
    let next = list_saves()
        .iter()
        .filter_map(|path| path.file_stem().and_then(|s| s.to_str()))
        .filter_map(|stem| stem.strip_prefix("slot_"))
        .filter_map(|n| n.parse::<u32>().ok())
        .max()
        .map_or(1, |n| n + 1);
    format!("{SAVE_DIR}/slot_{next}.ron")
}
