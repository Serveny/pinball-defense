use super::SAVE_DIR;
use std::path::PathBuf;

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
