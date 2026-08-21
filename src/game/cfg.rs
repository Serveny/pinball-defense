#[cfg(not(debug_assertions))]
pub const CONFIG: PinballDefenseConfig = PinballDefenseConfig {
    tower_hit_progress: 1. / 15.,
    damage_upgrade_factor: 1.2,
    range_upgade_factor: 0.01,
    slow_down_upgrade_factor: 0.98,
};

#[cfg(debug_assertions)]
pub const CONFIG: PinballDefenseConfig = PinballDefenseConfig {
    tower_hit_progress: 1.,
    damage_upgrade_factor: 1.2,
    range_upgade_factor: 0.01,
    slow_down_upgrade_factor: 0.98,
};

pub struct PinballDefenseConfig {
    pub tower_hit_progress: f32,
    pub damage_upgrade_factor: f32,
    pub range_upgade_factor: f32,
    pub slow_down_upgrade_factor: f32,
}
