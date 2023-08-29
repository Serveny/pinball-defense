#[cfg(not(debug_assertions))]
pub const CONFIG: PinballDefenseConfig = PinballDefenseConfig {
    foundation_hit_progress: 1. / 30.,
    tower_hit_progress: 1. / 15.,
};

#[cfg(debug_assertions)]
pub const CONFIG: PinballDefenseConfig = PinballDefenseConfig {
    foundation_hit_progress: 1.,
    tower_hit_progress: 1.,
};

pub struct PinballDefenseConfig {
    pub foundation_hit_progress: f32,
    pub tower_hit_progress: f32,
}
