#[cfg(not(debug_assertions))]
pub const CONFIG: PinballDefenseConfig = PinballDefenseConfig {
    tower_hit_progress: 1. / 15.,
    tower_enemy_killed_progress: 1. / 50.,
};

#[cfg(debug_assertions)]
pub const CONFIG: PinballDefenseConfig = PinballDefenseConfig {
    tower_hit_progress: 1.,
    tower_enemy_killed_progress: 0.5,
};

pub struct PinballDefenseConfig {
    pub tower_hit_progress: f32,
    pub tower_enemy_killed_progress: f32,
}
