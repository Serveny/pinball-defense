use crate::prelude::*;

#[derive(PhysicsLayer, Clone, Copy, Debug, Default)]
pub enum GameLayer {
    Ball,
    Enemy,
    Tower,
    #[default]
    Map,
}
