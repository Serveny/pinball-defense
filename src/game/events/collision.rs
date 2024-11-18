use crate::prelude::*;

#[derive(PhysicsLayer)]
pub enum GameLayer {
    Ball,
    Enemy,
    Tower,
    Map,
}
