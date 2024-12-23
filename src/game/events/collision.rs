use crate::prelude::*;

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    Ball,
    Enemy,
    Tower,
    #[default]
    Map,
}
