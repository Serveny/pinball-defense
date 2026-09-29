pub const CONFIG: PinballDefenseConfig = PinballDefenseConfig {
    tower_hit_progress: 1. / 15.,
    damage_upgrade_factor: 1.2,
    tower_kick_velocity: 2.,
    ball_damage_fraction: 0.3,
    ball_damage_fraction_per_level: 0.05,
    ball_damage_fraction_max: 0.5,
};

pub struct PinballDefenseConfig {
    pub tower_hit_progress: f32,
    pub damage_upgrade_factor: f32,
    pub tower_kick_velocity: f32,
    pub ball_damage_fraction: f32,
    pub ball_damage_fraction_per_level: f32,
    pub ball_damage_fraction_max: f32,
}
