#[cfg(not(debug_assertions))]
pub const CONFIG: PinballDefenseConfig = PinballDefenseConfig {
    tower_hit_progress: 1. / 15.,
};

#[cfg(debug_assertions)]
pub const CONFIG: PinballDefenseConfig = PinballDefenseConfig {
    tower_hit_progress: 1.,
};

pub struct PinballDefenseConfig {
    pub tower_hit_progress: f32,
}
